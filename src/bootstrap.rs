use std::fmt::Debug;
use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;


use crate::config::Config;
use crate::config::database::DatabaseConfig;
use crate::service_providers::providers;

pub struct Application {
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

	pub fn with(server: Server, base_url: ApplicationBaseUrl) -> Self {
		Application {
			server,
			base_url,
		}
	}

	pub fn base_url(&self) -> &str {
		&self.base_url.0.as_str()
	}
}

impl ApplicationBuilder {

	pub fn new() -> Self {
		Self {}
	}

	pub async fn build(config: &Config) -> Result<Application, std::io::Error> {
		let address = format!("{}:{}",&config.app.host,&config.app.port);

		let listener = TcpListener::bind(&address)?;

		let builder = ApplicationBuilder{};

		let database = builder.get_connection_pool(&config.database);

		let base_url = ApplicationBaseUrl(address);

		let server = builder.spin_server(listener,database,base_url.clone())?;

		let app = Application {
			server,
			base_url,
		};

		Ok(app)
	}

	pub fn get_connection_pool(&self, conf: &DatabaseConfig) -> usize {
		1
	}

	pub fn spin_server(&self, listener: TcpListener, connection_pool: usize, base_url: ApplicationBaseUrl) -> Result<Server,std::io::Error> {
		let db_pool = web::Data::new(connection_pool);
		let base_url = web::Data::new(base_url);

		let server = HttpServer::new(move || {

				let services = providers();

				let mut app = App::new();

				for svc in services {
					app = app.service(svc);
				}

				app
					.app_data(db_pool.clone())
					.app_data(base_url.clone())
			})
			.listen(listener)?
			.run();

		Ok(server)
	}
}
