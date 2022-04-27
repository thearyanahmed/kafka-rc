use clockwork::bootstrap::Application;
use clockwork::config::Config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("failed to read .env file.");

    let config = Config::get().expect("could not build config.");

    let app = Application::bootstrap(&config).await?;

    app.run();

    Ok(())
}

