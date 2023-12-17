use crate::{
    data_recovery::structs::{Archive, ArchiveStatus},
    db::db,
    schema,
};

use {diesel::prelude::*, don_error::*};

pub(crate) fn get_to_parse() -> DonResult<Vec<Archive>> {
    let mut db = db()?;
    Ok(schema::archives::table
        .select(Archive::as_select())
        .filter(schema::archives::status.eq(ArchiveStatus::ToParse))
        .get_results(&mut db)?)
}
