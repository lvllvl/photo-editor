use crate::db;
use crate::db::*; 
use actix_web::{web, App, http, HttpResponse, HttpServer, Responder, test};
use deadpool_postgres::{Config, Pool};
use serde::Deserialize;
use serde_json::json;
use tokio_postgres::{Error, NoTls, Row};
use super::MyDbError;
//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
////////////// *****  Session Route Handler Functions ***** //////////////////////
//////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Create_session
/// To start a new user session upon login
pub async fn create_a_session_handler( pool: web::Data<Pool>, username: &str ) -> HttpResponse
{
    match db::sessions::create_a_session( &pool, username ).await {
        Ok( session ) => HttpResponse::Ok().json( session ),
        Err( e ) => {
            println!( "Error creating session: {:?}", e );
            HttpResponse::InternalServerError().json( "Internal server error" )
        }

    }
}


// pub async fn create_a_session( pool: &Pool, user_id: i32, session_data: serde_json::Value ) -> Result< Session, MyDbError > {

//     let client = pool.get().await.map_err(MyDbError::PoolError)?; // .map_err(MyDbError::PoolError)?; // .await?;
//     let statement = client
//         .prepare( "INSERT INTO sessions (user_id, creation_time, expiration_time, last_activity, session_data) VALUES ($1, $2, $3, $4, $5 ) RETURNING *" )
//         .await.map_err(MyDbError::QueryError)?;

//     let session_data_str = serde_json::to_string( &session_data ).map_err( MyDbError::SerializeError )?; 
//     let expiration_time = calculate_expiration_time();

//     // Execute prepared statment 
//     match client.query_one( &statement, &[&user_id, &expiration_time, &session_data_str ] ).await {
//         Ok( row ) => {
//             // Session is a struct, that represents a single session
//             let session = Session::from_row( &row )?;
//             Ok( session )
//         },
//         Err( e ) => Err( MyDbError::QueryError( e )),
//     }
// }


/// End_session
/// To allow users to log out OR just end the session after a certain amount of
/// time OR end session after a certain amount of inactivity.

/// Get_active_sessions
/// To retrieve active sessions, useful for administrative purposes.




// /// Route handler to add a new user //////////////////////////////////////////////
// /// This allows new users to register.
// /// 
// /// # Arguements
// /// * 'pool' - A reference to the database connection pool.
// /// * 'new_user' - A web::Json tuple containing the new user data.
// /// 
// /// # Returns
// /// 
// /// Return an HttpResponse indicating the outcome of the operation.
// /// 
// /// # Example Request
// /// 
// /// POST /add_user
// /// Body: { "username": "new_user", "email": "sample@email.com"}
// /// 
// /// # Example Response
// /// 
// /// "User added successfully with ID: 1"
// pub async fn add_user_handler(pool: web::Data<Pool>, new_user: web::Json<NewUser>) -> HttpResponse
// {
//     match db::users::add_user(&pool, &new_user.username, &new_user.email).await
//     {
//         Ok( user_id) => HttpResponse::Ok().json( format!("User added successfully with ID: {}", user_id )),
//         Err(MyDbError::PostgresError(e)) =>
//         {
//             HttpResponse::InternalServerError().json(format!("Postgres error: {}", e))
//         }
//         Err(MyDbError::PoolError(e)) =>
//         {
//             HttpResponse::InternalServerError().json(format!("Pool error: {}", e))
//         }
//         Err(_) => HttpResponse::InternalServerError().json("Unhandled error"),
//         // Handle other errors
//     }
// }