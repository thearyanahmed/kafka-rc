use crate::config::app::AppConfig;

pub mod app;

pub struct Config {
    pub app: AppConfig,
}

impl Config {
    pub fn get() -> Result<Self, envy::Error> {
        let app = get_app_config()?;

        Ok(Config { app })
    }
}

pub fn get_app_config() -> Result<AppConfig,envy::Error> {
    match envy::from_env::<AppConfig>() {
        Ok(config) => Ok(config),
        Err(e) => return Err(e),
    }
}
