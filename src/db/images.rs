#![allow(dead_code)]

use super::MyDbError;
use chrono::{DateTime, Duration, Utc};
use deadpool_postgres::{Config, Pool};
// use postgres::types::ToSql;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use tokio_postgres::{Error, NoTls, Row};
use std::fmt;

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Image Management Functions ********** ////////////////////
//////////////////////////////////////////////////////////////////////////////////

// add_image: add new image to database //////////////////////////////////////////
pub async fn add_image(pool: &Pool, session_id: i32, file_path: &str) -> Result<i32, MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("INSERT INTO images (session_id, file_path, created_at, updated_at) VALUES ( $1, $2, NOW(), NOW() ) RETURNING id")
        .await
        .map_err( MyDbError::PostgresError )?;

    match client.query_one(&statement, &[&session_id, &file_path]).await {

        Ok(row) => {
            let image_id: i32 = row.get(0);
            println!("Image ID: {}", image_id); 
            assert!( image_id > 0); 
            Ok(image_id)
        },
        Err(e) => {
            println!("Error adding image: {:?}", e);
            Err(MyDbError::NotFound)
        }
    }
        // CREATE TABLE IF NOT EXISTS images (
        //     id              SERIAL PRIMARY KEY,
        //     session_id      INTEGER REFERENCES sessions,
        //     file_path       VARCHAR NOT NULL,
        //     created_at      TIMESTAMP NOT NULL,
        //     updated_at      TIMESTAMP NOT NULL
        // )
}

// get_all_images: all images associated with a user_id ///////////////////////////////
pub async fn get_all_images(pool: &Pool, user_id: i32) -> Result<Vec< Image >, MyDbError> {

    let client = pool.get().await?;
    let statement = client.prepare("SELECT * FROM images WHERE user_id = $1").await?;
    let rows = client.query(&statement, &[&user_id]).await?;

    let mut images = Vec::new();
    for row in rows {
        let image = Image {
            id: row.get("id"),
            session_id: row.get("session_id"),
            file_path: row.get("file_path"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        };
        images.push(image); 
    }

    if images.is_empty() {
        Err( MyDbError::NotFound )
    } else {
        Ok( images )
    }
}

// get_single_image: a single image by image_ID ///////////////////////////////
pub async fn get_single_image(pool: &Pool, image_id: i32) -> Result<Image, MyDbError> {

    let client = pool.get().await?;
    let statement = client.prepare("SELECT * FROM images WHERE image_id = $1").await?;
    let rows = client.query(&statement, &[&image_id ]).await?;
    if let Some(row) = rows.into_iter().next() {
        Ok(Image {
            id: row.get("id"),
            session_id: row.get("session_id"),
            file_path: row.get("file_path"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    } else {
        Err(MyDbError::NotFound)
    }
}

// udpate_image: update image data/details ///////////////////////////////////////
pub async fn update_image(pool: &Pool, id: i32, new_file_path: &str) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("UPDATE images set file_path = $1 WHERE id = $2")
        .await?;
    let result = client.execute(&statement, &[&new_file_path, &id]).await?;

    if result == 0 {
        // No rows were updated, i.e., the image was not found
        Err(MyDbError::NotFound)
    } else {
        Ok(())
    }
}

// delete_image: delete image from database //////////////////////////////////////
pub async fn delete_image(pool: &Pool, id: i32) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client.prepare("DELETE FROM images WHERE id = $1").await?;
    let result = client.execute(&statement, &[&id]).await?;
    if result == 0 {
        // No rows were deleted, i.e., the image was not found
        Err(MyDbError::NotFound)
    } else {
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Image Representation ********** //////////////////////////
//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: i32,
    pub session_id: i32,
    pub file_path: String,
    pub created_at: String,
    pub updated_at: String,
    // Add other fields TODO:
}