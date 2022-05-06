use actix_web::{guard, Scope, web};

use crate::health_check::routes::{health_check, hello_world};

mod routes;

pub fn register() -> Scope {
	web::scope("/v1/status")
		.guard(guard::Header("Content-Type","application/json"))
		.service(
			web::resource("/health-check").route(
				web::route()
					.guard(guard::Get())
					.to(health_check),
			)
		)
		.service(
			web::resource("/hello-world").route(
				web::route()
					.guard(guard::Get())
					.to(hello_world),
			)
		)
}