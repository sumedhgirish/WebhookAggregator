use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use wagg::dispatcher::Doorbells;

/// [WACDP-10] Concurrency Stress Suite
/// Verifies high-concurrency doorbells notification, race-free wakeups,
/// and semaphore permit bounding under parallel task load.

#[tokio::test]
async fn test_doorbells_high_concurrency_stress() {
    let doorbells = Doorbells::default();
    let num_tasks = 100;
    let mut handles = Vec::new();

    // Spawn 100 concurrent tasks ringing doorbells for service 1 and 2
    for i in 0..num_tasks {
        let db = doorbells.clone();
        handles.push(tokio::spawn(async move {
            let service_id = (i % 2) + 1;
            db.ring(service_id);
        }));
    }

    for h in handles {
        h.await.expect("Task panicked during concurrent doorbell ringing");
    }

    let services = doorbells.services();
    assert!(services.contains(&1), "Doorbells must register service 1");
    assert!(services.contains(&2), "Doorbells must register service 2");
}

#[tokio::test]
async fn test_doorbells_multi_service_isolation() {
    let doorbells = Doorbells::default();
    let bell_1 = doorbells.bell(10);
    let bell_2 = doorbells.bell(20);

    // Ring service 10
    doorbells.ring(10);

    // Assert service 10 received notification within 100ms
    let notified_1 = tokio::time::timeout(Duration::from_millis(100), bell_1.notified()).await;
    assert!(notified_1.is_ok(), "Service 10 bell should receive notification");

    // Assert service 20 received NO notification (timeout occurs)
    let notified_2 = tokio::time::timeout(Duration::from_millis(50), bell_2.notified()).await;
    assert!(notified_2.is_err(), "Service 20 bell should not receive notification when service 10 is rung");
}

#[tokio::test]
async fn test_doorbells_notification_collapse() {
    let doorbells = Doorbells::default();
    let bell = doorbells.bell(42);

    // Ring 50 times in rapid succession
    for _ in 0..50 {
        doorbells.ring(42);
    }

    // First wait consumes the collapsed notification
    let first = tokio::time::timeout(Duration::from_millis(50), bell.notified()).await;
    assert!(first.is_ok(), "Notification should be received");

    // Second wait immediately times out because rings collapsed into one without unbounded buffering
    let second = tokio::time::timeout(Duration::from_millis(50), bell.notified()).await;
    assert!(second.is_err(), "Extra rings should have collapsed to prevent queue overflow");
}

#[tokio::test]
async fn test_concurrent_semaphore_capacity_bounding() {
    let capacity = 5;
    let sem = Arc::new(Semaphore::new(capacity));
    let in_flight = Arc::new(AtomicUsize::new(0));
    let max_observed = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();

    // Spawn 25 concurrent jobs attempting to acquire slots
    for _ in 0..25 {
        let sem_clone = sem.clone();
        let in_flight_clone = in_flight.clone();
        let max_obs_clone = max_observed.clone();

        handles.push(tokio::spawn(async move {
            let _permit = sem_clone.acquire_owned().await.unwrap();
            let current = in_flight_clone.fetch_add(1, Ordering::SeqCst) + 1;

            // Track max concurrent in-flight executions
            max_obs_clone.fetch_max(current, Ordering::SeqCst);

            tokio::time::sleep(Duration::from_millis(20)).await;

            in_flight_clone.fetch_sub(1, Ordering::SeqCst);
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    let peak = max_observed.load(Ordering::SeqCst);
    assert!(peak <= capacity, "In-flight concurrency ({peak}) exceeded configured capacity ({capacity})");
    assert_eq!(in_flight.load(Ordering::SeqCst), 0, "All permits should be returned to semaphore");
}

