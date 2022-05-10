use actix_web::{HttpResponse, web};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
	message: String,
	producers: Vec<String>,
}

pub async fn producers() -> HttpResponse {
	let v = vec![];

	let res = HelloResponse {
		message: "list of producers".to_string(),
		producers: v,
	};

	HttpResponse::Ok().json(web::Json(res))
}