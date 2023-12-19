// web api endpoints 

// POST /api/transform/rotate for a rotate tool.
// POST /api/filter/blur for applying a blur filter.
// use crate::db::{ add_user, create_pool };
use crate::db::{ add_user };
use actix_web::{ web, App, HttpServer, Responder };
use deadpool_postgres::Pool;
 

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

// Define route handler functions ///////////////////////////////////////////////// 
async fn index() -> impl Responder { 
    "Hello world!" 

}

async fn greet_mank() -> impl Responder { 
    "Hello Mank!" 
}
/////////////////////////// Define more routes here ///////////////////////////////

// Function to start the server ///////////////////////////////////////////////////

pub async fn start_server( pool: Pool ) -> Result< (), MyError > {
    HttpServer::new( move || {
        App::new()
            .data( pool.clone() ) // Pass the pool to the applicaiton
            .route( "/", web::get().to( index ))
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