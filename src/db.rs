use deadpool_postgres::{Config, Pool};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, NoTls, Row};

// Manage database connections ////////////////////////////////////////////////////
// setup a pool of connections to the database ////////////////////////////////////
pub fn create_pool() -> Pool {
    let mut cfg = Config::new();
    // Set configuration details...
    cfg.create_pool(None, NoTls).expect("Failed to create pool")
}

// Add a user to the database ////////////////////////////////////////////////////
pub async fn add_user(pool: &Pool, username: &str, email: &str) -> Result<(), MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("INSERT INTO users (username, email) VALUES ($1, $2)")
        .await?;
    client.execute(&statement, &[&username, &email]).await?;
    Ok(())
}

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

// Setup database schema /////////////////////////////////////////////////////////
pub async fn setup_database(client: &mut deadpool_postgres::Client) -> Result<(), Error> {
    // Create User Table
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

    // Create Session Table
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS sessions (
            id              SERIAL PRIMARY KEY,
            user_id         INTEGER REFERENCES users,
            start_time      TIMESTAMP NOT NULL,
            end_time        TIMESTAMP
        )
    ",
        )
        .await?;
    println!("sessions table created successfully.");

    // Create Image Table
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS images (
            id              SERIAL PRIMARY KEY,
            session_id      INTEGER REFERENCES sessions,
            file_path       VARCHAR NOT NULL
            -- Additional fields as necessary
        )
    ",
        )
        .await?;
    println!("images table created successfully.");

    // Create Layers Table
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS layers (
            id              SERIAL PRIMARY KEY,
            image_id        INTEGER REFERENCES images,
            layer_data      BYTEA
            -- Additional fields as necessary
        )
    ",
        )
        .await?;
    println!("layers table created successfully.");

    Ok(())
}
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

// Struct to represent a user //////////////////////////////////////////////////
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
