use deadpool_postgres::{ Config, Pool }; 
use tokio_postgres::{ Error, NoTls, }; 

// Manage database connections ////////////////////////////////////////////////////
// setup a pool of connections to the database ////////////////////////////////////
pub fn create_pool() -> Pool {
    let mut cfg = Config::new();
    // Set configuration details...
    cfg.create_pool(None, NoTls).expect("Failed to create pool")
}

// Add a user to the database ////////////////////////////////////////////////////
pub async fn add_user(pool: &Pool, username: &str, email: &str) -> Result<(), Error> {

    let mut client = pool.get().await?;
    let statement = client.prepare("INSERT INTO users (username, email) VALUES ($1, $2)").await?;
    client.execute(&statement, &[&username, &email]).await?;
    Ok(())
}

// Setup database schema /////////////////////////////////////////////////////////
pub async fn setup_database(client: &mut deadpool_postgres::Client) -> Result<(), Error> {
    
    // Create User Table
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
        )
    ",
    ).await?;
    println!("Users table created successfully.");

    // Create Session Table
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS sessions (
            id              SERIAL PRIMARY KEY,
            user_id         INTEGER REFERENCES users,
            start_time      TIMESTAMP NOT NULL,
            end_time        TIMESTAMP
        )
    ",
    ).await?;
    println!("sessions table created successfully.");

    // Create Image Table
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS images (
            id              SERIAL PRIMARY KEY,
            session_id      INTEGER REFERENCES sessions,
            file_path       VARCHAR NOT NULL
            -- Additional fields as necessary
        )
    ",
    ).await?;
    println!("images table created successfully.");

    // Create Layers Table
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS layers (
            id              SERIAL PRIMARY KEY,
            image_id        INTEGER REFERENCES images,
            layer_data      BYTEA
            -- Additional fields as necessary
        )
    ",
    ).await?;
    println!("layers table created successfully.");
    
    Ok(())

}

#[derive(Debug)]
pub enum MyDbError {
    PostgresError(postgres::Error),
    PoolError(deadpool::managed::errors::PoolError<postgres::Error>),
    // ... other error types as needed
}

impl From<postgres::Error> for MyDbError {
    fn from(err: postgres::Error) -> MyDbError {
        MyDbError::PostgresError(err)
    }
}

impl From<deadpool::managed::errors::PoolError<postgres::Error>> for MyDbError {
    fn from(err: deadpool::managed::errors::PoolError<postgres::Error>) -> MyDbError {
        MyDbError::PoolError(err)
    }
}
