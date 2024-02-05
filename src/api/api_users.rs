use crate::db;
use actix_web::{web, HttpResponse};
// use actix_web::{web, App, http, HttpResponse, HttpServer, Responder, test};
use deadpool_postgres::Pool;
// use deadpool_postgres::{Config, Pool};
use serde::Deserialize;
use serde_json::json;
// use tokio_postgres::{Error, NoTls, Row};

use super::MyDbError;
//////////////////////////////////////////////////////////////////////////////////
////////////// ***** User Route Handler Functions ***** //////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Route handler to add a new user //////////////////////////////////////////////
/// This allows new users to register.
/// 
/// # Arguements
/// * 'pool' - A reference to the database connection pool.
/// * 'new_user' - A web::Json tuple containing the new user data.
/// 
/// # Returns
/// 
/// Return an HttpResponse indicating the outcome of the operation.
/// 
/// # Example Request
/// 
/// POST /add_user
/// Body: { "username": "new_user", "email": "sample@email.com"}
/// 
/// # Example Response
/// 
/// "User added successfully with ID: 1"
pub async fn add_user_handler(pool: web::Data<Pool>, new_user: web::Json<NewUser>) -> HttpResponse
{
    match db::users::add_user(&pool, &new_user.username, &new_user.email).await
    {
        Ok( user_id) => HttpResponse::Ok().json( format!("User added successfully with ID: {}", user_id )),
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
#[derive(Debug, Deserialize)] 
pub struct NewUser
{
    username: String,
    email: String,
}

/// Get a User by providing a username ///////////////////////////////////////////
/// This is a GET request to get a user by a username. 
/// This allows users to get their user profile
///
/// # Arguements
///
/// * 'pool' - A reference to the database connection pool.
/// * 'path' - A web::Path tuple.
///
/// # Returns
///
/// Return the user data if successful.
///
/// # Example Request
///
/// GET /users/{username}
//////////////////////////////////////////////////////////////////////////////////
pub async fn get_user_handler(pool: web::Data<Pool>, path: web::Path<(String,)>) -> HttpResponse
{
    let username = &path.into_inner().0;
    match db::users::get_user_by_username(&pool, username).await
    {
        // Ok(user) => HttpResponse::Ok().json(user),
        Ok(user) => HttpResponse::Ok().body(format!("This is the requested user: {}", user)),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json(format!("User {} not found", username)),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),
    }
}

/// Get all users //////////////////////////////////////////////////////////////
/// This is for administrative purposes. It retrieves all users in the database.
/// 
/// # Arguements
/// 
/// * 'pool' - A reference to the database connection pool.
/// 
/// # Returns
/// 
/// Return a JSON response containing all users.
/// 
/// # Example Request
/// 
/// GET /users
//////////////////////////////////////////////////////////////////////////////////
pub async fn get_all_users_handler(pool: web::Data<Pool>) -> HttpResponse
{
    match db::users::get_all_users(&pool).await
    {
        Ok(users) => {
            let response = json!({
                "status": "success", 
                "total_users": users.len(),
                "users": users
            });
            HttpResponse::Ok().json( response)
        }
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),
    }
}

/// Get user by email
/// 
/// # Arguements
/// 
/// * 'pool' - A reference to the database connection pool.
/// * 'email' - A web::Path tuple containing the email.
/// 
/// # Returns
/// 
/// Return the user data if successful.
/// 
/// # Example Request
/// 
/// GET /get_user_by_email/{email}/
pub async fn get_user_by_email_handler(pool: web::Data<Pool>, email: web::Path<(String,)>) -> HttpResponse
{
    let email = &email.into_inner().0;
    match db::users::get_user_by_email(&pool, email).await
    {
        // Ok(user) => HttpResponse::Ok().json(user),
        // Ok(user) => HttpResponse::Ok().json(user),
        Ok(user) => HttpResponse::Ok().body(format!("This is the requested user: {}", user)),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("User not found"),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),
    }
}

/// Get user by user ID
pub async fn get_user_by_user_id_handler( pool: web::Data<Pool>, user_id: web::Path< i32 > ) -> HttpResponse
{
    let user_id = user_id.into_inner();

    // Call the get_user_by_id function from db::users
    match db::users::get_user_by_id( &pool, user_id ).await {

        Ok(user) => HttpResponse::Ok().json(user),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("User not found"),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),

    }
}

/// Delete user by username //////////////////////////////////////
/// This is for user account deletion.
/// 
/// # Arguements
/// 
///     * 'pool' - A reference to the database connection pool.
///    * 'path' - A web::Path tuple containing the username.
/// 
/// # Returns
/// 
/// Return an HttpResponse indicating the outcome of the operation.
/// 
/// # Example Request
/// 
/// DELETE /delete_user/{username}
/// curl -X DELETE http://localhost:8080/delete_user/{username}
pub async fn delete_user_handler(pool: web::Data<Pool>, path: web::Path<(String,)>) -> HttpResponse
{
    let username = &path.into_inner().0;

    match db::users::delete_user(&pool, username).await
    {
        Ok(_) => HttpResponse::Ok().json( format!("User deleted successfully: {}", username )),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json(format!("User: {}, not found", username)),
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
pub async fn update_user_email_handler(pool: web::Data<Pool>,
                                   path: web::Path<String>,
                                   new_email: String)
                                   -> HttpResponse
{
    let username = path.into_inner();

    match db::users::update_user_email(&pool, &username, &new_email).await
    {
        Ok(_) => HttpResponse::Ok().json(format!("User: {} email changed successfully", username)),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("User not found"),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error"),
    }
}

/// Delete all users ////////////////////////////////////////////////////////////
pub async fn delete_all_users_handler( pool: web::Data<Pool> ) -> HttpResponse
{
    match db::users::delete_all_users( &pool ).await
    {
        Ok(_) => HttpResponse::Ok().json( "All users deleted successfully" ),
        Err(_) => HttpResponse::InternalServerError().json( "Internal server error" ),
    }
}