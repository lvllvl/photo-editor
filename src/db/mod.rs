#![allow(dead_code)]
pub mod users;
pub mod images;
pub mod sessions;
pub mod layers;
// ... other module declarations ...


use chrono::{DateTime, Duration, Utc};
use deadpool_postgres::{Config, Pool};
// use postgres::types::ToSql;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tokio_postgres::{Error, NoTls, Row};
use std::fmt;

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** DB Connection Management ********** //////////////////////
//////////////////////////////////////////////////////////////////////////////////

// Manage database connections ///////////////////////////////////////////////////
// setup a pool of connections to the database ///////////////////////////////////
pub fn create_pool() -> Pool {
    
    let mut cfg = Config::new();
    // Set configuration details... Retrieve from .env file
    cfg.host = std::env::var("DB_HOST").ok();
    cfg.user = std::env::var("DB_USER").ok();
    cfg.password = std::env::var("DB_PASSWORD").ok();
    cfg.dbname = std::env::var("DB_NAME").ok();  // Make sure this line is correctly retrieving the DB name

    cfg.create_pool(None, NoTls).expect("Failed to create pool")
}

//////////// ********** Setup Database Schema ********** /////////////////////////
pub async fn setup_database(client: &mut deadpool_postgres::Client) -> Result<(), Error> {

    // Create User Table ////////////////////////////////////////////////////////
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
        )
    ",
        )
        .await?;
    println!("Users table created successfully.");

    // Create Session Table //////////////////////////////////////////////////////
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS sessions (
            id              SERIAL PRIMARY KEY,
            user_id         INTEGER REFERENCES users(id),
            creation_time   TIMESTAMP NOT NULL,
            expiration_time TIMESTAMP NOT NULL,
            last_activity   TIMESTAMP NOT NULL,
            session_data    JSONB
        )
    ",
        )
        .await?;
    println!("Sessions table created successfully.");

    // // Create Image Table ///////////////////////////////////////////////////////
    // client
    //     .batch_execute(
    //         "
    //     CREATE TABLE IF NOT EXISTS images (
    //         id              SERIAL PRIMARY KEY,
    //         session_id      INTEGER REFERENCES sessions,
    //         file_path       VARCHAR NOT NULL,
    //         created_at      TIMESTAMP NOT NULL,
    //         updated_at      TIMESTAMP NOT NULL
    //     )
    // ",
    //     )
    //     .await?;
    // println!("images table created successfully.");

    // // Create Layers Table //////////////////////////////////////////////////////
    // client
    //     .batch_execute(
    //         "
    //     CREATE TABLE IF NOT EXISTS layers (
    //         id              SERIAL PRIMARY KEY,
    //         image_id        INTEGER REFERENCES images,
    //         layer_name      VARCHAR( 255 ),
    //         creation_date   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    //         last_modified   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    //         user_id         INTEGER REFERENCES users,
    //         layer_type      VARCHAR( 50 ),
    //         visibility      BOOLEAN DEFAULT TRUE,
    //         opacity         FLOAT DEFAULT 100,
    //         layer_data      BYTEA,
    //         layer_order     INTEGER
    //     );  
    // ",
    //     )
    //     .await?;
    // println!("layers table created successfully.");

    Ok(())
}
// TODO: move to sessions.rs
// // end_session: for an individual  ///////////////////////////////////////////////
// pub async fn end_session(pool: &Pool, user_id: i32) -> Result<(), MyDbError> {
//     // Fetch a database connection from the pool
//     let client = pool.get().await?;

//     // Prep the SQL query to update the session
//     let statement = client
//         .prepare("UPDATE session SET end_time = NOW() WHERE id = $1")
//         .await?;

//     let session_id = get_session_id_for_user(&pool, user_id).await?;

//     // Execute the query
//     let result = client.execute(&statement, &[&session_id]).await?;

//     // Check if any rows were affected
//     if result == 0 {
//         // No rows were updated, session not found or already ended
//         Err(MyDbError::NotFound)
//     } else {
//         // Session succesfully ended
//         Ok(())
//     }
// }
//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Error Handling ********** ////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum MyDbError {
    PostgresError(postgres::Error),
    PoolError(deadpool::managed::PoolError<postgres::Error>),
    NotFound,
    JsonError(String),
    // ... other error types as needed
}

impl From<serde_json::Error> for MyDbError {
    fn from(err: serde_json::Error) -> MyDbError {
        MyDbError::JsonError(err.to_string())
    }
}

impl From<postgres::Error> for MyDbError {
    fn from(err: postgres::Error) -> MyDbError {
        MyDbError::PostgresError(err)
    }
}

impl From<deadpool::managed::PoolError<postgres::Error>> for MyDbError {
    fn from(err: deadpool::managed::PoolError<postgres::Error>) -> MyDbError {
        MyDbError::PoolError(err)
    }

}

impl std::error::Error for MyDbError {}
impl fmt::Display for MyDbError {
    fn fmt( &self, f: &mut fmt::Formatter< '_> ) -> fmt::Result {
        write!( f, "Database error: {:?}", self )
    }
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Analytics & Reports ********** ///////////////////////////
//////////////////////////////////////////////////////////////////////////////////

// TODO: user_activity_report: generate reports on user activity ///////////////////////
// TODO: image_statistics: get statistics on image uploads, edits, etc. ////////////////

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** DB Health & Maintenance********** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////

// TODO: check_db_health: check database health ////////////////////////////////////////
// TODO: backup_db: backup database ////////////////////////////////////////////////////
// TODO: restore_db: restore database //////////////////////////////////////////////////
// TODO: delete_db: delete database ////////////////////////////////////////////////////
// TODO: clean_db: clean database, optimize, etc. //////////////////////////////////////