pub mod users;
pub mod images;
// pub mod sessions;
// pub mod layers;

// use crate::db;
use crate::db::*;

use actix_web::{web, App, http, HttpResponse, HttpServer, Responder, test};
use deadpool_postgres::{Config, Pool};
use serde::Deserialize;
use serde_json::json;
use tokio_postgres::{Error, NoTls, Row};

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
                  .route("/add_user", web::post().to(users::add_user_handler))
                  .route("get_user_by_username/{username}", web::get().to(users::get_user_handler))
                  .route( "/get_user_by_email/{email}", web::get().to(users::get_user_by_email_handler))
                  .route("/users", web::get().to(users::get_all_users_handler)) // TODO: remove this in PROD
                  .route("/user/{username}/update_email", web::put().to( users::update_user_email_handler ))
                  .route( "/delete_user/{username}", web::delete().to( users::delete_user_handler )) 
                
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
