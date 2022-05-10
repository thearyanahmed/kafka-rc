use actix_web::{guard, Scope, web};
use crate::auth::routes::a_random_route;

mod routes;

pub fn register() -> Scope {
	web::scope("/v1/auth")
		.guard(guard::Header("Content-Type","application/json"))
		.service(
			web::resource("/profile").route(
				web::route()
					.guard(guard::Get())
					.to(a_random_route),
			)
		)
}