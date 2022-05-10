use actix_web::Scope;
use crate::{kafka, health_check};

pub fn providers() -> Vec<Scope> {
	let mut v = vec![];

	v.push(kafka::register());
	v.push(health_check::register());

	v
}