use std::fmt::Debug;
use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySqlPool};

use crate::config::Config;
use crate::config::database::DatabaseConfig;
use crate::service_providers::providers;

pub struct Application {
	port: u16,
	server: Server,
	base_url: ApplicationBaseUrl,
}

pub struct ApplicationBuilder {}

#[derive(Clone, Debug)]
pub struct ApplicationBaseUrl(pub String);

impl Application {
	pub async fn serve(self) -> Result<(), std::io::Error> {
		self.server.await
	}

	pub fn with(server: Server, port: u16, base_url: ApplicationBaseUrl) -> Self {
		Application {
			server,
			port,
			base_url,
		}
	}

	pub fn base_url(&self) -> &str {
		&self.base_url.0.as_str()
	}

	pub fn port(&self) -> u16 {
		self.port
	}
}

impl ApplicationBuilder {

	pub fn new() -> Self {
		Self {}
	}

	pub async fn build(config: &Config) -> Result<Application, std::io::Error> {
		let address = format!("{}:{}",&config.app.host,&config.app.port);

		let listener = TcpListener::bind(&address)?;
		let port = listener.local_addr().unwrap().port();

		let database = get_connection_pool(&config.database);

		let base_url = ApplicationBaseUrl(address);

		let server = spin_server(listener,database,base_url.clone())?;

		let app = Application {
			server,
			port,
			base_url,
		};

		Ok(app)
	}
}

pub fn get_connection_pool(conf: &DatabaseConfig) -> MySqlPool {
	MySqlPoolOptions::new()
		.connect_timeout(std::time::Duration::from_secs(2)) // todo take from .env
		.connect_lazy_with(conf.with_db())
}

pub fn spin_server(listener: TcpListener, connection_pool: MySqlPool, base_url: ApplicationBaseUrl) -> Result<Server,std::io::Error> {
	let db_pool = web::Data::new(connection_pool);
	let base_url = web::Data::new(base_url);

	let server = HttpServer::new(move || {

		let services = xproviders();

		let mut app = App::new();

		for svc in services {
			app = app.service(svc);
		}

		app.configure();
		app
			.app_data(db_pool.clone())
			.app_data(base_url.clone())


	})
		.listen(listener)?
		.run();

	Ok(server)
}

use actix_web::Scope;
use actix_web::web::ServiceConfig;
use crate::{auth, health_check};

pub fn xproviders() -> Vec<Scope> {
	let mut v = vec![];

	v.push(auth::register());
	v.push(health_check::register());

	v
}
