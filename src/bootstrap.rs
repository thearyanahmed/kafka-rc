use std::fmt::Debug;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use actix_web::dev::{Server, Service, ServiceFactory};
use actix_web::{App, HttpResponse, HttpServer, options, Scope, web};
use crate::config::Config;
use crate::config::database::DatabaseConfig;
use serde::Serialize;
use crate::service_registry::providers;

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
}

impl ApplicationBuilder {
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

	fn get_connection_pool(&self, conf: &DatabaseConfig) -> PgPool {
		PgPoolOptions::new()
			.connect_timeout(std::time::Duration::from_secs(conf.db_connection_timeout as u64))
			.connect_lazy_with(conf.with_db())
	}

	fn spin_server(&self, listener: TcpListener, connection_pool: PgPool, base_url: ApplicationBaseUrl) -> Result<Server,std::io::Error> {
		let db_pool = web::Data::new(connection_pool);
		let base_url = web::Data::new(base_url);

		let server = HttpServer::new(move || {

				let services = providers();

				let mut app = App::new()
					.route("/health-check",web::get().to(route_a));

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

	fn not_a_random_service(&self) -> Scope {
		web::scope("/api/v1").service(
			web::resource("/projects")
				.route(web::get().to(route_a))
				.app_data(web::Data::new("hello world!"))
		)
	}
}

fn a_random_service() -> Scope {
	web::scope("/api/v1").service(
		web::resource("/users")
			.route(web::get().to(route_users))
			.app_data(web::Data::new("hello world!"))
	)
}

#[derive(Serialize)]
struct SomeResponse {
	message: String,
}

async fn route_a() -> HttpResponse {
	let res = SomeResponse { message: "ok".to_string() };

	HttpResponse::Ok().insert_header(("Custom","Header")).json(web::Json(res))
}

async fn route_users(base_url: web::Data<ApplicationBaseUrl>) -> HttpResponse {
	let res = SomeResponse { message: "from users".to_string() };

	println!("app base url {:?}",&base_url.0);

	HttpResponse::Ok().insert_header(("Custom","Header")).json(web::Json(res))
}