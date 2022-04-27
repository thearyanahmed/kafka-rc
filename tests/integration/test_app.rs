use std::net::TcpListener;
use std::thread::sleep;
use std::time;
use sqlx::{Connection, PgConnection, PgPool};
use uuid::Uuid;
use tokio::runtime::Runtime;
use clockwork::bootstrap::{Application, ApplicationBaseUrl, ApplicationBuilder};
use clockwork::config::Config;
use clockwork::config::database::DatabaseConfig;
use sqlx::Executor;

pub struct TestApplication {
	// pub server: Server,
	// pub base_url: ApplicationBaseUrl,
	pub db_pool: PgPool,
	pub db_name: String,
	pub address: String,
}

impl Drop for TestApplication {
	fn drop(&mut self) {
		let (tx, rx) = std::sync::mpsc::channel();
		let db_name = self.db_name.clone();

		sleep(time::Duration::from_secs(10));

		std::thread::spawn(move || {
			let rt = Runtime::new().unwrap();
			rt.block_on(async {
				let config = Config::get().expect("failed to read configuration.");
				let mut conn = PgConnection::connect_with(&config.database.without_db())
					.await
					.expect("Failed to connect to Postgres");

				// Kick off open sessions (from the spawned app). This could be replaced
				// by WITH (FORCE) in Postgres 13+.
				conn.execute(&*format!(
					"SELECT pg_terminate_backend(pg_stat_activity.pid)
                    FROM pg_stat_activity
                    WHERE datname = '{}'
                      AND pid <> pg_backend_pid();",
					db_name
				))
					.await
					.expect("Failed to disconnect other sessions");

				conn.execute(&*format!("DROP DATABASE \"{}\";", db_name))
					.await
					.expect(&format!("Failed to drop temporary database: {}", db_name));
				println!("Dropped database: {db_name}");
				let _ = tx.send(());
			})
		});

		let _ = rx.recv();
		println!("ran test teardown");
	}
}

pub async fn spawn_app() -> TestApplication {
	dotenv::from_filename(".env.testing").expect("could not read .env.testing file.");

	let mut config = Config::get().expect("could not build config.");

	config.database.db_name = Uuid::new_v4().to_string();

	let builder = ApplicationBuilder::new();

	let address = format!("{}:{}",&config.app.host,&config.app.port);

	let listener = TcpListener::bind(&address).expect("tcp listening failed.");

	let db_pool = configure_database(&config.database).await;

	let base_url = ApplicationBaseUrl(address.clone());

	let server = builder.spin_server(listener, db_pool, base_url.clone()).expect("server build failed.");

	let app = Application::with(server,base_url);

	let _ = tokio::spawn(app.serve());

	let db_pool = builder.get_connection_pool(&config.database);

	TestApplication {
		address,
		db_pool,
		db_name: config.database.db_name,
	}
}

// Configures the database. Creates a connection pool and runs migration.
pub async fn configure_database(config: &DatabaseConfig) -> PgPool {
	println!("db config {:?}",config);
	let mut connection = PgConnection::connect_with(&config.without_db())
		.await
		.expect("failed to connect to postgres.");

	// create a database
	connection.execute(
		format!(
			r#"CREATE DATABASE "{}" WITH OWNER {};"#,
			config.db_name,
			config.db_username
		).as_str()
	)
		.await
		.expect("failed to created database.");

	let connection_pool = PgPool::connect_with(config.with_db()).await.expect("failed to connect to pool.");

	sqlx::migrate!("./migrations")
		.run(&connection_pool)
		.await
		.expect("failed to migrate.");

	return connection_pool;
}