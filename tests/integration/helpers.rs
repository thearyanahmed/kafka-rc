use clockwork::bootstrap::ApplicationBuilder;
use clockwork::config::Config;
use crate::{TestApplication};

pub async fn app() -> TestApplication {
    dotenv::from_filename(".env.testing").expect("could not read .env.testing file.");

    let config = Config::get().expect("could not build config.");

    let app = ApplicationBuilder::build(&config)
        .await
        .expect("failed to build application.");

    let port = app.port();

    let _ = tokio::spawn(app.serve());

    let address = format!("http://localhost:{}",port);

    TestApplication {
        address,
        port,
    }
}
