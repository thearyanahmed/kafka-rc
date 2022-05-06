use actix_web::{guard, HttpResponse, Resource, Scope, web};
use actix_web::dev::WebService;
use crate::health_check::routes::health_check;

mod routes;

pub fn register() -> Scope {
	web::scope("/api/v1")
		.service(
			web::resource("/health-check").route(
				web::route()
					.guard(guard::Get())
					// .guard(guard::Header("content-type", "application/json"))
					.to(health_check),
			)
		)

}