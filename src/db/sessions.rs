#![allow(dead_code)]
use super::MyDbError;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use crate::db::users::User;
use crate::db::users::get_user_by_id; 

/// Create a single session for a user ////////////////////////////////////////////
/// 
/// This treats all the users the same, just creates a session for a user
/// 
pub async fn create_a_session( pool: &Pool, user_id: i32, session_data: serde_json::Value ) -> Result< Session, MyDbError > 
{
    // Check if user_id exists in the users table
    match get_user_by_id( pool, user_id ).await {
        Ok( _user ) => {
            let client = pool.get().await.map_err(MyDbError::PoolError)?; // .map_err(MyDbError::PoolError)?; // .await?;
            let statement = client
                .prepare( "INSERT INTO sessions (user_id, creation_time, expiration_time, last_activity, session_data) VALUES ($1, $2, $3, $4, $5 ) RETURNING *" )
                .await.map_err(MyDbError::QueryError)?;

            let session_data_str = serde_json::to_string( &session_data ).map_err( MyDbError::SerializeError )?; 
            let expiration_time = calculate_expiration_time();

            // Execute prepared statment 
            match client.query_one( &statement, &[&user_id, &expiration_time, &session_data_str ] ).await {
                Ok( row ) => {
                    // Session is a struct, that represents a single session
                    let session = Session::from_row( &row )?;
                    Ok( session )
                },
                Err( e ) => Err( MyDbError::QueryError( e )),
            }
        },
        Err( MyDbError::NotFound ) => Err( MyDbError::NotFound ),
        Err( e ) => Err( e )
    }

}

/// Get user info from a session_id, returns a user struct
pub async fn get_user_from_session_id( pool: &Pool, session_id: i32 ) -> Result< User, MyDbError > {
    
    let client = pool.get().await?;

    // This should return a single row, since session_id is unique
    let statement = client.prepare( "SELECT user_id FROM sessions WHERE id = $1" ).await?;
    let rows = client.query( &statement,  &[ &session_id ] ).await?;

    // TODO: Get the user_id ( i32 ), then get the user info from the users table
    if let Some( row ) = rows.into_iter().next() {
        // Extract the user_id from the row
        let user_id: i32 = row.get::<&str, i32>( "user_id" );

        // Get user info from users table with user_id
        let user = get_user_by_id( pool, user_id ).await?;
        Ok( user )
    } else {
        Err( MyDbError::NotFound )
    }
}

// get_active_sessions: for all current users ////////////////////////////////////
pub async fn get_active_sessions(pool: &Pool) -> Result<Vec<Session>, MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("SELECT * FROM sessions WHERE end_time IS NULL")
        .await?;
    let rows = client.query(&statement, &[]).await?;
    let mut sessions = Vec::new();

    for row in rows {
        // Session is a struct, that represents a single session
        sessions.push(Session::from_row(&row)?);
    }

    if sessions.is_empty() {
        Err(MyDbError::NotFound)
    } else {
        Ok(sessions)
    }
}

// get_session_ID for a SINGLE user //////////////////////////////////////////////
pub async fn get_session_id_for_user(pool: &Pool, user_id: i32) -> Result<i32, MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("SELECT id FROM sessions WHERE user_id = $1")
        .await?;

    let rows = client.query(&statement, &[&user_id]).await?;
    if let Some(row) = rows.into_iter().next() {
        Ok(row.get("id"))
    } else {
        Err(MyDbError::NotFound)
    }
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Helper Functions ********** //////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
fn calculate_expiration_time() -> SystemTime{ 
    // TODO: update the time for expiration
    SystemTime::now() + std::time::Duration::new( 86_400, 0 ) // 86_400 seconds in a day
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Session Representation ********** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: i32,
    pub user_id: i32,
    pub creation_time: DateTime<Utc>,
    pub expiration_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub session_data: serde_json::Value,
}

impl Session {
    // Function to create a session instance from a database row
    pub fn from_row(row: &Row) -> Result<Session, MyDbError> {
        let session_data_str: String = row.get("session_data");
        let session_data = serde_json::from_str(&session_data_str)
            .map_err(|e| MyDbError::JsonError(e.to_string()))?;

        Ok(Session {
            id: row.get("id"),
            user_id: row.get("user_id"),
            creation_time: row.get("creation_time"),
            expiration_time: row.get("expiration_time"),
            last_activity: row.get("last_activity"),
            session_data,
        })
    }
}
