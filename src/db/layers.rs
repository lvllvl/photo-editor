#![allow(dead_code)]
use super::MyDbError;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_postgres::Row;
// use serde_json::json;
// use tokio_postgres::{Error, NoTls, Row};
// use std::fmt;
// use deadpool_postgres::{Config, Pool};
// use postgres::types::ToSql;
// use chrono::{DateTime, Duration, Utc};

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Layer Management Functions ********** ////////////////////
//////////////////////////////////////////////////////////////////////////////////

// add_layer: add new layer to an image //////////////////////////////////////////
pub async fn add_layer(
    pool: &Pool,
    image_id: i32,
    layer_name: &str,
    layer_type: &str,
    layer_data: &[u8],
    layer_order: i32,
) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare( "INSERT INTO layers (image_id, layer_name, layer_type, layer_data, layer_order ) VALUES ($1, $2, $3, $4, $5)")
        .await?;

    client
        .execute(
            &statement,
            &[&image_id, &layer_name, &layer_type, &layer_data, &layer_order],
        )
        .await?;
    Ok(())
}

// get a single layer by layer id ////////////////////////////////////////////////
pub async fn get_layer_by_layer_id(pool: &Pool, id: i32) -> Result<Layer, MyDbError> {
    let client = pool.get().await?;
    let statement = client.prepare("SELECT * FROM layers WHERE id = $1").await?;
    let rows = client.query(&statement, &[&id]).await?;
    if let Some(row) = rows.into_iter().next() {
        Ok(Layer {
            id: row.get("id"),
            image_id: row.get("image_id"),
            layer_name: row.get("layer_name"),
            creation_date: row.get("creation_date"),
            last_modified: row.get("last_modified"),
            user_id: row.get("user_id"),
            layer_type: row.get("layer_type"),
            visibility: row.get("visibility"),
            opacity: row.get("opacity"),
            layer_data: row.get("layer_data"),
            layer_order: row.get("layer_order"),
        })
    } else {
        Err(MyDbError::NotFound)
    }
}

// get_layers_by_image_id: Retrieve ALL layers for a specific image //////////////
pub async fn get_layers_by_image_id(pool: &Pool, image_id: i32) -> Result<Vec<Layer>, MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("SELECT * FROM layers WHERE image_id = $1 ORDER BY \"layer_order\"")
        .await?;
    let rows = client.query(&statement, &[&image_id]).await?;

    // Sort the layers based on the order field
    let mut layers = Vec::new();
    for row in rows {
        layers.push(Layer {
            id: row.get("id"),
            image_id: row.get("image_id"),
            layer_name: row.get("layer_name"),
            creation_date: row.get("creation_date"),
            last_modified: row.get("last_modified"),
            user_id: row.get("user_id"),
            layer_type: row.get("layer_type"),
            visibility: row.get("visibility"),
            opacity: row.get("opacity"),
            layer_data: row.get("layer_data"),
            layer_order: row.get("layer_order"),
        });
    }
    if layers.is_empty() {
        Err(MyDbError::NotFound)
    } else {
        Ok(layers)
    }
}

// update_layer_order: update layer order ////////////////////////////////////////
pub async fn update_layer_order(
    pool: &Pool,
    image_id: i32,
    layer_id: i32,
    new_order: i32,
) -> Result<(), MyDbError> {
    let layers = get_layers_by_image_id(pool, image_id).await?;
    let mut layer_map = HashMap::new();

    // Create a map from layer id to layer data
    for layer in layers {
        layer_map.insert(layer.id, layer);
    }

    reorder_layers_in_memory(&mut layer_map, layer_id, new_order);

    let batch_update_query = construct_batch_update_query(&layer_map);
    execute_batch_update(pool, batch_update_query).await?;

    Ok(())
}

