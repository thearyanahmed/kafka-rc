use crate::config::app::AppConfig;
use crate::config::database::DatabaseConfig;

pub mod app;
pub mod database;

pub fn get_app_config() -> Result<AppConfig,envy::Error> {
    match envy::from_env::<AppConfig>() {
        Ok(config) => Ok(config),
        Err(e) => return Err(e),
    }
}

pub fn get_database_config() -> Result<DatabaseConfig,envy::Error> {
    match envy::from_env::<DatabaseConfig>() {
        Ok(config) => Ok(config),
        Err(e) => return Err(e),
    }
}