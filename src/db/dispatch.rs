use super::{Database, Queryable};

impl Database {
    pub async fn get_service(
        &self,
        service_id: u32,
    ) -> Result<(String, usize), mysql_async::Error> {
        let mut conn = self.pool.get_conn().await?;

        let Some(service) = conn
            .exec_first(
                r"SELECT url, capacity FROM services WHERE status = 'UP' AND id = ?;",
                (service_id,),
            )
            .await?
        else {
            unreachable!();
        };

        Ok(service)
    }

    pub async fn pending_services(&self) -> Result<Vec<u32>, mysql_async::Error> {
        let mut conn = self.pool.get_conn().await?;

        conn.query(r"SELECT DISTINCT service_id FROM queries WHERE status = 'PENDING';")
            .await
    }

    // Only the service's own claimer moves its rows out of PENDING, so a plain read
    // followed by an update is race free and needs no row locks
    pub async fn claim(
        &self,
        service_id: u32,
        limit: usize,
    ) -> Result<Vec<(u64, String)>, mysql_async::Error> {
        let mut conn = self.pool.get_conn().await?;

        let jobs: Vec<(u64, String)> = conn
            .exec(
                r"SELECT seq, payload FROM queries
                        WHERE service_id = ? AND status = 'PENDING'
                        ORDER BY seq
                        LIMIT ?;",
                (service_id, limit as u64),
            )
            .await?;

        if jobs.is_empty() {
            return Ok(jobs);
        }

        let seqs: Vec<String> = jobs.iter().map(|(seq, _)| seq.to_string()).collect();
        conn.query_drop(format!(
            r"UPDATE queries
                    SET status = 'PROCESSING'
                    WHERE status = 'PENDING' AND seq IN ({});",
            seqs.join(",")
        ))
        .await?;

        Ok(jobs)
    }

    pub async fn processing(
        &self,
        service_id: u32,
    ) -> Result<Vec<(u64, String)>, mysql_async::Error> {
        let mut conn = self.pool.get_conn().await?;

        conn.exec(
            r"SELECT seq, payload FROM queries
                    WHERE service_id = ? AND status = 'PROCESSING'
                    ORDER BY seq;",
            (service_id,),
        )
        .await
    }

    pub async fn set_query_status(&self, seq: u64, status: &str) -> Result<(), mysql_async::Error> {
        let mut conn = self.pool.get_conn().await?;

        conn.exec_drop(
            r"UPDATE queries
                    SET status = ?
                    WHERE seq = ?;",
            (status, seq),
        )
        .await
    }
}