// Reorder layers in memory based on new order ///////////////////////////////////
fn reorder_layers_in_memory(
    layer_map: &mut HashMap<i32, Layer>,
    moved_layer_id: i32,
    new_order: i32,
) {
    // Get the old order number of the moved layer, e.g., 1 or 2 or 3, etc.
    let old_order = layer_map.get(&moved_layer_id).unwrap().layer_order;

    // Iterate over all layers and update the order of layers in between
    for layer in layer_map.values_mut() {
        // Compare the old_order and new order of the moved layer
        match old_order.cmp(&new_order) {
            // Layer is moved down: Decrease order of layers in between
            std::cmp::Ordering::Less => {
                // if current layer order > old_order && current layer order <= new_order
                // Layer is moved down
                if layer.layer_order > old_order && layer.layer_order <= new_order {
                    layer.layer_order -= 1;
                }
            }
            std::cmp::Ordering::Greater => {
                // Layer is moved up: Increase order of layers in between
                if layer.layer_order < old_order && layer.layer_order >= new_order {
                    layer.layer_order += 1;
                }
            }
            std::cmp::Ordering::Equal => {
                // Layer is moved to the same position: Do nothing
            }
        }
    }

    // Finally, set the new order for the moved layer
    layer_map.get_mut(&moved_layer_id).unwrap().layer_order = new_order;
}

// Construct a batch update query for all affected layers ////////////////////////
fn construct_batch_update_query(layer_map: &HashMap<i32, Layer>) -> String {
    let mut query = String::new();
    for layer in layer_map.values() {
        query.push_str(&format!(
            "UPDATE layers SET order = {} WHERE id = {};",
            layer.layer_order, layer.id
        ));
    }
    query
}

// Execute the batch update query ////////////////////////////////////////////////
async fn execute_batch_update(pool: &Pool, query: String) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client.prepare(&query).await?;
    client.execute(&statement, &[]).await?;
    Ok(())
}

// update_layer: update layer data/details ///////////////////////////////////////
pub async fn update_layer(
    pool: &Pool,
    id: i32,
    new_layer_name: &str,
    new_layer_type: &str,
    new_layer_data: &[u8],
    new_layer_order: i32)
-> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare(
            "UPDATE layers set layer_name = $1, layer_type = $2, layer_data = $3 WHERE id = $4",
        )
        .await?;
    let result = client
        .execute(
            &statement,
            &[&id, &new_layer_name, &new_layer_type, &new_layer_data, &new_layer_data],
        )
        .await?;

    if result == 0 {
        // No rows were updated, i.e., the layer was not found
        Err(MyDbError::NotFound)
    } else {
        Ok(())
    }
}

// delete_layer: delete layer from database/image ////////////////////////////////
pub async fn delete_layer(pool: &Pool, id: i32) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client.prepare("DELETE FROM layers WHERE id = $1").await?;
    let result = client.execute(&statement, &[&id]).await?;
    if result == 0 {
        // No rows were deleted, i.e., the layer was not found
        Err(MyDbError::NotFound)
    } else {
        Ok(())
    }
}

// update_toggle_layer_visibility: toggle layer visibility ///////////////////////
pub async fn update_toggle_layer_visibility(
    pool: &Pool,
    layer_id: i32,
    visible: bool,
) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("UPDATE layers SET visibility = $1 WHERE id = $2")
        .await?;
    let result = client.execute(&statement, &[&visible, &layer_id]).await?;
    if result == 0 {
        // No rows were updated, i.e., the layer was not found
        Err(MyDbError::NotFound)
    } else {
        Ok(())
    }
}

// duplicate_layer: duplicate a layer, returns new layer ID //////////////////////
pub async fn duplicate_layer(pool: &Pool, layer_id: i32) -> Result<i32, MyDbError> {
    let client = pool.get().await?;
    let statement = client.prepare("SELECT * FROM layers WHERE id = $1").await?;
    let rows = client.query(&statement, &[&layer_id]).await?;

    if let Some(row) = rows.into_iter().next() {
        let layer = Layer {
            id: row.get("id"),
            image_id: row.get("image_id"),
            layer_name: row.get("layer_name"),
            creation_date: row.get("creation_date"),
            last_modified: row.get("last_modified"),
            user_id: row.get("user_id"),
            layer_type: row.get("layer_type"),
            visibility: row.get("visibility"),
            opacity: row.get("opacity"),
            layer_data: row.get("layer_data"),
            layer_order: row.get("layer_order"),
        };

        let statement = client.prepare( "INSERT INTO layers (image_id, layer_name, creation_date, last_modified, user_id, layer_type, visibility, opacity, layer_data, layer_order) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)").await?;
        let result = client
            .execute(
                &statement,
                &[
                    &layer.image_id,
                    &layer.layer_name,
                    &layer.creation_date,
                    &layer.last_modified,
                    &layer.user_id,
                    &layer.layer_type,
                    &layer.visibility,
                    &layer.opacity,
                    &layer.layer_data,
                    &layer.layer_order,
                ],
            )
            .await?;

        if result == 0 {
            // No rows were inserted, i.e., the layer was not duplicated
            Err(MyDbError::NotFound)
        } else {
            Ok(layer.id)
        }
    } else {
        Err(MyDbError::NotFound)
    }
}

