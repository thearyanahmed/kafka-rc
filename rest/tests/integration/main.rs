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

impl Drop for TestApplication {
	fn drop(&mut self) {
		let (tx, rx) = std::sync::mpsc::channel();
		let db_name = self.db_name.clone();

		println!("[+] spawning clean up thread.\n");

		std::thread::spawn(move || {
			let rt = Runtime::new().unwrap();
			rt.block_on(async {
				let config = Config::get().expect("could not build config.");

				let mut conn = MySqlConnection::connect_with(&config.database.without_db())
					.await
					.expect("failed to connect to database during shutdown.");

				let query = format!("DROP DATABASE {}", &db_name.clone());

				conn.execute(query.as_str())
					.await
					.expect(&format!("failed to drop temporary database: {}", db_name));
				let _ = tx.send(());
			})
		});

		let _ = rx.recv();

		println!("[+] database {} deleted.\n", self.db_name.clone());

		println!("[+] test thread ended.\n");
	}
}
