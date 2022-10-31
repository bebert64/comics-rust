use super::{establish_connection, schema::creators, super::Creator};
use crate::{Result, ComicsError};
use diesel::prelude::*;

pub fn fetch_by_id(id: i32) -> Result<Option<Creator>> {
    let mut connection = establish_connection()?;
    Ok(creators::table
        .filter(creators::id.eq(id))
        .first::<Creator>(&mut connection)
        .optional()?)
}

pub fn save(creator: Creator) -> Result<Creator> {
    match creator.id {
        Some(_) => update(creator),
        None => insert(creator),
    }
}

fn insert(mut creator: Creator) -> Result<Creator> {
    let mut connection = establish_connection()?;
    let id = diesel::insert_into(creators::table)
        .values(&creator)
        .returning(creators::id)
        .get_result(&mut connection)?;
    creator.id = Some(id);
    Ok(creator)
}

fn update(creator: Creator) -> Result<Creator> {
    let id = creator.id.unwrap();
    let mut connection = establish_connection()?;
    diesel::update(creators::table.find(id))
        .set(&creator)
        .execute(&mut connection)?;
    Ok(creator)
}

pub fn delete(creator: &Creator) -> Result<()> {
    let id = creator.id.ok_or(ComicsError::NoIdError)?;
    let mut connection = establish_connection()?;
    diesel::delete(creators::table.find(id)).execute(&mut connection)?;
    Ok(())
}
