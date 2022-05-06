use actix_web::{guard, Scope, web};
use crate::auth::routes::route_a;

mod routes;

pub fn register() -> Scope {
	// web::scope("/api/v1/auth")
	// 	.route("/auth",web::get().to(route_a))

	web::scope("/v1/auth")
		.guard(guard::Header("Content-Type","application/json"))
		.service(
			web::resource("/profile").route(
				web::route()
					.guard(guard::Get())
					.to(route_a),
			)
		)
}