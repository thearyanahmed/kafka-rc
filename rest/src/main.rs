use clockwork::bootstrap::ApplicationBuilder;
use clockwork::config::Config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("failed to read .env file.");

    let config = Config::get().expect("could not build config.");

    let app = ApplicationBuilder::build(&config).await?;

    println!("running on : {:}",app.base_url());

    app.serve().await?;

    Ok(())
}
