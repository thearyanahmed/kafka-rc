use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use crate::config::Config;
use crate::config::database::DatabaseConfig;

pub struct Application {
	server: Server,
	baseurl: ApplicationBaseUrl
}

pub struct ApplicationBuilder {}

pub struct ApplicationBaseUrl(pub String);

impl Application {
	pub async fn run(self) -> Result<(), std::io::Error> {
		self.server.await
	}
}

impl ApplicationBuilder {
	pub async fn build(config: &Config) -> Result<Application, std::io::Error> {
		println!("got config {}",config.app.host);

		let address = format!("{}:{}",&config.application.host,&config.application.port);

		let builder = ApplicationBuilder{};

		println!("address: {}",address);

		let listener = TcpListener::bind(&address)?;

		println!("listener port : {}",listener.local_addr().unwrap().port());

		let database = builder.get_connection_pool(&config.database);

		let base_url = ApplicationBaseUrl(address);

		let server = builder.spin_server(listener,database,base_url)?;

		let app = Application {
			server,
			baseurl,
		};

		Ok(app)
	}

	fn get_connection_pool(&self, conf: &DatabaseConfig) -> PgPool {
		PgPoolOptions::new()
			.connect_timeout(std::time::Duration::from_secs(conf.db_connection_timeout as u64))
			.connect_lazy_with(conf.with_db())
	}

	fn spin_server(&self, listener: TcpListener, connection_pool: PgPool, base_url: ApplicationBaseUrl) -> Result<Server,std::io::Error> {
		let db_pool = web::Data::new(connection_pool);
		let base_url = web::Data::new(base_url);

		let server = HttpServer::new(move || {
				App::new()
					.route("/health-check",web::get().to(route_a))
					.app_data(db_pool.clone())
					.app_data(base_url.clone())
			})
			.listen(listener)
			.run();

		Ok(server)
	}
}

async fn route_a() -> HttpResponse {
	HttpResponse::Ok().finish()
}