// TODO:
// Returns new merged layer ID
// pub async fn merge_layers(pool: &Pool, layer_ids: Vec<i32>) -> Result<LayerGroup, MyDbError> {
//     let client = pool.get().await?;

//     // Fetch all layers in one query
//     let fetch_statement = client
//         .prepare("SELECT * FROM layers WHERE id = ANY($1)")
//         .await?;
//     let rows = client.query(&fetch_statement, &[&layer_ids]).await?;

//     // Process rows into layers and merge their data
//     let mut merged_layer_data: Vec<u8> = Vec::new();
//     for row in rows {
//         let layer_data: Vec<u8> = row.get("layer_data");
//         // Actual merging logic here (currently just appending)
//         merged_layer_data.extend(layer_data);
//     }

//     // Insert the merged layer into the database
//     let insert_statement = client
//         .prepare("INSERT INTO layers (image_id, layer_name, layer_type, visibility, opacity, layer_data) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
//         .await?;

//     // Assuming you've determined the appropriate values for these parameters
//     let image_id = /* determine image_id */;
//     let layer_name = "Merged Layer";
//     let layer_type = /* determine layer_type */;
//     let visibility = true;
//     let opacity = 1.0;

//     let merged_layer_id: i32 = client
//         .query_one(&insert_statement, &[&image_id, &layer_name, &layer_type, &visibility, &opacity, &merged_layer_data])
//         .await?
//         .get(0);

//     // Create and return a new LayerGroup
//     let layer_group = LayerGroup {
//         group_id: merged_layer_id, // Assuming group_id is the merged layer's ID
//         group_name: String::from("Merged Group"),
//         layer_ids: layer_ids,
//         total_layers: layer_ids.len() as i32,
//         creation_date: Utc::now().to_rfc3339(), // Using chrono for current timestamp
//     };

//     Ok(layer_group)
// }

// TODO: pub async fn search_layers(pool: &Pool, search_query: &str) -> Result<Vec<Layer>, MyDbError>;
// TODO: pub async fn create_layer_group(pool: &Pool, group_name: &str, layer_ids: Vec<i32>) -> Result<i32, MyDbError>; // Returns group ID

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** LayerStatistics Representation ********** ////////////////
//////////////////////////////////////////////////////////////////////////////////
/// Merge a group of layers into a single layer
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerGroup {
    pub group_id: i32,
    pub group_name: String,
    pub layer_ids: Vec<i32>,
    pub total_layers: i32,
    pub creation_date: String,
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** LayerStatistics Representation ********** ////////////////
//////////////////////////////////////////////////////////////////////////////////
/// Struct to represent layer statistics
#[derive(Debug, Serialize, Deserialize)]
pub struct LayerStatistics {
    pub total_layers: i32,
    pub average_layers_per_image: f32,
    // most_active_users: Vec<User>, // Assuming UserId is a type representing a user ID
    // recent_activity: LayerActivityStatistics,
    // size_statistics: LayerSizeStatistics,
    // visibility_statistics: LayerVisibilityStatistics,
    // opacity_usage: OpacityusageStatistics,
    // layer_type_distribution: LayerTypeDistribution,
    // modifications_frequency: ModificationFrequencyStatistics,
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Layer Representation ********** //////////////////////////
//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    pub id: i32,
    pub image_id: i32,
    pub layer_name: String,
    pub creation_date: String,
    pub last_modified: String,
    pub user_id: i32,
    pub layer_type: String,
    pub visibility: bool,
    pub opacity: f32,
    pub layer_data: Vec<u8>, // Raw data for the layer
    pub layer_order: i32,          // Maintain layer order!
                             // Add other fields TODO:
}

impl Layer {
    // Create a new layer instance from a database row
    pub fn from_row(row: &Row) -> Layer {
        Layer {
            id: row.get("id"),
            image_id: row.get("image_id"),
            layer_name: row.get("layer_name"),
            creation_date: row.get("creation_date"),
            last_modified: row.get("last_modified"),
            user_id: row.get("user_id"),
            layer_type: row.get("layer_type"),
            visibility: row.get("visibility"),
            opacity: row.get("opacity"),
            layer_data: row.get("layer_data"),
            layer_order: row.get("layer_order"),
        }
    }
}