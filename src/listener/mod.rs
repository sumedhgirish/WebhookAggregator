pub mod request;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::post,
};
use tokio::net::TcpListener;

use crate::Database;
use crate::db::query::{Query, QueryErr};
use crate::dispatcher::Doorbells;
use crate::info;
use request::Request;

pub async fn request_handler(
    State((db, doorbells)): State<(Database, Doorbells)>,
    Path(endpoint): Path<String>,
    Json(request): Json<Request>,
) -> Result<StatusCode, QueryErr> {
    info!("Got query on endpoint: {endpoint}");
    let query = Query::try_from(request)?;

    if let Some(service_id) = db.process_query(&query, &endpoint).await? {
        doorbells.ring(service_id);
    }

    Ok(StatusCode::OK)
}

pub struct WebListener {
    iface: String,
    port: u16,
    pub router: Router,
}

impl WebListener {
    pub fn create_router(database: Database, doorbells: Doorbells) -> Router {
        Router::new()
            .route("/webhook/{endpoint}", post(request_handler))
            .with_state((database, doorbells))
    }

    #[must_use]
    pub fn init(database: Database, doorbells: Doorbells) -> Self {
        let router = Self::create_router(database, doorbells);

        Self {
            iface: "0.0.0.0".to_string(),
            port: 31337,
            router,
        }
    }

    pub async fn run(self) -> std::io::Result<()> {
        let addr = format!("{}:{}", self.iface, self.port);
        let listener = TcpListener::bind(&addr).await?;
        info!("Listening on {addr}");

        axum::serve(listener, self.router)
            .with_graceful_shutdown(async {
                let _ = tokio::signal::ctrl_c().await;
                info!("Shutdown signal received, stopping listener");
            })
            .await
    }
}
