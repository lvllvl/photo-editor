use std::collections::HashMap;
use chrono::{ Utc, Duration }; 
use deadpool_postgres::{Config, Pool};
use serde_json::json;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, NoTls, Row };
use postgres::types::ToSql;

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** DB Connection Management ********** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////
// Manage database connections ////////////////////////////////////////////////////
// setup a pool of connections to the database ////////////////////////////////////
pub fn create_pool() -> Pool {
    let cfg = Config::new();
    // Set configuration details...
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

    // Create Image Table ///////////////////////////////////////////////////////
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS images (
            id              SERIAL PRIMARY KEY,
            session_id      INTEGER REFERENCES sessions,
            file_path       VARCHAR NOT NULL
            created_at      TIMESTAMP NOT NULL,
            updated_at      TIMESTAMP NOT NULL,
            -- Additional fields as necessary
        )
    ",
        )
        .await?;
    println!("images table created successfully.");

    // Create Layers Table //////////////////////////////////////////////////////
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS layers (
            id              SERIAL PRIMARY KEY,
            image_id        INTEGER REFERENCES images,
            layer_name      VARCHAR( 255 ),
            creation_date   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            last_modified   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            user_id         INTEGER REFERENCES users,
            layer_type      VARCHAR( 50 ),
            visibility      BOOLEAN DEFAULT TRUE,
            opacity         FLOAT DEFAULT 100,
            layer_data      BYTEA
            order           INTEGER,
            -- Additional fields as necessary
        )
    ",
        )
        .await?;
    println!("layers table created successfully.");

    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////
// ************* User Insertion Functions ************** /////////////////////////
//////////////////////////////////////////////////////////////////////////////////

// Add a user to the database ////////////////////////////////////////////////////
pub async fn add_user(pool: &Pool, username: &str, email: &str) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("INSERT INTO users (username, email) VALUES ($1, $2)")
        .await?;
    client.execute(&statement, &[&username, &email]).await?;
    Ok(())
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** User Retrieval Functions ********** //////////////////////
//////////////////////////////////////////////////////////////////////////////////

// Get a user by username from the database //////////////////////////////////////
pub async fn get_user_by_username(
    client: &mut deadpool_postgres::Client,
    username: &str,
) -> Result<User, MyDbError> {
    let statement = client
        .prepare("SELECT * FROM users WHERE username = $1")
        .await?;
    let rows = client.query(&statement, &[&username]).await?;

    if let Some(row) = rows.into_iter().next() {
        // Assuming 'User' is a struct representing a user
        Ok(User::from_row(&row))
    } else {
        Err(MyDbError::NotFound)
    }
}
// Get a user by email from the database /////////////////////////////////////////
pub async fn get_user_by_email(pool: &Pool, email: &str) -> Result<User, MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("SELECT * FROM users WHERE email = $1")
        .await?;
    let rows = client.query(&statement, &[&email]).await?;

    if let Some(row) = rows.into_iter().next() {
        Ok(User::from_row(&row))
    } else {
        Err(MyDbError::NotFound)
    }
}
// Get all users from the database ///////////////////////////////////////////////
pub async fn get_all_users(pool: &Pool) -> Result<Vec<User>, MyDbError> {
    let client = pool.get().await?;
    let statement = client.prepare("SELECT * FROM users").await?;
    let rows = client.query(&statement, &[]).await?;

    let mut users = Vec::new();

    for row in rows {
        users.push(User::from_row(&row));
    }

    Ok(users)
}

// TODO: Retrieve users based on various filters e.g., age, location, etc. ///////
// TODO: Retrieve recent users, from a certain timeframe /////////////////////////

//////////////////////////////////////////////////////////////////////////////////
// User Update Functions /////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////

// Update user email in the database /////////////////////////////////////////////
pub async fn update_user_email(
    pool: &Pool,
    username: &str,
    new_email: &str,
) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("UPDATE users SET email = $1 WHERE username = $2")
        .await?;
    let result = client.execute(&statement, &[&new_email, &username]).await?;

    if result == 0 {
        // No rows were updated, i.e., the user was not found
        Err(MyDbError::NotFound)
    } else {
        Ok(())
    }
}

