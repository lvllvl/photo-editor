// web api endpoints 
// POST /api/transform/rotate for a rotate tool.
// POST /api/filter/blur for applying a blur filter.
// use crate::db::{ add_user, create_pool };
use actix_web::{ web, App, HttpServer, Responder, HttpResponse };
use deadpool_postgres::Pool;
use serde::Deserialize;

use crate::db;
use crate::db::{ add_user, MyDbError };
 
#[derive(Debug)]
pub enum MyError { 
    Io( std::io::Error ), 
    Postgres( postgres::Error ),
    // Other error types as needed HERE 
}

/////////////////////////////////////////////////////////////////////////////////////
impl From< std::io::Error > for MyError {
    fn from( err: std::io::Error ) -> MyError {
        MyError::Io( err )
    }
}

impl From< postgres::Error > for MyError {
    fn from( err: postgres::Error ) -> MyError {
        MyError::Postgres( err )
    }
}
/////////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize)]
struct NewUser {
    username: String, 
    email: String,
} 

// Route handler to add a new user
async fn add_user_handler(pool: web::Data<Pool>, new_user: web::Json<NewUser>) -> HttpResponse {

    match add_user(&pool, &new_user.username, &new_user.email).await {

        Ok(_) => HttpResponse::Ok().json("User added successfully"),
        Err(MyDbError::PostgresError(e)) => HttpResponse::InternalServerError().json(format!("Postgres error: {}", e)),
        Err(MyDbError::PoolError(e)) => HttpResponse::InternalServerError().json(format!("Pool error: {}", e)),
        Err(_) => HttpResponse::InternalServerError().json("Unhandled error"),
        
        // Handle other errors
    }

}


// Define route handler functions ///////////////////////////////////////////////// 
async fn index() -> impl Responder { 
    "Hello world!" 

}

// Route handler to get a user by username /////////////////////////////////////////
async fn get_user_handler( pool: web::Data< Pool >, path: web::Path<( String, )> ) -> HttpResponse {

    let username = &path.into_inner().0;

    match pool.get().await {
        Ok( mut client ) => {
            
            match db::get_user_by_username( &mut client, username ).await {
                
                Ok( user ) => HttpResponse::Ok().json( user ),
                Err( MyDbError::NotFound ) => HttpResponse::NotFound().finish(),
                Err( _ ) => HttpResponse::InternalServerError().finish(),

            }

        }
        Err( _ ) => HttpResponse::InternalServerError().finish(),

    }

}

// Route handler to greet Mank ////////////////////////////////////////////////////
async fn greet_mank() -> impl Responder { 
    "Hello Mank!" 
}
/////////////////////////// Define more routes here ///////////////////////////////
// Route handler to delete user by username ///////////////////////////////////////
async fn delete_user_handler( pool: web::Data< Pool >, path: web::Path<( String, )> ) -> HttpResponse {

    let username = &path.into_inner().0;

    match db::delete_user( &pool, username ).await {

        Ok( _ ) => HttpResponse::Ok().json( "User deleted successfully" ),
        Err( MyDbError::NotFound ) => HttpResponse::NotFound().json( "User not found" ),
        Err( _ ) => HttpResponse::InternalServerError().json( "Internal server error" ),

    }

}
// Route handler to get a user by email ///////////////////////////////////////////

// Function to start the server ///////////////////////////////////////////////////

pub async fn start_server( pool: Pool ) -> Result< (), MyError > {
    HttpServer::new( move || {
        App::new()
            .app_data( web::Data::new( pool.clone()))
            .route( "/", web::get().to( index ))
            .route( "/add_user", web::post().to( add_user_handler ))
            .route( "/user/{username}", web::get().to( get_user_handler ))
            .route( "/user/{username}", web::delete().to( delete_user_handler ))
            .route( "/mank", web::get().to( greet_mank ))
            // Other routes

    })
    .bind( "127.0.0.1:8080" )?
    .run()
    .await?;

    Ok(())
}


// #[post("/upload" )]
// fn upload_image() -> Result<HttpResponse, Error> {

    // Determine if this is a new image or a new layer for an existing image.
    // Process the image 
    // save the image metadata to the database 
    // Return a response with the image or layer identifier 
// }