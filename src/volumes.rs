use crate::{db::db, schema};

use {
    diesel::{dsl, prelude::*},
    diesel_helpers::GetOnlyResult,
    don_error::*,
};

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = schema::volumes)]
pub(crate) struct Volume {
    id: i32,
    name: String,
    comic_vine_id: Option<i32>,
    url_thumbnail: Option<String>,
    url_cover: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct VolumeRename {
    id: i32,
    name: String,
}

pub(crate) fn get_all() -> DonResult<Vec<Volume>> {
    Ok(schema::volumes::table
        .select(Volume::as_select())
        .get_results(&mut db()?)?)
}

pub(crate) fn get(id: i32) -> DonResult<Volume> {
    Ok(schema::volumes::table
        .find(id)
        .select(Volume::as_select())
        .get_only_result(&mut db()?)?)
}

pub(crate) fn rename(volume: VolumeRename) -> DonResult<()> {
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

pub(crate) fn update(volume: Volume) -> DonResult<()> {
    Ok(
        match dsl::update(schema::volumes::table.find(volume.id))
            .set((
                schema::volumes::name.eq(volume.name),
                schema::volumes::comic_vine_id.eq(volume.comic_vine_id),
                schema::volumes::url_cover.eq(volume.url_cover),
                schema::volumes::url_thumbnail.eq(volume.url_thumbnail),
            ))
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
            dsl::update(schema::issues::table.filter(schema::issues::volume_id.eq_any(merge_from)))
                .set(schema::issues::volume_id.eq(merge_to))
                .execute(&mut db()?)?;
            dsl::update(schema::books::table.filter(schema::books::volume_id.eq_any(merge_from)))
                .set(schema::books::volume_id.eq(merge_to))
                .execute(&mut db()?)?;
            dsl::delete(schema::volumes::table.filter(schema::volumes::id.eq_any(merge_from)))
                .execute(&mut db()?)?;
        }
    };
    Ok(())
}
