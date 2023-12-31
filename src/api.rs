// web api endpoints
// POST /api/transform/rotate for a rotate tool.
// POST /api/filter/blur for applying a blur filter.
// use crate::db::{ add_user, create_pool };
use crate::db;
use crate::db::*; // QUEST: Am I exposing all db.rs funcs by doing this?
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Pool;
use serde::Deserialize;

//////////////////////////////////////////////////////////////////////////////////
///////////////// ******* Function to start the server ******* ///////////////////
//////////////////////////////////////////////////////////////////////////////////
pub async fn start_server(pool: Pool) -> Result<(), MyError> {

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index ))
            .route("/add_user", web::post().to(add_user_handler))
            .route("/user/{username}", web::get().to(get_user_handler))
            .route("/user/{username}", web::delete().to(delete_user_handler))
        // Other routes
    })
    .bind("127.0.0.1:8080")? // TODO: Does this need to change in PROD?
    .run()
    .await?;

    Ok(())
}
/// Index handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body( "Welcome to the API!" )
}

// Error Enum for server function ////////////////////////////////////////////////
#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Postgres(postgres::Error),
    // Other error types as needed HERE
}
impl From<std::io::Error> for MyError {
    fn from(err: std::io::Error) -> MyError {
        MyError::Io(err)
    }
}
impl From<postgres::Error> for MyError {
    fn from(err: postgres::Error) -> MyError {
        MyError::Postgres(err)
    }
}

//////////////////////////////////////////////////////////////////////////////////
////////////// ***** User Route Handler Functions ***** //////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Route handler to add a new user //////////////////////////////////////////////
/// This allows new users to register.
async fn add_user_handler(pool: web::Data<Pool>, new_user: web::Json<NewUser>) -> HttpResponse {
    match add_user(&pool, &new_user.username, &new_user.email).await {
        Ok(_) => HttpResponse::Ok().json("User added successfully"),
        Err(MyDbError::PostgresError(e)) => {
            HttpResponse::InternalServerError().json(format!("Postgres error: {}", e))
        }
        Err(MyDbError::PoolError(e)) => {
            HttpResponse::InternalServerError().json(format!("Pool error: {}", e))
        }
        Err(_) => HttpResponse::InternalServerError().json("Unhandled error"),
        // Handle other errors
    }
}
#[derive(Debug, Deserialize)] // QUEST: What does the Debug macro allow you to do?
struct NewUser {
    username: String,
    email: String,
}

/// Route handler to get a user by username //////////////////////////////////////
/// This allows users to get their user profile
async fn get_user_handler(pool: web::Data<Pool>, path: web::Path<(String,)>) -> HttpResponse {
    let username = &path.into_inner().0;

    match pool.get().await {
        Ok(mut client) => match db::get_user_by_username(&mut client, username).await {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(MyDbError::NotFound) => HttpResponse::NotFound().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Delete user by username //////////////////////////////////////
/// This is for user account deletion.
async fn delete_user_handler(pool: web::Data<Pool>, path: web::Path<(String,)>) -> HttpResponse {
    let username = &path.into_inner().0;

    match db::delete_user(&pool, username).await {
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
async fn update_user_email_handler(
    pool: web::Data<Pool>,
    path: web::Path<(String)>,
    new_email: String,
) -> HttpResponse {
    let username = path.into_inner();

    match db::update_user_email(&pool, &username, &new_email).await {
        Ok(_) => HttpResponse::Ok().json("User email changed successfully"),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("User not found"),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),
    }
}

//////////////////////////////////////////////////////////////////////////////////
////////////// *****  Session Route Handler Functions ***** //////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Create_session
/// To start a new user session upon login

/// End_session 
/// To allow users to log out.

/// Get_active_sessions 
/// To retrieve active sessions, useful for administrative purposes. 


//////////////////////////////////////////////////////////////////////////////////
////////////// *****  Image Route Handler Functions ***** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Add image
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
async fn add_image_handler(
    pool: web::Data<Pool>,
    path: web::Path< String >, // QUEST: is this parameter necessary? 
    image_path: String,
    user_id: i32,
) -> HttpResponse {

    // let username: &str = &path.into_inner();

    // Fetch session ID
    let session_id = match db::get_session_id_for_user( &pool, user_id ).await {
        Ok( id ) => id, 
        Err( _ ) => return HttpResponse::InternalServerError().json( "Error fetching session ID" ),
    };

    // Call the associated function from db.rs
    match db::add_image( &pool, session_id, &image_path ).await {

        Ok( _ ) => HttpResponse::Ok().json( "Image added successfully" ),
        Err( MyDbError::NotFound ) => HttpResponse::NotFound().json( "User not found" ),
        Err( _ ) => HttpResponse::InternalServerError().json( "Internal server error" ),
    }
}

// Get_image
// Update image
// Delete image

//////////////////////////////////////////////////////////////////////////////////
////////////// *****  Layer Route Handler Functions ***** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////

// Add Layer
// Get layer by layer_id
// Update layer
// Delete layer
