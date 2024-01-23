// web api endpoints
// POST /api/transform/rotate for a rotate tool.
// POST /api/filter/blur for applying a blur filter.
// use crate::db::{ add_user, create_pool };
use crate::db;
use crate::db::*; // QUEST: Am I exposing all db.rs funcs by doing this?
use actix_web::{web, App, http, HttpResponse, HttpServer, Responder, test};
use deadpool_postgres::{Config, Pool};
use serde::Deserialize;
use tokio_postgres::{Error, NoTls, Row};

//////////////////////////////////////////////////////////////////////////////////
///////////////// ******* Function to start the server ******* ///////////////////
//////////////////////////////////////////////////////////////////////////////////
pub async fn start_server(pool: Pool) -> Result<(), MyError>
{
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone()))
                  .route("/", web::get().to(index))
                  .route("/add_user", web::post().to(add_user_handler))
                  .route("/user/{username}", web::get().to(get_user_handler))
                  .route("/user/{username}", web::delete().to(delete_user_handler))
        // Other routes
    }).bind("127.0.0.1:8080")? // TODO: Does this need to change in PROD?
      .run()
      .await?;

    Ok(())
}
/// Index handler
async fn index() -> impl Responder
{
    HttpResponse::Ok().body("Welcome to the API!")
}

//////////////////////////////////////////////////////////////////////////////////
pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/add_user")
            .route(web::post().to(add_user_handler))
        // ... other routes
    );
}

  

// Error Enum for server function ////////////////////////////////////////////////
#[derive(Debug)]
pub enum MyError
{
    Io(std::io::Error),
    Postgres(postgres::Error),
    // Other error types as needed HERE
}
impl From<std::io::Error> for MyError
{
    fn from(err: std::io::Error) -> MyError
    {
        MyError::Io(err)
    }
}
impl From<postgres::Error> for MyError
{
    fn from(err: postgres::Error) -> MyError
    {
        MyError::Postgres(err)
    }
}

//////////////////////////////////////////////////////////////////////////////////
////////////// ***** User Route Handler Functions ***** //////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Route handler to add a new user //////////////////////////////////////////////
/// This allows new users to register.
async fn add_user_handler(pool: web::Data<Pool>, new_user: web::Json<NewUser>) -> HttpResponse
{
    match add_user(&pool, &new_user.username, &new_user.email).await
    {
        Ok(_) => HttpResponse::Ok().json("User added successfully"),
        Err(MyDbError::PostgresError(e)) =>
        {
            HttpResponse::InternalServerError().json(format!("Postgres error: {}", e))
        }
        Err(MyDbError::PoolError(e)) =>
        {
            HttpResponse::InternalServerError().json(format!("Pool error: {}", e))
        }
        Err(_) => HttpResponse::InternalServerError().json("Unhandled error"),
        // Handle other errors
    }
}
#[derive(Debug, Deserialize)] // QUEST: What does the Debug macro allow you to do?
struct NewUser
{
    username: String,
    email: String,
}

