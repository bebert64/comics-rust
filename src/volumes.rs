use crate::{db::db, schema};

use {
    diesel::{dsl, prelude::*},
    don_error::*,
};

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = schema::volumes)]
pub(crate) struct Volume {
    id: i32,
    name: String,
}

pub(crate) fn get() -> DonResult<Vec<Volume>> {
    Ok(schema::volumes::table
        .select(Volume::as_select())
        .get_results(&mut db()?)?)
}

pub(crate) fn rename(volume: Volume) -> DonResult<()> {
    Ok(
        match dsl::update(schema::volumes::table.find(volume.id))
            .set(schema::volumes::name.eq(volume.name))
            .execute(&mut db()?)?
        {
            0 => return Err(err_msg!("volume with id {} not found", volume.id)),
            1 => (),
            n => return Err(err_msg!("{} volumes with id {}", n, volume.id)),
        },
    )
}

pub(crate) fn merge(ids: Vec<i32>) -> DonResult<()> {
    match ids.as_slice() {
        [] => (),
        [_id] => (),
        [merge_to, merge_from @ ..] => {
            dsl::update(schema::books::table.filter(schema::books::volume_id.eq_any(merge_from)))
                .set(schema::books::volume_id.eq(merge_to))
                .execute(&mut db()?)?;
            dsl::update(schema::issues::table.filter(schema::issues::volume_id.eq_any(merge_from)))
                .set(schema::issues::volume_id.eq(merge_to))
                .execute(&mut db()?)?;
            dsl::delete(schema::volumes::table.filter(schema::volumes::id.eq_any(merge_from)))
                .execute(&mut db()?)?;
        }
    };
    Ok(())
}
