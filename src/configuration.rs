#[derive(serde::Deserialize, Debug)]
pub struct Config {
    host: String,
    port: u16
}

pub fn get_configuration() -> Result<Config,envy::Error> {
    dotenv::dotenv().expect("Failed to read .env file");

    return match envy::from_env::<Config>() {
        Ok(config) => Ok(config),
        Err(e) => Err(e),
    };
}
