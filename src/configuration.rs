#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub app: AppConfig,
    pub db: DatabaseConfig,
}

#[derive(serde::Deserialize, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub connection_url: String,
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
