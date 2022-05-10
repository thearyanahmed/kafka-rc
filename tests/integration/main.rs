mod health_check;
mod helpers;

pub struct TestApplication {
	pub address: String,
	pub port: u16,
}

impl TestApplication {
	pub fn url(&self, path: &str) -> String {
		format!("{}{}", self.address, path)
	}
}