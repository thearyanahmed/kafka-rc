use actix_web::{Scope, web};
use crate::auth::routes::route_a;

mod routes;

pub fn register() -> Scope {
	web::scope("/api/v1").service(
		web::resource("/auth")
			.route(web::get().to(route_a))
			.app_data(web::Data::new("hello world!"))
	)
}