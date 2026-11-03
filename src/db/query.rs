use super::{params, info, Database, Queryable};

use chrono::{DateTime, Utc};
use ed25519_dalek::{Signature, VerifyingKey};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Query {
    pub signature: [u8; 64], // signed hash of payload
    pub event_id: [u8; 16],
    pub client_name: String,
    pub datetime: DateTime<Utc>,
    pub payload: String,
}

#[derive(Debug)]
pub enum QueryErr {
    BadRequest,
    UnknownClient(String),
    UnknownService(String),
    ServiceUnavailable(String),
    UnauthorizedRequest,
    DatabaseError(mysql_async::Error),
}

impl From<mysql_async::Error> for QueryErr {
    fn from(e: mysql_async::Error) -> Self {
        Self::DatabaseError(e)
    }
}

impl Database {
    async fn get_client_key(&self, query: &Query) -> Result<(u32, [u8; 32]), QueryErr> {
        let mut conn = self.pool.get_conn().await?;

        let row: Option<(u32, Vec<u8>)> = conn
            .exec_first(
                r"SELECT id, public_key FROM clients WHERE name = ?",
                (&query.client_name,),
            )
            .await?;

        let Some((client_id, pub_key)) = row else {
            return Err(QueryErr::UnknownClient(query.client_name.clone()));
        };

        let pub_key: [u8; 32] = pub_key
            .try_into()
            .map_err(|_| QueryErr::UnauthorizedRequest)?;

        Ok((client_id, pub_key))
    }

    async fn get_endpoint_service(&self, endpoint: &str) -> Result<u32, QueryErr> {
        let mut conn = self.pool.get_conn().await?;

        let row: Option<(u32, String, String)> = conn
            .exec_first(
                r"SELECT s.id, s.status, s.name
                        FROM routes r JOIN services s
                            ON r.service_id = s.id
                        WHERE r.endpoint = ?",
                (endpoint,),
            )
            .await?;

        let Some((service_id, status, service_name)) = row else {
            return Err(QueryErr::UnknownService(endpoint.to_string()));
        };

        if status == "DOWN" {
            return Err(QueryErr::ServiceUnavailable(service_name));
        }

        Ok(service_id)
    }

    /// Returns the service to wake when the event is new, `None` for a duplicate
    pub async fn process_query(
        &self,
        query: &Query,
        endpoint: &str,
    ) -> Result<Option<u32>, QueryErr> {
        let ((client_id, pub_key), service_id) = tokio::try_join!(
            self.get_client_key(query),
            self.get_endpoint_service(endpoint)
        )?;

        verify_signature(query, &pub_key)?;

        let mut conn = self.pool.get_conn().await?;

        // The same request seen twice is ignored
        conn.exec_drop(
            r"INSERT INTO queries (client_id, event_id, service_id, timestamp, payload, signature)
                    VALUES (:client_id, :event_id, :service_id, :timestamp, :payload, :signature)
                    ON DUPLICATE KEY UPDATE client_id = client_id",
            params! {
                "client_id" => client_id,
                "event_id" => query.event_id,
                "timestamp" => query.datetime.naive_utc(),
                "payload" => &query.payload,
                "signature" => query.signature,
                "service_id" => service_id
            },
        )
        .await?;

        if conn.affected_rows() == 0 {
            info!(
                "Ignoring duplicate event {} from {}",
                hex::encode(query.event_id),
                query.client_name
            );
            return Ok(None);
        }

        Ok(Some(service_id))
    }
}

#[must_use]
pub fn qhash(query: &Query) -> [u8; 32] {
    let mut h = Sha256::new();

    // Mix content into digest
    h.update(query.client_name.as_bytes());
    h.update(query.event_id);
    h.update(query.datetime.timestamp().to_be_bytes());
    h.update(query.payload.as_bytes());

    h.finalize().into()
}

pub fn verify_signature(query: &Query, pub_key: &[u8; 32]) -> Result<(), QueryErr> {
    let vk = VerifyingKey::from_bytes(pub_key).map_err(|_| QueryErr::UnauthorizedRequest)?;
    let sig = Signature::from_bytes(&query.signature);

    let digest = qhash(query);

    vk.verify_strict(&digest, &sig)
        .map_err(|_| QueryErr::UnauthorizedRequest)
}
