use super::MyDbError;
use deadpool_postgres::{Pool, Client};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Error, NoTls, Row};
use std::fmt;

//////////////////////////////////////////////////////////////////////////////////
// ************* User Insertion Functions ************** /////////////////////////
//////////////////////////////////////////////////////////////////////////////////

// Add a user to the database ////////////////////////////////////////////////////
pub async fn add_user(pool: &Pool, username: &str, email: &str) -> Result<i32, MyDbError> {
    let client = pool.get().await?;
    let statement = client
        .prepare("INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id")
        .await?;
    let row = client.query_one(&statement, &[&username, &email]).await?;
    let user_id: i32 = row.get(0);
    Ok(user_id)
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** User Retrieval Functions ********** //////////////////////
//////////////////////////////////////////////////////////////////////////////////

// Get a user by username from the database //////////////////////////////////////
pub async fn get_user_by_username(
    pool: &Pool,
    // client: &mut deadpool_postgres::Client,
    username: &str,
) -> Result<User, MyDbError> {

    let client = pool.get().await?;
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

/// Delete ALL users from the database ////////////////////////////////////////////
pub async fn delete_all_users( pool: &Pool ) -> Result< (), MyDbError > {
    let client = pool.get().await?;
    let statement = client.prepare( "DELETE FROM users" ).await?;
    let result = client.execute( &statement, &[] ).await?;

    if result == 0 {
        Err( MyDbError::NotFound )
    } else {
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////////
//////////// ********** User Representation ********** ///////////////////////////
//////////////////////////////////////////////////////////////////////////////////
// Struct to represent a user ////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize,)]
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

// Implement the Display trait for the User struct
impl fmt::Display for User {
    
    fn fmt( &self, f: &mut fmt::Formatter< '_ > ) -> fmt::Result {
        write!( f, "ID: {}, Username: {}, Email: {}", self.id, self.username, self.email )
    }
}