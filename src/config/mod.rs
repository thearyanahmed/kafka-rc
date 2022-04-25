use crate::config::app::AppConfig;
use crate::config::database::DatabaseConfig;

pub mod app;
pub mod database;

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub app: AppConfig,
    pub db: DatabaseConfig,
}

pub fn get_configuration() -> Result<Config,envy::Error> {
    let app : AppConfig;

    match envy::from_env::<AppConfig>() {
        Ok(config) => app = config,
        Err(e) => return Err(e),
    };

    let db: DatabaseConfig;

    match envy::from_env::<DatabaseConfig>() {
        Ok(config) => db = config,
        Err(e) => return Err(e),
    };

    let config = Config { app, db };

    Ok(config)
}

pub fn get_database_config() -> Result<DatabaseConfig,envy::Error> {
    match envy::from_env::<DatabaseConfig>() {
        Ok(config) => Ok(config),
        Err(e) => return Err(e),
    }
}