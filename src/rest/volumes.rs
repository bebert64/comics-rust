use super::try_or_send_err;

use crate::{db::db, schema};

use {
    actix_web::{get, http::header::ContentType, HttpResponse, Responder},
    diesel::prelude::*,
    don_error::DonResult,
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
