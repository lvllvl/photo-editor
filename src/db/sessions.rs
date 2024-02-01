#![allow(dead_code)]
use super::MyDbError;
// use chrono::{DateTime, Duration, Utc};
use chrono::{DateTime, Utc};
// use deadpool_postgres::{Config, Pool};
use deadpool_postgres::Pool;
// use postgres::types::ToSql;
use serde::{Deserialize, Serialize};
// use serde_json::json;
// use std::collections::HashMap;
// use tokio_postgres::{Error, NoTls, Row};
use tokio_postgres::Row;
// use std::fmt;


// use dotenv::dotenv;
// use std::env;

/// Get a user_id ( i32 ) by providing the session_id (i32) 
pub async fn get_user_id_by_session_id( pool: &Pool, session_id: i32 ) -> Result< i32, MyDbError > {
    let client = pool.get().await?;
    let statement = client.prepare( "SELECT user_id FROM sessions WHERE id = $1" ).await?;
    let rows = client.query( &statement,  &[ &session_id ] ).await?;

    if let Some( row ) = rows.into_iter().next() {
        Ok( row.get( "user_id" ))
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