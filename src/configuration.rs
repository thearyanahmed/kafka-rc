#[derive(serde::Deserialize, Debug)]
pub struct Config {
    host: String,
    hello: Hello,
}

#[derive(serde::Deserialize, Debug)]
pub struct Hello {
    port: u16
}

pub fn get_configuration() -> Result<Config,envy::Error> {
    return match envy::from_env::<Config>() {
        Ok(config) => Ok(config),
        Err(e) => Err(e),
    };
}
