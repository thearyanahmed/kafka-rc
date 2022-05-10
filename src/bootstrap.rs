use std::fmt::Debug;
use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;

use crate::config::Config;
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

	pub async fn build(config: &Config) -> Result<Application, std::io::Error> {
		let address = format!("{}:{}",&config.app.host,&config.app.port);

		let listener = TcpListener::bind(&address)?;

		let port = listener.local_addr().unwrap().port();

		let base_url = ApplicationBaseUrl(address);

		let server = spin_server(listener,base_url.clone())?;

		let app = Application {
			server,
			port,
			base_url,
		};

		Ok(app)
	}
}

pub fn spin_server(listener: TcpListener, base_url: ApplicationBaseUrl) -> Result<Server,std::io::Error> {
	let base_url = web::Data::new(base_url);

	let server = HttpServer::new(move || {

		let services = providers();

		let mut app = App::new();
		//
		for svc in services {
			app = app.service(svc);
		}

		app.app_data(base_url.clone())


	})
		.listen(listener)?
		.run();

	Ok(server)
}