// TODO: Update user profile, profile details, names, contact info, etc. /////////
// TODO: Deactivate user account, or activate ////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Session Management Functions ********** //////////////////
//////////////////////////////////////////////////////////////////////////////////

// create_session: for an individual /////////////////////////////////////////////
pub async fn create_session( pool: &Pool, user_id: i32 ) -> Result< i32, MyDbError >{

    let client = pool.get().await?;
    let mut creation_time = Utc::now();
    let mut expiration_time = creation_time + Duration::hours(1); // Max session length 
    let last_activity = creation_time;
    let session_data = json!( {} ); // Initialize with empty JSON object, using serde_json json! macro

    let statement = client
        .prepare( "INSERT INTO sessions (user_id, creation_time, expiration_time, last_activity, session_data ) VALUES ($1, $2, $3, $4, $5) RETURNING id" )
        .await?;

    let session_id: i32 = client
        .query_one( &statement, &[&user_id, &creation_time, &expiration_time, &last_activity, &session_data] )
        .await?
        .get( 0 );

    Ok( session_id )

}

// end_session: for an individual  ///////////////////////////////////////////////
pub async fn end_session( pool: &Pool, user_id: i32 ) -> Result< (), MyDbError > {

    // Fetch a database connection from the pool
    let client = pool.get().await?;

   // Prep the SQL query to update the session
   let statement = client
    .prepare( "UPDATE session SET end_time = NOW() WHERE id = $1" )
    .await?;

    // Execute the query
    let result = client.execute( &statement, &[ &session_id ] ).await?;

    // Check if any rows were affected
    if result == 0 {
        // No rows were updated, session not found or already ended
        Err( Error::from( std::io::Error::new( std::io::ErrorKind::NotFound, "Session not found" )))
    } else {
        // Session succesfully ended 
        Ok( () )
    }
}

// get_active_sessions: for all current users ////////////////////////////////////
pub async fn get_active_sessions( pool: &Pool, user_id: i32 ) -> Result< Vec< Session >, MyDbError > {

    let client = pool.get().await?;
    let statement = client
        .prepare( "SELECT * FROM sessions WHERE user_id = $1 AND end_time IS NULL" )
        .await?;
    let rows = client.query( &statement, &[ &user_id ]).await?;
    let mut sessions = Vec::new();

    for row in rows {
        // Session is a struct, that represents a single session
        sessions.push( Session::from_row( &row ));
    }

    if sessions.is_empty() {
        Err( MyDbError::NotFound )
    } else {
        Ok( sessions )
    }
}

// get_session_ID for a SINGLE user //////////////////////////////////////////////
pub async fn get_session_id_for_user( pool: &Pool, user_id: i32 ) -> Result< i32, MyDbError > {
    let client = pool.get().await?;
    let statement = client.prepare( "SELECT id FROM sessions WHERE user_id = $1" ).await?;

    let rows = client.query( &statement, &[ &user_id ] ).await?;
    if let Some( row ) = rows.into_iter().next() {
        Ok( row.get( "id" ) )
    } else {
        Err( MyDbError::NotFound )
    }
} 

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Image Management Functions ********** ////////////////////
//////////////////////////////////////////////////////////////////////////////////

// add_image: add new image to database //////////////////////////////////////////
pub async fn add_image(pool: &Pool, session_id: i32, file_path: &str) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare( "INSERT INTO images (session_id, file_path, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())")
        .await?;
    client
        .execute(&statement, &[&session_id, &file_path])
        .await?;
    Ok(())
}

// get_image: get image by id ////////////////////////////////////////////////////
pub async fn get_image(pool: &Pool, id: i32) -> Result<Image, MyDbError> {
    let client = pool.get().await?;
    let statement = client.prepare("SELECT * FROM images WHERE id = $1").await?;
    let rows = client.query(&statement, &[&id]).await?;
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
//////////// ********** Layer Management Functions ********** ////////////////////
//////////////////////////////////////////////////////////////////////////////////

// add_layer: add new layer to an image //////////////////////////////////////////
pub async fn add_layer(
    pool: &Pool,
    image_id: i32,
    layer_name: &str,
    layer_type: &str,
    layer_data: &[u8],
    order: i32,
) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare( "INSERT INTO layers (image_id, layer_name, layer_type, layer_data) VALUES ($1, $2, $3, $4)")
        .await?;

    client
        .execute(
            &statement,
            &[&image_id, &layer_name, &layer_type, &layer_data],
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
            order: row.get("order"),
        })
    } else {
        Err(MyDbError::NotFound)
    }
}

