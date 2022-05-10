use sqlx::{Connection, MySqlConnection, MySqlPool};
use sqlx::Executor;
use tokio::runtime::Runtime;
use clockwork::config::Config;
use crate::database::configure_database;

mod health_check;
mod database;
mod helpers;

pub struct TestApplication {
	pub db_pool: MySqlPool,
	pub db_name: String,
	pub address: String,
	pub port: u16,
}

impl TestApplication {
	pub fn url(&self, path: &str) -> String {
		format!("{}{}",self.address,path)
	}
}