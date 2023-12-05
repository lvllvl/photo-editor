use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    let mut client = Client::connect(
        "postgresql://postgres:postgres@localhost/photoshop",
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

    // Create Image Table
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS images (
            id              SERIAL PRIMARY KEY,
            session_id      INTEGER REFERENCES sessions,
            file_path       VARCHAR NOT NULL
            // Additional fields as necessary
        )
    ",
    )?;

    // Create Layers Table
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS layers (
            id              SERIAL PRIMARY KEY,
            image_id        INTEGER REFERENCES images,
            layer_data      BYTEA
            // Additional fields as necessary
        )
    ",
    )?;

    Ok(())
}
