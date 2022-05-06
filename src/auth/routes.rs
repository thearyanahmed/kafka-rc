use actix_web::{HttpResponse, web};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
	message: String,
}

pub async fn a_random_route() -> HttpResponse {
	let res = HelloResponse { message: "this is a random route with a custom header response".to_string() };

	HttpResponse::Ok().insert_header(("Custom","Header")).json(web::Json(res))
}