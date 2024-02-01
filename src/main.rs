#![allow(dead_code)]
mod api;
mod db; // Declare the db module 
use db::create_pool;
use api::start_server;
use deadpool_postgres::{Config, Pool};
use dotenv::dotenv;

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
