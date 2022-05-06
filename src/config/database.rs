use secrecy::{ExposeSecret, Secret};
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[allow(unused_imports)]
use sqlx::PgPool;
#[allow(unused_imports)]
use sqlx::ConnectOptions;

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
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.db_require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        println!("pulling database information\n database = {}  \n username = {}",self.db_name.clone(),self.db_username.clone());

        PgConnectOptions::new()
            .host(&self.db_host)
            .username(&self.db_username)
            .password(&self.db_password.expose_secret())
            .port(self.db_port)
            .ssl_mode(ssl_mode)
    }

    pub fn with_db(&self) -> PgConnectOptions {
        let options = self.without_db().database(&self.db_name);

        return options
    }
}
