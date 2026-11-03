use crate::db::query::{Query, QueryErr};
use crate::{warn, error};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Request {
    pub client_name: String,
    pub nonce: String,
    pub id: String,      // 128 hex chars (64 bytes)
    pub datetime: i64,   // Unix seconds, the same value the client hashed
    pub payload: String, // the exact text that was signed
}

impl TryFrom<Request> for Query {
    type Error = QueryErr;

    fn try_from(req: Request) -> Result<Self, Self::Error> {
        let bytes = hex::decode(&req.id).map_err(|_| QueryErr::BadRequest)?;
        let id: [u8; 64] = bytes.try_into().map_err(|_| QueryErr::BadRequest)?;

        let bytes = hex::decode(&req.nonce).map_err(|_| QueryErr::BadRequest)?;
        let nonce: [u8; 16] = bytes.try_into().map_err(|_| QueryErr::BadRequest)?;

        let datetime =
            DateTime::<Utc>::from_timestamp(req.datetime, 0).ok_or(QueryErr::BadRequest)?;

        Ok(Self {
            signature: id,
            client_name: req.client_name,
            event_id: nonce,
            datetime,
            payload: req.payload,
        })
    }
}

impl IntoResponse for QueryErr {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => {
                warn!("Bad request");
                (StatusCode::BAD_REQUEST, "bad request")
            }
            Self::UnknownClient(client_name) => {
                warn!("Ignoring unknown client with name: {client_name}");
                (StatusCode::UNAUTHORIZED, "unauthorized")
            }
            Self::UnauthorizedRequest => {
                warn!("Request failed verification process");
                (StatusCode::UNAUTHORIZED, "unauthorized")
            }
            Self::DatabaseError(dberr) => {
                error!("Database error: {dberr}");
                (StatusCode::INTERNAL_SERVER_ERROR, "internal error")
            }
            Self::UnknownService(endpoint) => {
                warn!("No service assigned to requested endpoint: {endpoint}");
                (StatusCode::BAD_REQUEST, "bad request")
            }
            Self::ServiceUnavailable(service_name) => {
                error!("Got request to inactive service: {service_name}");
                (StatusCode::SERVICE_UNAVAILABLE, "service unavailable")
            }
        }
        .into_response()
    }
}
