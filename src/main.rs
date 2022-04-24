use clockwork::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = get_configuration().expect("could not read .env file.");

    println!("config {:?}",config);

    Ok(())
}
