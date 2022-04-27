use actix_web::Scope;
use crate::{auth, health_check};

pub fn providers() -> Vec<Scope> {
	let mut v = vec![];

	v.push(auth::register());
	v.push(health_check::register());

	v
}