/// Route handler to get a user by username //////////////////////////////////////
/// This allows users to get their user profile
async fn get_user_handler(pool: web::Data<Pool>, path: web::Path<(String,)>) -> HttpResponse
{
    let username = &path.into_inner().0;

    match pool.get().await
    {
        Ok(mut client) => match db::get_user_by_username(&mut client, username).await
        {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(MyDbError::NotFound) => HttpResponse::NotFound().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Delete user by username //////////////////////////////////////
/// This is for user account deletion.
async fn delete_user_handler(pool: web::Data<Pool>, path: web::Path<(String,)>) -> HttpResponse
{
    let username = &path.into_inner().0;

    match db::delete_user(&pool, username).await
    {
        Ok(_) => HttpResponse::Ok().json("User deleted successfully"),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("User not found"),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),
    }
}

/// Update user email ///////////////////////////////////////////////////////////
/// To allow users to update thier email.
///
/// # Arguements
///
/// * 'pool' - A reference to the database connection pool.
/// * 'path' - A web::Path tuple containing the username.
/// * 'new_email' - The new email address to update
///
/// # Returns
///
/// Return an HttpResponse indicating the outcome of the operation.
///
/// # Example Request
///
/// PUT /users/{username}/update_email
/// Body: { "new_email": "new_email@example.com" }
///
async fn update_user_email_handler(pool: web::Data<Pool>,
                                   path: web::Path<String>,
                                   new_email: String)
                                   -> HttpResponse
{
    let username = path.into_inner();

    match db::update_user_email(&pool, &username, &new_email).await
    {
        Ok(_) => HttpResponse::Ok().json("User email changed successfully"),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("User not found"),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),
    }
}

//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
////////////// *****  Session Route Handler Functions ***** //////////////////////
//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Create_session
/// To start a new user session upon login

/// End_session
/// To allow users to log out OR just end the session after a certain amount of
/// time OR end session after a certain amount of inactivity.

/// Get_active_sessions
/// To retrieve active sessions, useful for administrative purposes.

//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
////////////// *****  Image Route Handler Functions ***** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// ADD IMAGE HANDLER  
/// To allow users to upload an image. This function expect the username to ID
/// the user adn the path of the image to be uploaded.
///
/// # Arguements
///
/// * 'pool' - A reference to the database connection pool.
/// * 'path' - A web::Path tuple containing the username.
/// * 'image_path' - The file path of the image to be uploaded.
/// * 'user_id' - The ID of the user making the request.
///
/// # Returns
///
/// Return an HttpResponse indicating the outcome of the operation.
///
/// # Example Request
///
/// POST /image/{username}/add_image
/// Body: { "image_path": "/path/to/image.png" }
///
async fn add_image_handler(pool: web::Data<Pool>, image_path: String, user_id: i32)
                           -> HttpResponse
{
    // Fetch session ID
    let session_id = match db::get_session_id_for_user(&pool, user_id).await {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json("Error fetching session ID or User not found.")
        }
    };

    // Call the associated function from db.rs
    match db::add_image(&pool, session_id, &image_path).await
    {
        Ok(image_id) =>
        {
            HttpResponse::Ok().json(format!("Image added successfully with ID: {:?}", image_id))
        }
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),
    }
}

// TODO: Get_image
// QUEST: should this return a vector instead of HttpResonse?
async fn get_single_image_handler(pool: web::Data<Pool>, image_id: i32) -> HttpResponse
{
    match db::get_single_image(&pool, image_id).await
    {
        Ok(image) => HttpResponse::Ok().json(image), // Return the image data
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("Image not found."),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),
    }
}

/// Get all iamges
async fn get_all_images_handler(pool: web::Data<Pool>, user_id: i32) -> HttpResponse
{
    // FIXME: Assuming user_id is extracted from authenticated session

    match db::get_all_images(&pool, user_id).await
    {
        Ok(images) => HttpResponse::Ok().json(images), // Return the actual images
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("No images found for this user."),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error!"),
    }
}

/// Update image's file path
// TODO: what other updates would take place? Adjust this fucntion to reflect that
// e.g., any image change
async fn update_image_handler(pool: web::Data<Pool>,
                              path: web::Path<String>,
                              image_id: i32,
                              new_image_path: String)
                              -> HttpResponse
{
    match db::update_image(&pool, image_id, &new_image_path).await
    {
        Ok(_) => HttpResponse::Ok().json("Image path has been updated."),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("Image NOT found."),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error! Oh no."),
    }
}

/// Delete image: Take an image within the database and delete it.
async fn delete_image_handler(pool: web::Data<Pool>, image_id: web::Path<(i32)>) -> HttpResponse
{
    let image_id = image_id.into_inner();
    // Add authorization and validation logic stuff here

    match db::delete_image(&pool, image_id).await
    {
        Ok(_) => HttpResponse::Ok().json(format!("Image with ID {} was deleted succesfully!",
                                                 image_id.to_string())),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("Image NOT found!"),
        Err(_) => HttpResponse::InternalServerError().json("Internal Server Error!"),
    }
}

//////////////////////////////////////////////////////////////////////////////////
////////////// *****  Layer Route Handler Functions ***** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Add Layer: Add a layer to an existing image
/// The default should be 1 layer minimum.
async fn add_layer_handler(pool: web::Data<Pool>,
                           image_id: i32,
                           layer_name: &str,
                           layer_type: &str,
                           layer_data: &[u8],
                           order: i32)
                           -> HttpResponse
{
    match db::add_layer(&pool, image_id, layer_name, layer_type, layer_data, order).await
    {
        Ok(_) => HttpResponse::Ok().json("Image Layer was added successfully!"),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("Layer not added!"),
        Err(_) => HttpResponse::InternalServerError().json("Internal Server Error!"),
    }
}

