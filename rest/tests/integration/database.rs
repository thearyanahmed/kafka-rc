use sqlx::{Connection, MySqlConnection, MySqlPool};
use clockwork::config::database::DatabaseConfig;
use sqlx::Executor;

// Configures the database. Creates a connection pool and runs migration.
pub async fn configure_database(config: &DatabaseConfig) -> MySqlPool {

    let mut connection = MySqlConnection::connect_with(&config.without_db())
        .await
        .expect("failed to connect to database.");

    let query = format!("CREATE DATABASE {}", &config.db_name.clone());

    // create a database
    connection.execute(
        query.as_str()
    )
        .await
        .expect("failed to create database.");

    let connection_pool = MySqlPool::connect_with(config.with_db())
        .await
        .expect("failed to connect to pool.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("failed to migrate.");

    println!("[+] database \"{}\" created and connected.\n",&config.db_name);

    connection_pool
}