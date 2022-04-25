#[derive(serde::Deserialize, Debug)]
pub struct Config {
    app: AppConfig,
    db: DatabaseConfig,
}

#[derive(serde::Deserialize, Debug)]
pub struct AppConfig {
    host: String,
    port: u16
}

#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    connection_url: String,
}

pub fn get_configuration() -> Result<Config,envy::Error> {
    let app_config : AppConfig;

    match envy::from_env::<AppConfig>() {
        Ok(config) => app_config = config,
        Err(e) => return Err(e),
    };

    let db: DatabaseConfig;

    match envy::from_env::<DatabaseConfig>() {
        Ok(config) => db = config,
        Err(e) => return Err(e),
    };

    let config = Config { app: app_config, db };

    Ok(config)
}
