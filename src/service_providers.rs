use actix_web::Scope;
use crate::auth;

pub fn providers() -> Vec<Scope> {
	let mut v = vec![];

	v.push(auth::register());


	v
}