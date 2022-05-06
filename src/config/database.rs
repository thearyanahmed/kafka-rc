use secrecy::{ExposeSecret, Secret};

#[allow(unused_imports)]
use sqlx::ConnectOptions;
use sqlx::mysql::{MySqlPoolOptions, MySqlSslMode};

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub db_username: String,
    pub db_password: Secret<String>,
    pub db_port: u16,
    pub db_host: String,
    pub db_name: String,
    pub db_require_ssl: bool,
    pub db_connection_timeout: u16, // in seconds
}

impl DatabaseConfig {
    pub fn without_db(&self) -> MySqlPoolOptions {
        let ssl_mode = if self.db_require_ssl {
            MySqlSslMode::Required
        } else {
            MySqlSslMode::Preferred
        };

        MySqlPoolOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> MySqlPoolOptions {
        self.without_db().database(&self.db_name)
    }
}
