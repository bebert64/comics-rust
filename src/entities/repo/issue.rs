use super::{super::Issue, establish_connection, schema::issues};
use crate::{ComicsError, Result};
use diesel::prelude::*;

pub fn fetch_by_id(id: i32) -> Result<Option<Issue>> {
    let mut connection = establish_connection()?;
    Ok(issues::table
        .filter(issues::id.eq(id))
        .first::<Issue>(&mut connection)
        .optional()?)
}

pub fn save(issue: Issue) -> Result<Issue> {
    match issue.id {
        Some(_) => update(issue),
        None => insert(issue),
    }
}

fn insert(mut issue: Issue) -> Result<Issue> {
    let mut connection = establish_connection()?;
    let id = diesel::insert_into(issues::table)
        .values(&issue)
        .returning(issues::id)
        .get_result(&mut connection)?;
    issue.id = Some(id);
    Ok(issue)
}

fn update(issue: Issue) -> Result<Issue> {
    let mut connection = establish_connection()?;
    let id = issue.id.unwrap();
    diesel::update(issues::table.find(id))
        .set(&issue)
        .execute(&mut connection)?;
    Ok(issue)
}

pub fn delete(issue: &Issue) -> Result<()> {
    let id = issue.id.ok_or(ComicsError::NoIdError)?;
    let mut connection = establish_connection()?;
    diesel::delete(issues::table.find(id)).execute(&mut connection)?;
    Ok(())
}
