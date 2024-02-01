// use crate::db;
// use crate::db::*; 
// use actix_web::{web, App, http, HttpResponse, HttpServer, Responder, test};
// use deadpool_postgres::{Config, Pool};
// use serde::Deserialize;
// use serde_json::json;
// use tokio_postgres::{Error, NoTls, Row};




// #[cfg(test)]
// mod tests
// {
//     use super::*;
//     use dotenv::dotenv;
//     use std::env;

//     //////////////////////////////////////////////////////////////////////////////
//     ///////////////////// ********** Setup ********** ////////////////////////////
//     //////////////////////////////////////////////////////////////////////////////

//     // Test user struct
//     struct TestUser
//     {
//         username: String,
//         email: String,
//         user_id: i32,
//     }
//     impl TestUser
//     {
//         // Create a new user in database
//         async fn create(pool: &Pool) -> Result<Self, MyDbError>
//         {
//             let username = format!("user_{}", rand::random::<u32>());
//             let email = format!("{}@example.com", username);
//             let user_id = db::users::add_user(pool, &username, &email).await?;
//             Ok(TestUser { username,
//                           email,
//                           user_id })
//         }

//         // Cleanup test user in database
//         async fn cleanup(self, pool: &Pool) -> Result<(), MyDbError>
//         {
//             db::users::delete_user(pool, &self.username).await?;
//             Ok(())
//         }
//     }

//     // Setup mock database connection ////////////////////////////////////////////
//     fn setup() -> Pool
//     {
//         dotenv().ok(); // Load variables from .env file
//         let mut cfg = Config::new();

//         cfg.host = env::var("DB_HOST").ok();
//         cfg.user = env::var("DB_USER").ok();
//         cfg.password = env::var("DB_PASSWORD").ok();
//         cfg.dbname = env::var("DB_NAME").ok();

//         cfg.create_pool(None, NoTls).expect("Failed to create pool")
//     }
//     /////////////////////// ********** Helper Functions ********** ///////////////
//     /// Helper function to create a test session    //////////////////////////////
//     // async fn create_test_session(pool: &Pool, user_id: i32) -> Result<i32, MyDbError>
//     // {
//     //     db::sessions::create_session(pool, user_id).await
//     // }
//     // /////////////////////// ********** User Tests ********** /////////////////////
//     // async fn create_test_user( pool: &Pool ) -> Result<i32, MyDbError> {
//     //     let username = format!("user_{}", rand::random::<u32>());
//     //     let email = format!("{}@example.com", username);
//     //     add_user(pool, &username, &email).await
//     // }

//     // async fn upload_test_image( pool: &Pool, session_id: i32 ) -> Result< i32, MyDbError > {
//     //     let file_path = "./tests/images/testImage.png";
//     //     db::add_image( pool, session_id, file_path  ).await
//     // }

//     //////////////////////////////////////////////////////////////////////////////
//     ///////////////////// ********** User Tests ********** ///////////////////////
//     //////////////////////////////////////////////////////////////////////////////
//     use super::*;
//     use actix_web::{test, App, http::StatusCode};

//     #[tokio::test]
//     async fn test_add_user_handler() {

//         let pool = setup(); // Setup your database connection pool
//         let app = test::init_service(
//             App::new()
//                 .configure(configure_api))
//                 .await;

//         // Define the new user data in JSON format
//         let new_user_data = r#"{"username": "test_user", "email": "test@example.com"}"#;

//         // Create a POST request to the add_user endpoint
//         let req = test::TestRequest::post()
//             .uri("/add_user")
//             .set_json(&new_user_data)
//             .to_request();

//         // Send the request to the app
//         let resp = test::call_service(&app, req).await;

//         // Check the response status
//         assert_eq!(resp.status(), StatusCode::OK); // Or another expected status
//         // assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR); // Or another expected status
//     }


//     //////////////////////////////////////////////////////////////////////////////
//     ///////////////////// ********** Session Tests ********** ////////////////////
//     //////////////////////////////////////////////////////////////////////////////

//     //////////////////////////////////////////////////////////////////////////////
//     ///////////////////// ********** Image Tests ********** //////////////////////
//     //////////////////////////////////////////////////////////////////////////////
//     // #[tokio::test]
//     // async fn test_add_image_handler()
//     // {
//     //     let pool = setup();

//     //     let test_user = TestUser::create(&pool).await.unwrap();
//     //     let session_id = create_test_session(&pool, test_user.user_id).await.unwrap(); // create a test session
//     //                                                                                    // Setup test data
//     //     let file_path = "./tests/images/testImage.png";
//     //     let result = add_image(&pool, session_id, file_path).await;

//     //     assert!(result.is_ok());
//     //     test_user.cleanup(&pool).await.unwrap();
//     // }
// }
