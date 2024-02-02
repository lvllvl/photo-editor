use crate::db;
use actix_web::{web, HttpResponse, Error };
// use actix_web::{web, App, http, HttpResponse, HttpServer, Responder, test};
// use deadpool_postgres::{Config, Pool};
use deadpool_postgres::Pool;
// use actix_multipart::Multipart;
// use futures::{StreamExt, TryStreamExt};// for file streaming
// use std::io::Write; // for file writing
// use serde::Deserialize;
// use serde_json::json;
// use tokio_postgres::{Error, NoTls, Row};
use super::MyDbError;
// use actix_web::web::Data;
use rand::Rng;
// use uuid::Uuid;

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
// async fn add_image_handler( pool: web::Data<Pool>, file_path: &str, file_type: &str ) -> HttpResponse {

//     let user_id: i32 = 1; // FIXME: Assuming user_id is extracted from authenticated session
//     match db::images::add_image( &pool, file_path, user_id, file_type ).await {

//         Ok( image_id ) => HttpResponse::Ok().json( image_id ),
//         Err( MyDbError::NotFound ) => HttpResponse::NotFound().json( "Image was not uploaded." ),
//         Err( _ ) => HttpResponse::InternalServerError().json( "Internal server error" ),
//     }
// }
async fn add_image_handler( 
    pool: web::Data<Pool>,
    file_path: &str,
    file_type: &str,
) -> HttpResponse
{
    let mut rng = rand::thread_rng();
    let user_id: i32 = rng.gen(); // FIXME: Assuming user_id is extracted from authenticated session

    match db::images::add_image( &pool, file_path, user_id, file_type ).await {

        Ok( image_id ) => {
            HttpResponse::Ok().json( format!( "message: Image was uploaded successfully. Image ID: {}", image_id ))
        },
        Err( e ) => {
            println!( "Error adding image: {:?}", e );
            HttpResponse::InternalServerError().json( "Internal server error" )
        },
    }

}

// TODO: Get_image
// QUEST: should this return a vector instead of HttpResonse?
async fn get_single_image_handler(pool: web::Data<Pool>, image_id: i32) -> HttpResponse
{
    match db::images::get_single_image(&pool, image_id).await
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

    match db::images::get_all_images(&pool, user_id).await
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
                            //   _path: web::Path<String>,
                              image_id: i32,
                              new_image_path: String)
                              -> HttpResponse
{
    match db::images::update_image(&pool, image_id, &new_image_path).await
    {
        Ok(_) => HttpResponse::Ok().json("Image path has been updated."),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("Image NOT found."),
        Err(_) => HttpResponse::InternalServerError().json("Internal server error! Oh no."),
    }
}

/// Delete image: Take an image within the database and delete it.
async fn delete_image_handler(pool: web::Data<Pool>, image_id: web::Path<i32>) -> HttpResponse
{
    let image_id = image_id.into_inner();
    // Add authorization and validation logic stuff here

    match db::images::delete_image(&pool, image_id).await
    {
        Ok(_) => HttpResponse::Ok().json(format!("Image with ID {} was deleted succesfully!",
                                                 image_id.to_string())),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("Image NOT found!"),
        Err(_) => HttpResponse::InternalServerError().json("Internal Server Error!"),
    }
}
