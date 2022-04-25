use clockwork::config::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let config = get_configuration().expect("could not read .env file.");

    println!("config {:?}",config.app);

    Ok(())
}