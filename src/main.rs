use dotenv::dotenv;
mod db;
mod api;
use db::{ setup_database, create_pool };
use api::start_server;

#[tokio::main]
async fn main() -> Result<(), api::MyError> {
    dotenv::dotenv().ok(); // Load variables from .env file

    let db_name = std::env::var("DB_NAME").expect("DB_NAME not set");
    println!("DB_NAME: {}", db_name);

    // Create the database connection pool
    let pool = create_pool();

    // Setup database schema - this can be moved to a separate setup script
    // Or adjusted to run only once during the application deployment
    if let Ok( mut client ) = pool.get().await {

        setup_database( &mut client ).await?;
        eprint!("Connected to database.")

    } else {
        eprint!("Failed to connect to database.")
    }

    // Start the server with the database pool
    start_server( pool ).await 
}
