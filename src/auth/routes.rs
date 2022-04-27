use actix_web::{HttpResponse, web};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
	message: String,
}

pub async fn route_a() -> HttpResponse {
	let res = HelloResponse { message: "internal mod stuff".to_string() };

	HttpResponse::Ok().insert_header(("Custom","Header")).json(web::Json(res))
}