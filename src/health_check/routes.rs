use actix_web::{HttpResponse, web};
use serde::Serialize;

#[derive(Serialize)]
struct HealthCheckResponse {
	status: String,
}

pub async fn health_check() -> HttpResponse {
	let res = HealthCheckResponse { status: "ok".to_string() };

	HttpResponse::Ok().json(web::Json(res))
}