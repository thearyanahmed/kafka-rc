use clockwork::config::Config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("failed to read .env file.");

    let _config = Config::get().expect("could not build config.");

    Ok(())
}

