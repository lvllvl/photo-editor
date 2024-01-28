#![allow(dead_code)]
mod api;
mod db;
use actix_web::test;
use db::*;
// use db::{ setup_database, create_pool, add_image };
use api::start_server;
use deadpool_postgres::{Config, Pool};
use dotenv::dotenv;

// TODO: remove this struct + impl
struct TestUser
{
    username: String,
    email: String,
    user_id: i32,
}
impl TestUser
{
    // Create a new user
    async fn create(pool: &Pool) -> Result<Self, MyDbError>
    {
        let username = format!("user_{}", rand::random::<u32>());
        let email = format!("{}@example.com", username);
        let user_id = add_user(pool, &username, &email).await?;
        Ok(TestUser { username,
                      email,
                      user_id })
    }

    // Cleanup test user
    async fn cleanup(self, pool: &Pool) -> Result<(), MyDbError>
    {
        delete_user(pool, &self.username).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), api::MyError>
{
    dotenv().ok(); // Load variables from .env file

    let db_name = std::env::var("DB_NAME").expect("DB_NAME not set");
    println!("DB_NAME: {}", db_name);

    // Create the database connection pool
    let pool = create_pool();

    // Setup database schema - this can be moved to a separate setup script
    // Or adjusted to run only once during the application deployment

    // if let Ok( mut client ) = pool.get().await {

    //     // Setup the database schema
    //     setup_database( &mut client ).await?;
    //     eprint!("Connected to database.");

    //     // TODO: delete this test_user statement
    //     // Setup a testuser for testing purposes
    //     let test_user = TestUser::create( &pool ).await.unwrap();
    //     assert!( test_user.user_id > 0 );

    //     // TODO: delete this match statement
    //     // create a session for the test user
    //     let session_id = match create_test_session( &pool, test_user.user_id ).await {
    //         Ok( session_id ) => {
    //             println!("Test create_session: Session created successfully");
    //             // Cleanup the test user
    //             // delete_session( &pool, session_id ).await.expect("Failed to cleanup test session");
    //             session_id
    //         },
    //         Err(e) => panic!("Test create_session failed: {:?}", e),
    //     };
    //     assert!( session_id > 0 );

    //     // TODO: delete this match statement
    //     // Upload a test image for the test user
    //     let file_path = "./tests/images/testImage.png";
    //     let image_id = add_image(&pool, session_id, file_path).await.unwrap();
    //     assert!( image_id > 0 );

    // } else {
    //     eprint!("Failed to connect to database.")
    // }

    // Start the server with the database pool
    start_server(pool).await
}

/// Create a session for the test user
async fn create_test_session(pool: &Pool, user_id: i32) -> Result<i32, MyDbError>
{
    create_session(pool, user_id).await
}
