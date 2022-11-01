use super::{establish_connection, schema::creators, super::Creator};
use crate::Result;
use diesel::prelude::*;

pub fn fetch_by_id(id: i32) -> Result<Option<Creator>> {
    let mut connection = establish_connection()?;
    Ok(creators::table
        .filter(creators::comic_vine_id.eq(id))
        .first::<Creator>(&mut connection)
        .optional()?)
}

pub fn save(creator: &Creator) -> Result<()> {
    match fetch_by_id(creator.comic_vine_id)? {
        Some(_) => update(creator),
        None => insert(creator),
    }
}

fn insert(creator: &Creator) -> Result<()> {
    let mut connection = establish_connection()?;
    diesel::insert_into(creators::table)
        .values(creator)
        .execute(&mut connection)?;
    Ok(())
}

fn update(creator: &Creator) -> Result<()> {
    let mut connection = establish_connection()?;
    diesel::update(creator)
        .set(creator)
        .execute(&mut connection)?;
    Ok(())
}

pub fn delete(creator: &Creator) -> Result<()> {
    let mut connection = establish_connection()?;
    diesel::delete(creator).execute(&mut connection)?;
    Ok(())
}
