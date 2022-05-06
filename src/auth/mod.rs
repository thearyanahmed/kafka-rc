use actix_web::{Scope, web};
use crate::auth::routes::route_a;

mod routes;

pub fn register() -> Scope {
	web::scope("/api/v1/auth")
		.route("/auth",web::get().to(route_a))
}