// get_layers_by_image_id: Retrieve ALL layers for a specific image //////////////
pub async fn get_layers_by_image_id(pool: &Pool, image_id: i32) -> Result<Vec<Layer>, MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("SELECT * FROM layers WHERE image_id = $1 ORDER BY \"order\"")
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
            order: row.get("order"),
        });
    }
    if layers.is_empty() {
        Err(MyDbError::NotFound)
    } else {
        Ok(layers)
    }
}

// TODO: pub async fn get_layer_statistics(pool: &Pool) -> Result<LayerStatistics, MyDbError>;

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
    let old_order = layer_map.get(&moved_layer_id).unwrap().order;

    // Iterate over all layers and update the order of layers in between
    for layer in layer_map.values_mut() {
        // Compare the old_order and new order of the moved layer
        match old_order.cmp(&new_order) {
            // Layer is moved down: Decrease order of layers in between
            std::cmp::Ordering::Less => {
                // if current layer order > old_order && current layer order <= new_order
                // Layer is moved down
                if layer.order > old_order && layer.order <= new_order {
                    layer.order -= 1;
                }
            }
            std::cmp::Ordering::Greater => {
                // Layer is moved up: Increase order of layers in between
                if layer.order < old_order && layer.order >= new_order {
                    layer.order += 1;
                }
            }
            std::cmp::Ordering::Equal => {
                // Layer is moved to the same position: Do nothing
            }
        }
    }

    // Finally, set the new order for the moved layer
    layer_map.get_mut(&moved_layer_id).unwrap().order = new_order;
}

