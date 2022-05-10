use actix_web::{guard, Scope, web};
use crate::kafka::routes::producers;

mod routes;

pub fn register() -> Scope {
	web::scope("/v1/kafka")
		.service(
			web::resource("/producers").route(
				web::route()
					.guard(guard::Get())
					.to(producers),
			)
		)
}