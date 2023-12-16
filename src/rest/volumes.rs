use super::try_or_send_err;

use crate::{db::db, schema};

use {
    actix_web::{get, http::header::ContentType, post, HttpResponse, Responder},
    diesel::{dsl, prelude::*},
    don_error::*,
};

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = schema::volumes)]
struct Volume {
    id: i32,
    name: String,
}

#[get("/api/volumes")]
async fn get_volumes() -> impl Responder {
    try_or_send_err!({
        let mut db = db()?;
        let volumes = schema::volumes::table
            .select(Volume::as_select())
            .get_results(&mut db)?;
        println!("found {} volumes", volumes.len());
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&volumes.iter().collect::<Vec<_>>())?))
    })
}

#[post("/api/volumes/rename")]
async fn rename_volume(request_body: String) -> impl Responder {
    try_or_send_err!({
        let volume = serde_json::from_str::<Volume>(&request_body)?;
        println!("{volume:?}");
        match dsl::update(schema::volumes::table.find(volume.id))
            .set(schema::volumes::name.eq(volume.name))
            .execute(&mut db()?)?
        {
            0 => return Err(err_msg!("volume with id {} not found", volume.id)),
            1 => (),
            n => return Err(err_msg!("{} volumes with id {}", n, volume.id)),
        };
        Ok(HttpResponse::Ok().finish())
    })
}

#[post("/api/volumes/merge")]
async fn merge_volumes(request_body: String) -> impl Responder {
    try_or_send_err!({
        let ids = serde_json::from_str::<Vec<i32>>(&request_body)?;
        match ids.as_slice() {
            [] => (),
            [_id] => (),
            [merge_to, merge_from @ ..] => {
                dsl::update(
                    schema::books::table.filter(schema::books::volume_id.eq_any(merge_from)),
                )
                .set(schema::books::volume_id.eq(merge_to))
                .execute(&mut db()?)?;
                dsl::update(
                    schema::issues::table.filter(schema::issues::volume_id.eq_any(merge_from)),
                )
                .set(schema::issues::volume_id.eq(merge_to))
                .execute(&mut db()?)?;
                dsl::delete(schema::volumes::table.filter(schema::volumes::id.eq_any(merge_from)))
                    .execute(&mut db()?)?;
            }
        }
        println!("Done");
        Ok(HttpResponse::Ok().finish())
    })
}
