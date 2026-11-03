use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::{Notify, OwnedSemaphorePermit, Semaphore};
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};
use tokio_util::sync::CancellationToken;

use crate::Database;
use crate::{info, warn, debug, error};

pub const MAX_ATTEMPTS: u32 = 5;
pub const ATTEMPT_TIMEOUT: Duration = Duration::from_secs(10);
pub const BASE_DELAY: Duration = Duration::from_millis(500);
pub const MAX_DELAY: Duration = Duration::from_secs(30);
pub const MAX_DB_DELAY: Duration = Duration::from_secs(5);
pub const IDLE_RECHECK: Duration = Duration::from_secs(5);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Attempt {
    Delivered,
    Retry(String),
    GiveUp(String),
}

impl Attempt {
    #[must_use]
    pub fn classify_status(s: reqwest::StatusCode) -> Self {
        if s.is_success() {
            Self::Delivered
        } else if s.is_server_error() || s == reqwest::StatusCode::TOO_MANY_REQUESTS {
            Self::Retry(s.to_string())
        } else {
            Self::GiveUp(s.to_string())
        }
    }
}

#[must_use]
pub fn calculate_retry_delay(attempt: u32) -> Duration {
    let mut delay = BASE_DELAY;
    for _ in 1..attempt {
        delay = (delay * 2).min(MAX_DELAY);
    }
    delay
}

/// Per-service wake-up hints from the listener. A ring only means "look again": rings that
/// arrive while a claimer is busy collapse into one, so there is nothing to overflow.
#[derive(Clone, Default)]
pub struct Doorbells {
    bells: Arc<Mutex<HashMap<u32, Arc<Notify>>>>,
    added: Arc<Notify>,
}

impl Doorbells {
    pub fn ring(&self, service_id: u32) {
        self.bell(service_id).notify_one();
    }

    #[must_use]
    pub fn bell(&self, service_id: u32) -> Arc<Notify> {
        self.bells
            .lock()
            .unwrap()
            .entry(service_id)
            .or_insert_with(|| {
                self.added.notify_one();
                Arc::new(Notify::new())
            })
            .clone()
    }

    #[must_use]
    pub fn services(&self) -> Vec<u32> {
        self.bells.lock().unwrap().keys().copied().collect()
    }
}

#[derive(Clone)]
pub struct Dispatcher {
    database: Database,
    http: reqwest::Client,
    pub doorbells: Doorbells,
}

impl Dispatcher {
    #[must_use]
    pub fn init(database: Database) -> Self {
        Self {
            database,
            http: reqwest::Client::new(),
            doorbells: Doorbells::default(),
        }
    }

