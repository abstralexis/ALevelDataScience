//! This file contains helper functions for creating the
//! database for this project. I might eventually add
//! functions for abstracting adding and removing entries to
//! the databases in a generic way so that adding and removing
//! is easier to read when actually used.

use sqlx::{migrate::MigrateDatabase, Sqlite, SqliteConnection};

/// This is the Database URL for the project.
pub const DB_URL: &str = "sqlite://db.sql";

/// This function is intended for generating the tables for the project.
/// It adds the relevant tables to a connection.
async fn generate_tables() -> Result<(), sqlx::Error> {
    Ok(())
}

/// This function is how the database file is created.
async fn create_db() -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating Database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Database created at {}", DB_URL),
            Err(e) => return Err(e),
        }
    } else {
        // This is inherently flawed - the database could have just
        // failed to be found and it gives this error. Changing
        // The `unwrap_or` to a match statement would be more
        // beneficial.
        println!("Database already exists")
    }

    Ok(())
}

/// This function just gets the sqlite connection in a more concise way.
async fn get_connection() -> Result<sqlx::SqlitePool, sqlx::Error> {
    sqlx::SqlitePool::connect(DB_URL).await
}
