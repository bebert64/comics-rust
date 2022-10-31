use super::{establish_connection, schema::issues, super::Issue};
use crate::{Result};
use diesel::prelude::*;

pub fn fetch_by_id(id: i32) -> Result<Option<Issue>> {
    let mut connection = establish_connection()?;
    Ok(issues::table
        .filter(issues::id.eq(id))
        .first::<Issue>(&mut connection)
        .optional()?)
}

pub fn save(issue: &Issue) -> Result<()> {
    match fetch_by_id(issue.id)? {
        Some(_) => update(issue),
        None => insert(issue),
    }
}

fn insert(issue: &Issue) -> Result<()> {
    let mut connection = establish_connection()?;
    diesel::insert_into(issues::table)
        .values(issue)
        .execute(&mut connection)?;
    Ok(())
}

fn update(issue: &Issue) -> Result<()> {
    let mut connection = establish_connection()?;
    diesel::update(issue)
        .set(issue)
        .execute(&mut connection)?;
    Ok(())
}

pub fn delete(issue: &Issue) -> Result<()> {
    let mut connection = establish_connection()?;
    diesel::delete(issue).execute(&mut connection)?;
    Ok(())
}
