use actix_web::{HttpResponse, web};
use serde::Serialize;

#[derive(Serialize)]
struct HealthCheckResponse {
	status: String,
}

pub async fn health_check() -> HttpResponse {
	let res = HealthCheckResponse { status: "the string has changed".to_string() };

	HttpResponse::Ok().json(web::Json(res))
}
pub async fn hello_world() -> HttpResponse {
	let res = HealthCheckResponse { status: "hello changed world".to_string() };

	HttpResponse::Ok().json(web::Json(res))
}