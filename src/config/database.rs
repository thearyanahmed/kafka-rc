use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
#[allow(unused_imports)]
use sqlx::ConnectOptions;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub db_username: String,
    pub db_password: Secret<String>,
    pub db_port: u16,
    pub db_host: String,
    pub db_name: String,
    pub db_require_ssl: bool,
}

impl DatabaseConfig {
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.db_require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.db_host)
            .username(&self.db_username)
            .password(&self.db_password.expose_secret())
            .port(self.db_port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.db_name)
    }
}

pub fn get_connection_pool(conf: &DatabaseConfig) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(conf.with_db())
}
