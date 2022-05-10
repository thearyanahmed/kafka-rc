use uuid::Uuid;
use clockwork::bootstrap::{ApplicationBuilder, get_connection_pool};
use clockwork::config::Config;
use crate::{configure_database, TestApplication};

pub async fn app() -> TestApplication {
    dotenv::from_filename(".env.testing").expect("could not read .env.testing file.");

    let mut config = Config::get().expect("could not build config.");

    let db_name = Uuid::new_v4().to_string();

    config.database.db_name = (&db_name[..7]).parse().unwrap();

    let _ = configure_database(&config.database)
        .await;

    let app = ApplicationBuilder::build(&config)
        .await
        .expect("failed to build application.");

    let port = app.port();

    let _ = tokio::spawn(app.serve());

    let db_pool = get_connection_pool(&config.database);

    let address = format!("http://localhost:{}",port);

    let db_name = config.database.db_name.clone();

    TestApplication {
        address,
        port,
        db_pool,
        db_name,
    }
}