    pub async fn try_send(&self, url: &str, payload: &str) -> Attempt {
        let req = self
            .http
            .post(url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(payload.to_owned())
            .send();

        match timeout(ATTEMPT_TIMEOUT, req).await {
            Err(_) => Attempt::Retry("timed out".into()),
            Ok(Err(e)) if e.is_builder() => Attempt::GiveUp(e.to_string()),
            Ok(Err(e)) => Attempt::Retry(e.to_string()),
            Ok(Ok(resp)) => {
                Attempt::classify_status(resp.status())
            }
        }
    }

    // Holds its slot from claim until the final status is stored, so a service never has
    // more than `capacity` rows in memory and never more than `capacity` requests on the wire
    async fn deliver(
        self,
        seq: u64,
        url: Arc<str>,
        payload: String,
        _slot: OwnedSemaphorePermit,
        in_flight: Arc<Mutex<HashSet<u64>>>,
    ) {
        info!("Dispatched request to {url}");

        let mut delay = BASE_DELAY;
        let mut status = "FAILED";

        for attempt in 1..=MAX_ATTEMPTS {
            match self.try_send(&url, &payload).await {
                Attempt::Delivered => {
                    info!("Delivered to {url} on attempt {attempt}");
                    status = "SUCCESS";
                    break;
                }
                Attempt::GiveUp(why) => {
                    warn!("Delivery to {url} rejected: {why}");
                    break;
                }
                Attempt::Retry(why) if attempt < MAX_ATTEMPTS => {
                    warn!(
                        "Attempt {attempt}/{MAX_ATTEMPTS} to {url} failed: {why}, retrying in {delay:?}"
                    );
                    sleep(delay).await;
                    delay = (delay * 2).min(MAX_DELAY);
                }
                Attempt::Retry(why) => {
                    warn!(
                        "Giving up on {url} after {MAX_ATTEMPTS} attempts: {why}"
                    );
                }
            }
        }

        persist("Storing delivery status", || {
            self.database.set_query_status(seq, status)
        })
        .await;

        // Only after the status is stored, so recovery can never pick up a finished row
        in_flight.lock().unwrap().remove(&seq);
    }

    // Sole owner of one service's rows. Claims until the table has nothing left for it,
    // then sleeps until the next ring; a ring or a database error can never strand a row.
    async fn claimer(self, service_id: u32, bell: Arc<Notify>, stop: CancellationToken) {
        let (url, capacity) =
            persist("Loading service", || self.database.get_service(service_id)).await;
        info!("Loaded capacity {capacity} for service {service_id}");

        let url: Arc<str> = url.into();
        let slots = Arc::new(Semaphore::new(capacity));
        let in_flight = Arc::new(Mutex::new(HashSet::new()));
        let mut deliveries = JoinSet::new();
        let mut delay = BASE_DELAY;

        // After a failed claim the UPDATE may have committed without us seeing the reply,
        // so rows can be PROCESSING with nobody delivering them; adopt those first
        let mut recover = false;

        loop {
            let mut slot = vec![slots.clone().acquire_owned().await.expect("never closed")];
            while let Ok(s) = slots.clone().try_acquire_owned() {
                slot.push(s);
            }

            let claimed = if recover {
                self.database.processing(service_id).await.map(|rows| {
                    let owned = in_flight.lock().unwrap();
                    rows.into_iter()
                        .filter(|(seq, _)| !owned.contains(seq))
                        .take(slot.len())
                        .collect::<Vec<_>>()
                })
            } else {
                self.database.claim(service_id, slot.len()).await
            };

            match claimed {
                Err(e) => {
                    warn!(
                        "Claim for service {service_id} failed: {e}, retrying in {delay:?}"
                    );
                    recover = true;
                    drop(slot);
                    sleep(delay).await;
                    delay = (delay * 2).min(MAX_DB_DELAY);
                }
                Ok(jobs) if jobs.is_empty() && recover => recover = false,
                Ok(jobs) if jobs.is_empty() => {
                    drop(slot);
                    if stop.is_cancelled() {
                        break;
                    }
                    debug!("Service {service_id} has no pending requests");
                    tokio::select! {
                        () = bell.notified() => {}
                        () = sleep(IDLE_RECHECK) => {}
                        () = stop.cancelled() => {}
                    }
                }
                Ok(jobs) => {
                    delay = BASE_DELAY;
                    for ((seq, payload), slot) in jobs.into_iter().zip(slot) {
                        in_flight.lock().unwrap().insert(seq);
                        deliveries.spawn(self.clone().deliver(
                            seq,
                            url.clone(),
                            payload,
                            slot,
                            in_flight.clone(),
                        ));
                    }
                }
            }

            while let Some(res) = deliveries.try_join_next() {
                report(res);
            }
        }

        while let Some(res) = deliveries.join_next().await {
            report(res);
        }
    }

    fn spawn_claimers(
        &self,
        claimers: &mut JoinSet<()>,
        started: &mut HashSet<u32>,
        stop: &CancellationToken,
    ) {
        for id in self.doorbells.services() {
            if started.insert(id) {
                let bell = self.doorbells.bell(id);
                claimers.spawn(self.clone().claimer(id, bell, stop.clone()));
            }
        }
    }

    pub async fn run(&self, stop: CancellationToken) -> Result<(), mysql_async::Error> {
        info!("Dispatcher started");

        // Backlog left by the previous run is claimed exactly like new work
        for id in persist("Finding pending services", || {
            self.database.pending_services()
        })
        .await
        {
            self.doorbells.ring(id);
        }

        let mut claimers = JoinSet::new();
        let mut started = HashSet::new();

        loop {
            self.spawn_claimers(&mut claimers, &mut started, &stop);

            while let Some(res) = claimers.try_join_next() {
                report(res);
            }

            tokio::select! {
                () = self.doorbells.added.notified() => {}
                () = stop.cancelled() => break,
            }
        }

        self.spawn_claimers(&mut claimers, &mut started, &stop);

        info!("Dispatcher stopping, draining {} services", claimers.len());
        while let Some(res) = claimers.join_next().await {
            report(res);
        }
        info!("Dispatcher stopped");
        Ok(())
    }
}

async fn persist<T, F, Fut>(what: &str, mut op: F) -> T
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, mysql_async::Error>>,
{
    let mut delay = BASE_DELAY;
    loop {
        match op().await {
            Ok(v) => return v,
            Err(e) => {
                error!("{what} failed: {e}, retrying in {delay:?}");
                sleep(delay).await;
                delay = (delay * 2).min(MAX_DB_DELAY);
            }
        }
    }
}

fn report(res: Result<(), tokio::task::JoinError>) {
    if let Err(e) = res {
        error!("Dispatch task failed: {e}");
    }
}
