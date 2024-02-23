use crate::db;
use actix_web::{ App,
                 HttpResponse,
                 HttpRequest,
                 Error,
                 web,
                 post,
                 get,
                 http::header::CONTENT_LENGTH
                    }; 
use actix_multipart::Multipart;
use deadpool_postgres::Pool;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use serde::Serialize;
use super::MyDbError;
use rand::Rng;
use actix_web::http::header::ContentDisposition;

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
/// * 'file_path' - The path to the image file.
/// * 'file_type' - The type of the image file.
///
/// # Returns
/// 
/// Return an HttpResponse indicating the outcome of the operation.
///
/// # Example Request
/// 
/// 
///
#[derive(Serialize)]
struct ImageUploadResponse {
    message: String,
    image_id: i32,
    image_url: String,
}

#[post("/image/add_image")]
pub async fn add_image_handler(
    pool: web::Data<Pool>,
    mut payload: Multipart,
    req: HttpRequest,
) -> HttpResponse {

    let content_length: usize = match req.headers().get( CONTENT_LENGTH ){
        Some( hv) => hv.to_str()unwrap_or("0" ).parse().unwrap(); 
        None => 0, 
    }; 

    if content_length == 0 || content_length > max_file_size { return HttpResponse::BadRequest().into(); } 
    
    

    let mut rng = rand::thread_rng();
    let user_id: i32 = rng.gen(); // FIXME: This should be the user's ID, extracted from the session

    // Init variables to hold file details
    let mut saved_file_path = String::new();
    let mut file_type = String::new();

    // Process the multipart payload
    while let Ok( Some( mut field ) ) = payload.try_next().await{

        if let Some( content_disposition ) = field.content_disposition() {
            
            let filename = content_disposition.get_filename().unwrap_or( "unnamed" ).to_string();
            let filepath = format!( "./uploads/{}", sanitize_filename( &filename ));
            saved_file_path = filepath.clone(); // Store file for DB entry
            file_type = content_disposition.get_param( "Content-Type" ).map( |c| c.as_str()).unwrap_or( "unknown" ).to_string();

            let mut file = match web::block( || std::fs::File::create( &filepath )).await {
                Ok( file ) => file,
                Err( e ) => return HttpResponse::InternalServerError().finish(),
            };

            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok( data ) => data,
                    Err(_) => return HttpResponse::InternalServerError().finish(),
                };

                if let Err(_) = web::block( move || file.write_all( &data )).await {
                    return HttpResponse::InternalServerError().finish();
                }
            }
            // while let Some( chunk ) = field.next().await {
            //     let data = chunk.map_err( |_| HttpResponse::InternalServerError())?;
            //     web::block( move || file.write_all( &data ).map( |_| ())).await.map_err(|_| HttpResponse::InternalServerError())?;
            // }
        } else {
            eprintln!( "No content disposition found in the field"); 
        }

    }
    // Save the image to the database
    match db::images::add_image(&pool, &saved_file_path, user_id, &file_type ).await {

        Ok( image_id ) => {
            let response = ImageUploadResponse {
                message: "Image has been uploaded successfully.".to_string(),
                image_id,
                image_url: format!( "http://yourserver.com/path/to/images/{}", saved_file_path ),
            };
            HttpResponse::Ok().json( response )
            },
            Err( e ) => {
                println!( "Error adding image: {:?}", e ); 
                HttpResponse::InternalServerError().json( "Internal server error")
            }
        }
}

// Sanitize filename
fn sanitize_filename(filename: &str) -> String {

    let invalid_chars = ['/', '\\', '?', '%', '*', ':', '|', '"', '<', '>', '.'];
    let sanitized: String = filename.chars().filter(|c| !invalid_chars.contains(c)).collect();
    
    // Prevent empty filenames or use a default name
    if sanitized.trim().is_empty() {
        "unnamed".to_string()
    } else {
        sanitized
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
