use postgres::{Client, Error, NoTls};
use std::env;
use dotenv::dotenv;

fn main() -> Result<(), Error> {

    dotenv().ok(); // Load variables from .env file
    // Get the database url from the environment
    let database_url = env::var("DATABASE_URL")
        .expect( "DATABASE_URL must be set");

    let mut client = Client::connect(
        &database_url,
        NoTls,
    )?;

    // Create User Table
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            username        VARCHAR UNIQUE NOT NULL,
            email           VARCHAR UNIQUE NOT NULL
        )
    ",
    )?;
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
    )?;
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
    )?;
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
    )?;
    println!("layers table created successfully.");

    Ok(())
}
