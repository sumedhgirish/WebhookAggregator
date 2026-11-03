pub mod config;
pub mod dispatch;
pub mod query;

pub use config::DatabaseConfig;
pub use mysql_async::prelude::*;

use crate::info;

impl DatabaseConfig {
    #[must_use]
    pub fn to_opts(&self, with_db: bool) -> mysql_async::OptsBuilder {
        let b = mysql_async::OptsBuilder::default()
            .ip_or_hostname(self.host.clone())
            .tcp_port(self.port)
            .user(Some(self.user.clone()))
            .pass(Some(self.password.clone()));

        if with_db {
            b.db_name(Some(self.dbname.clone()))
        } else {
            b
        }
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pool: mysql_async::Pool,
}

impl Database {
    pub async fn init() -> Result<Self, mysql_async::Error> {
        info!("Initializing database");

        let env = DatabaseConfig::try_load_env();

        let pool = mysql_async::Pool::new(env.to_opts(false));
        let mut conn = pool.get_conn().await?;

        conn.query_drop(r"CREATE DATABASE IF NOT EXISTS webhookdb;")
            .await?;

        drop(conn);
        pool.disconnect().await?;

        let pool = mysql_async::Pool::new(env.to_opts(true));
        let mut conn = pool.get_conn().await?;

        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS clients (
                        id         INT UNSIGNED AUTO_INCREMENT NOT NULL,
                        name       VARCHAR(20)  NOT NULL,
                        public_key BINARY(32)   NOT NULL,

                        PRIMARY KEY (id),
                        UNIQUE KEY (name)
                     );",
        )
        .await?;

        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS services (
                        id         INT UNSIGNED AUTO_INCREMENT NOT NULL,
                        name       VARCHAR(20)  NOT NULL,
                        url        VARCHAR(100) NOT NULL,
                        status     ENUM('UP', 'DOWN') NOT NULL DEFAULT 'DOWN',
                        capacity   INT UNSIGNED NOT NULL DEFAULT 10,

                        PRIMARY KEY (id),
                        UNIQUE KEY (name)
                     );",
        )
        .await?;

        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS routes (
                        endpoint    VARCHAR(50) NOT NULL,
                        service_id  INT UNSIGNED NOT NULL,

                        PRIMARY KEY (endpoint),
                        FOREIGN KEY (service_id) REFERENCES services (id)
                            ON DELETE CASCADE
                     );",
        )
        .await?;

        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS queries (
                        seq        BIGINT UNSIGNED AUTO_INCREMENT NOT NULL,
                        client_id  INT UNSIGNED NOT NULL,
                        event_id   BINARY(16) NOT NULL,
                        service_id INT UNSIGNED NOT NULL,
                        timestamp  DATETIME NOT NULL,
                        recv_time  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        payload    JSON NOT NULL,
                        status     ENUM('PENDING', 'PROCESSING', 'SUCCESS', 'FAILED') NOT NULL DEFAULT 'PENDING',
                        signature  BINARY(64) NOT NULL,

                        PRIMARY KEY (seq),
                        UNIQUE KEY dedup (client_id, event_id, service_id),
                        KEY work (service_id, status, seq),
                        FOREIGN KEY (client_id) REFERENCES clients (id)
                            ON DELETE CASCADE,
                        FOREIGN KEY (service_id) REFERENCES services (id)
                            ON DELETE CASCADE
                     );",
        )
        .await?;

        Ok(Self { pool })
    }

    pub async fn close(&mut self) -> Result<(), mysql_async::Error> {
        info!("Closing database connection");
        self.pool.clone().disconnect().await?;

        Ok(())
    }
}
