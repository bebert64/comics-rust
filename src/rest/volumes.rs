use super::{try_or_send_err, ComicsApp};

use crate::volumes::{self, Volume};

use {
    actix_web::{get, http::header::ContentType, post, App, HttpResponse, Responder},
    don_error::*,
};

pub(super) fn add_services<T>(app: App<T>) -> App<T>
where
    T: ComicsApp,
{
    app.service(get_all).service(rename).service(merge)
}

#[get("/api/volumes")]
async fn get_all() -> impl Responder {
    try_or_send_err!({
        println!("Getting volumes");
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&volumes::get()?)?))
    })
}

#[post("/api/volumes/rename")]
async fn rename(request_body: String) -> impl Responder {
    try_or_send_err!({
        let volume = serde_json::from_str::<Volume>(&request_body)?;
        println!("Renaming {volume:?}");
        volumes::rename(volume)?;
        Ok(HttpResponse::Ok().finish())
    })
}

#[post("/api/volumes/merge")]
async fn merge(request_body: String) -> impl Responder {
    try_or_send_err!({
        let ids = serde_json::from_str::<Vec<i32>>(&request_body)?;
        println!("Merging volumes {ids:?}");
        volumes::merge(ids)?;
        Ok(HttpResponse::Ok().finish())
    })
}
