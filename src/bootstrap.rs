use crate::config::Config;

pub struct Application {}

impl Application {
	pub async fn bootstrap(config: &Config) -> Result<Self, std::io::Error> {
		println!("got config {}",config.app.host);

		Ok(Application {})
	}

	pub fn run(&self) {
		println!("RUN")
	}
}