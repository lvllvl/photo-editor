pub mod api_users;
pub mod api_images;
// pub mod api_sessions;
// pub mod api_layers;

// use crate::db;
use crate::db::*;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use actix_web::{web, App, http, HttpResponse, HttpServer, Responder, test};
// use deadpool_postgres::{Config, Pool};
use deadpool_postgres::Pool;
// use serde::Deserialize;
// use serde_json::json;
// use tokio_postgres::{Error, NoTls, Row};

// use crate::db::users::*;
// use crate::db::sessions;

// TODO: include all the following endpoints
// web api endpoints
// POST /api/transform/rotate for a rotate tool.
// POST /api/filter/blur for applying a blur filter.
// use crate::db::{ add_user, create_pool };

//////////////////////////////////////////////////////////////////////////////////
///////////////// ******* Function to start the server ******* ///////////////////
//////////////////////////////////////////////////////////////////////////////////
pub async fn start_server(pool: Pool) -> Result<(), MyError>
{
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone()))
                  .route("/", web::get().to(index))
                  .route("/user/add_user", web::post().to(api_users::add_user_handler))
                  .route( "/user/get_user_by_id/{id}", web::get().to( api_users::get_user_by_user_id_handler )) // TODO: remove this in PROD
                  .route("/user/get_user_by_username/{username}", web::get().to(api_users::get_user_handler))
                  .route( "/user/get_user_by_email/{email}", web::get().to(api_users::get_user_by_email_handler))
                  .route("/user/all_users", web::get().to(api_users::get_all_users_handler)) // TODO: remove this in PROD
                  .route("/user/{username}/update_email", web::put().to( api_users::update_user_email_handler ))
                  .route( "/user/delete_user/{username}", web::delete().to( api_users::delete_user_handler )) 
                  .route("/user/delete_all_users", web::delete().to(api_users::delete_all_users_handler)) // TODO: remove this in PROD
                  .route( "/image/add_image", web::post().to(api_images::add_image_handler))
                
                // Other routes
    // TODO: Does this number/address need to change in PROD?
    }).bind("127.0.0.1:8080")? 
      .run()
      .await?;

    Ok(())
}
/// Index handler
async fn index() -> impl Responder
{
    HttpResponse::Ok().body("Welcome to the API!")
}

// ///////////////////////////////////////////////////////////////////////////////////
// /// TODO: figure out what to do with this, if it's needed
// pub fn configure_api(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::resource("/add_user")
//             .route(web::post().to(users::add_user_handler))
//         // ... other routes
//     );
// }

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
