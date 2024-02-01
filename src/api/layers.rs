use crate::db;
use crate::db::*; 
use actix_web::{web, App, http, HttpResponse, HttpServer, Responder, test};
use deadpool_postgres::{Config, Pool};
use serde::Deserialize;
use serde_json::json;
use tokio_postgres::{Error, NoTls, Row};
use super::MyDbError;

//////////////////////////////////////////////////////////////////////////////////
////////////// *****  Layer Route Handler Functions ***** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////

/// Add Layer: Add a layer to an existing image
/// The default should be 1 layer minimum.
async fn add_layer_handler(pool: web::Data<Pool>,
                           image_id: i32,
                           layer_name: &str,
                           layer_type: &str,
                           layer_data: &[u8],
                           order: i32)
                           -> HttpResponse
{
    match db::layers::add_layer(&pool, image_id, layer_name, layer_type, layer_data, order).await
    {
        Ok(_) => HttpResponse::Ok().json("Image Layer was added successfully!"),
        Err(MyDbError::NotFound) => HttpResponse::NotFound().json("Layer not added!"),
        Err(_) => HttpResponse::InternalServerError().json("Internal Server Error!"),
    }
}

// Get layer by layer_id

// Update layer

// Delete layer