// Get layer by layer_id

// Update layer

// Delete layer

#[cfg(test)]
mod tests
{
    use super::*;
    use dotenv::dotenv;
    use std::env;

    //////////////////////////////////////////////////////////////////////////////
    ///////////////////// ********** Setup ********** ////////////////////////////
    //////////////////////////////////////////////////////////////////////////////

    // Test user struct
    struct TestUser
    {
        username: String,
        email: String,
        user_id: i32,
    }
    impl TestUser
    {
        // Create a new user in database
        async fn create(pool: &Pool) -> Result<Self, MyDbError>
        {
            let username = format!("user_{}", rand::random::<u32>());
            let email = format!("{}@example.com", username);
            let user_id = add_user(pool, &username, &email).await?;
            Ok(TestUser { username,
                          email,
                          user_id })
        }

        // Cleanup test user in database
        async fn cleanup(self, pool: &Pool) -> Result<(), MyDbError>
        {
            delete_user(pool, &self.username).await?;
            Ok(())
        }
    }

    // Setup mock database connection ////////////////////////////////////////////
    fn setup() -> Pool
    {
        dotenv().ok(); // Load variables from .env file
        let mut cfg = Config::new();

        cfg.host = env::var("DB_HOST").ok();
        cfg.user = env::var("DB_USER").ok();
        cfg.password = env::var("DB_PASSWORD").ok();
        cfg.dbname = env::var("DB_NAME").ok();

        cfg.create_pool(None, NoTls).expect("Failed to create pool")
    }
    /////////////////////// ********** Helper Functions ********** ///////////////
    /// Helper function to create a test session    //////////////////////////////
    async fn create_test_session(pool: &Pool, user_id: i32) -> Result<i32, MyDbError>
    {
        db::create_session(pool, user_id).await
    }
    // /////////////////////// ********** User Tests ********** /////////////////////
    // async fn create_test_user( pool: &Pool ) -> Result<i32, MyDbError> {
    //     let username = format!("user_{}", rand::random::<u32>());
    //     let email = format!("{}@example.com", username);
    //     add_user(pool, &username, &email).await
    // }

    // async fn upload_test_image( pool: &Pool, session_id: i32 ) -> Result< i32, MyDbError > {
    //     let file_path = "./tests/images/testImage.png";
    //     db::add_image( pool, session_id, file_path  ).await
    // }

    //////////////////////////////////////////////////////////////////////////////
    ///////////////////// ********** User Tests ********** ///////////////////////
    //////////////////////////////////////////////////////////////////////////////
    use super::*;
    use actix_web::{test, App, http::StatusCode};

    #[tokio::test]
    async fn test_add_user_handler() {

        let pool = setup(); // Setup your database connection pool
        let app = test::init_service(
            App::new()
                .configure(configure_api))
                .await;
        // let app = test::init_service(App::new()
        //                             .app_data(web::Data::new(pool.clone()))
        //                             .configure( start_server( &pool ) ) // Use your actual server configuration function
        //                             .await);

        // Define the new user data in JSON format
        let new_user_data = r#"{"username": "test_user", "email": "test@example.com"}"#;

        // Create a POST request to the add_user endpoint
        let req = test::TestRequest::post()
            .uri("/add_user")
            .set_json(&new_user_data)
            .to_request();

        // Send the request to the app
        let resp = test::call_service(&app, req).await;

        // Check the response status
        assert_eq!(resp.status(), StatusCode::OK); // Or another expected status
        // assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR); // Or another expected status
    }


    //////////////////////////////////////////////////////////////////////////////
    ///////////////////// ********** Session Tests ********** ////////////////////
    //////////////////////////////////////////////////////////////////////////////

    //////////////////////////////////////////////////////////////////////////////
    ///////////////////// ********** Image Tests ********** //////////////////////
    //////////////////////////////////////////////////////////////////////////////
    #[tokio::test]
    async fn test_add_image_handler()
    {
        let pool = setup();

        let test_user = TestUser::create(&pool).await.unwrap();
        let session_id = create_test_session(&pool, test_user.user_id).await.unwrap(); // create a test session
                                                                                       // Setup test data
        let file_path = "./tests/images/testImage.png";
        let result = add_image(&pool, session_id, file_path).await;

        assert!(result.is_ok());
        test_user.cleanup(&pool).await.unwrap();
    }
}
