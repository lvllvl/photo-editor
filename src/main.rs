mod api;
mod db;

use db::create_pool;
use api::start_server;
use dotenv::dotenv;

// use deadpool_postgres::{Config, Pool};

#[tokio::main]
async fn main() -> Result<(), api::MyError>
{
    dotenv().ok(); // Load variables from .env file

    let db_name = std::env::var("DB_NAME").expect("DB_NAME not set");
    println!("DB_NAME: {}", db_name);

    // Create the database connection pool
    let pool = create_pool();

    // Start the API server
    start_server(pool).await
}
