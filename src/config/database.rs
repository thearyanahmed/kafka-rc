#[derive(serde::Deserialize, Debug)]
pub struct DatabaseConfig {
    pub connection_url: String,
}
