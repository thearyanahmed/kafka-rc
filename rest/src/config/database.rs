use secrecy::{ExposeSecret, Secret};

#[allow(unused_imports)]
use sqlx::ConnectOptions;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};

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
    pub fn without_db(&self) -> MySqlConnectOptions {
        let ssl_mode = if self.db_require_ssl {
            MySqlSslMode::Required
        } else {
            MySqlSslMode::Preferred
        };

        MySqlConnectOptions::new()
            .host(&self.db_host)
            .username(&self.db_username)
            .password(&self.db_password.expose_secret())
            .port(self.db_port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> MySqlConnectOptions {
        self.without_db().database(&self.db_name)
    }
}
