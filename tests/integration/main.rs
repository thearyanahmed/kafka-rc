use sqlx::{Connection, MySqlConnection, MySqlPool};
use sqlx::Executor;
use uuid::Uuid;

use clockwork::bootstrap::{ApplicationBuilder, get_connection_pool};
use clockwork::config::Config;
use clockwork::config::database::DatabaseConfig;

mod health_check;

pub struct TestApplication {
	// pub server: Server,
	// pub base_url: ApplicationBaseUrl,
	pub db_pool: MySqlPool,
	pub db_name: String,
	pub address: String,
	pub port: u16,
}

impl TestApplication {
	pub fn url(&self, path: &str) -> String {
		format!("{}/{}",self.address,path)
	}

}

pub async fn spawn_app() -> TestApplication {
	dotenv::from_filename(".env.testing").expect("could not read .env.testing file.");

	let mut config = Config::get().expect("could not build config.");

	let db_name = Uuid::new_v4().to_string();

	config.database.db_name = (&db_name[..7]).parse().unwrap();

	let _ = configure_database(&config.database)
		.await;

	let app = ApplicationBuilder::build(&config)
		.await
		.expect("failed to build application.");

	let port = app.port();

	let _ = tokio::spawn(app.serve());

	let db_pool = get_connection_pool(&config.database);

	let address = format!("http://localhost:{}",port);

	let db_name = config.database.db_name.clone();

	TestApplication {
		address,
		port,
		db_pool,
		db_name,
	}
}

// Configures the database. Creates a connection pool and runs migration.
pub async fn configure_database(config: &DatabaseConfig) -> MySqlPool {

	println!("created a database called {} ", config.db_name);

	let mut connection = MySqlConnection::connect_with(&config.without_db())
		.await
		.expect("failed to connect to database.");

	let query = format!("CREATE DATABASE {}", &config.db_name.clone());

	// create a database
	connection.execute(
		query.as_str()
	)
		.await
		.expect("failed to create database.");

	let connection_pool = MySqlPool::connect_with(config.with_db())
		.await
		.expect("failed to connect to pool.");

	sqlx::migrate!("./migrations")
		.run(&connection_pool)
		.await
		.expect("failed to migrate.");

	println!("database created and connected.");

	return connection_pool;
}