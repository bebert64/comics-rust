pub mod creator;
pub mod issue;
pub mod schema;

use crate::Result;
use diesel::sqlite::SqliteConnection;
use diesel::{prelude::*, sql_types::Text};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let mut connection = SqliteConnection::establish(&database_url)?;
    diesel::dsl::sql::<Text>("PRAGMA foreign_keys = ON").execute(&mut connection)?;
    Ok(connection)
}
