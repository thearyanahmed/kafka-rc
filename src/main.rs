use clockwork::config::get_app_config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let config = get_app_config().expect("could not read .env file.");

    println!("config {:?}",config);

    Ok(())
}