// Construct a batch update query for all affected layers ////////////////////////
fn construct_batch_update_query(layer_map: &HashMap<i32, Layer>) -> String {
    let mut query = String::new();
    for layer in layer_map.values() {
        query.push_str(&format!(
            "UPDATE layers SET order = {} WHERE id = {};",
            layer.order, layer.id
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
    new_layer_order: i32,
) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare(
            "UPDATE layers set layer_name = $1, layer_type = $2, layer_data = $3 WHERE id = $4",
        )
        .await?;
    let result = client
        .execute(
            &statement,
            &[&new_layer_name, &new_layer_type, &new_layer_data, &id],
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
            order: row.get("order"),
        };

        let statement = client.prepare( "INSERT INTO layers (image_id, layer_name, creation_date, last_modified, user_id, layer_type, visibility, opacity, layer_data, order) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)").await?;
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
                    &layer.order,
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

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** User Deletion Fuctions ********** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////

// Delete a user from the database ///////////////////////////////////////////////
pub async fn delete_user(pool: &Pool, username: &str) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("DELETE FROM users WHERE username = $1")
        .await?;
    let result = client.execute(&statement, &[&username]).await?;

    if result == 0 {
        // No rows were deleted, i.e., the user was not found
        Err(MyDbError::NotFound)
    } else {
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Error Handling ********** ////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum MyDbError {
    PostgresError(postgres::Error),
    PoolError(deadpool::managed::PoolError<postgres::Error>),
    NotFound,
    // ... other error types as needed
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

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** User Representation ********** ///////////////////////////
//////////////////////////////////////////////////////////////////////////////////

// Struct to represent a user ////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    // Add other fields TODO:
}

impl User {
    // Create a new user instance from a database row
    pub fn from_row(row: &Row) -> User {
        User {
            id: row.get("id"),
            username: row.get("username"),
            email: row.get("email"),
            // TODO: add other fields
        }
    }
}
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
    pub order: i32,          // Maintain layer order!
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
            order: row.get("order"),
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Session Representation ********** ////////////////////////
//////////////////////////////////////////////////////////////////////////////////
#[ derive( Debug, Serialize, Deserialize )]
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
    pub fn from_row( row: &Row ) -> Session {
        Session {
            id: row.get( "id" ),
            user_id: row.get( "user_id" ),
            creation_time: row.get( "creation_time" ),
            expiration_time: row.get( "expiration_time" ),
            last_activity: row.get( "last_activity" ),
            session_data: row.get( "session_data" )
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** Unit Tests ********** ////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    //////////////////////////////////////////////////////////////////////////////
    ///////////////////// ********** Setup ********** ////////////////////////////
    //////////////////////////////////////////////////////////////////////////////
    // Setup mock database connection
    fn setup() -> Pool {
        dotenv().ok(); // Load variables from .env file
        let mut cfg = Config::new();

        cfg.host = env::var("DB_HOST").ok();
        cfg.user = env::var("DB_USER").ok();
        cfg.password = env::var("DB_PASSWORD").ok();
        cfg.dbname = env::var("DB_NAME").ok();

        cfg.create_pool(None, NoTls).expect("Failed to create pool")
    }

    //////////////////////////////////////////////////////////////////////////////
    ////////////////////// ********** User Tests ********** //////////////////////
    //////////////////////////////////////////////////////////////////////////////
    mod user_tests {
        use super::*;

        #[tokio::test]
        async fn test_add_user() {
            let pool = setup();
            match add_user(&pool, "test_user", "test@example.com").await {
                Ok(_) => println!("Test add_user: User added successfully"),
                Err(e) => eprintln!("Test add_user failed: {:?}", e),
            }
        }

        #[tokio::test]
        async fn test_get_user_by_username() {
            let pool = setup();
            let _ = add_user(&pool, "testuser", "test@example.com").await; // Add a user for test

            let mut client = pool.get().await.unwrap();

            match get_user_by_username(&mut client, "testuser").await {
                Ok(user) => {
                    assert_eq!(user.username, "testuser");
                    assert_eq!(user.email, "test@example.com");
                    println!("Test get_user_by_username: User found successfully");
                }
                Err(e) => eprintln!("Test get_user_by_username failed: {:?}", e),
            }
        }
    }

    // TODO:
    //////////////////////////////////////////////////////////////////////////////
    ////////////////////// ********** Image Tests ********** /////////////////////
    //////////////////////////////////////////////////////////////////////////////
    mod image_tests {
        use super::*;

        async fn setup_for_image_tests(pool: &Pool) -> Result<(i32, String), MyDbError> {
            let user_id = add_user(&pool, "test_user", "test@example.com").await?;
            // TODO: Setup session ID
            let session_id = create_session(&pool, user_id).await?;
            Ok((session_id, String::from("path/to/test/image.png")))
        }

        #[tokio::test]
        async fn test_add_image() {
            let pool = setup();
            let (session_id, file_path) = setup_for_image_tests(&pool).await.unwrap();

            let result = add_image(&pool, session_id, &file_path).await;
            assert!(result.is_ok());
            // TODO: Add additional checks here
        }

        #[tokio::test]
        async fn test_get_image_by_id() {
            let pool = setup();
            let (session_id, file_path) = setup_for_image_tests(&pool).await.unwrap();

            let add_result = add_image(&pool, session_id, &file_path).await;
            assert!(add_result.is_ok());

            // Retreieve the added image
            // TODO: let image_id = /* get the ID of the newly added image */
            let get_result = get_image(&pool, image_id).await;
            assert!(get_result.is_ok());

            // TODO: add other checks on the retrieved image
        }

        // #[tokio::tests]
        // async fn test_update_image() {
        //     // similar setup and assertion as previous tests
        //     // update the image details and verify changes 

        // }

        // #[tokio::tests]
        // async fn test_delete_image() {
        //     // similar setup and assertion as previous tests
        //     // delete the image and verify it is NO LONGER retrievable 

        // }
    }

    //////////////////////////////////////////////////////////////////////////////
    ////////////////////// ********** Layer Tests ********** /////////////////////
    //////////////////////////////////////////////////////////////////////////////

    //////////////////////////////////////////////////////////////////////////////
    ////////////////////// ********** Session Tests ********** ///////////////////
    //////////////////////////////////////////////////////////////////////////////

    //////////////////////////////////////////////////////////////////////////////
    ////////////////////// ********** Analytics Tests ********** /////////////////
    //////////////////////////////////////////////////////////////////////////////
}
