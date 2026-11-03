#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 3306,
            user: "root".to_string(),
            password: "password".to_string(),
            dbname: "webhookdb".to_string(),
        }
    }
}

impl DatabaseConfig {
    #[must_use]
    pub fn try_load_env() -> Self {
        dotenvy::dotenv().ok();

        let mut config = Self::default();

        if let Ok(user) = std::env::var("DB_USER") {
            config.user = user;
        }

        if let Ok(password) = std::env::var("DB_PASSWORD") {
            config.password = password;
        }

        config
    }
}
