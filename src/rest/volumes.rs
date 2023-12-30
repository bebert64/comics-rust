use super::{async_try_or_set_err, try_or_send_err, ComicsApp};

use crate::{
    comic_vine_sdk,
    volumes::{self, Volume, VolumeRename},
};

use {
    actix_web::{get, http::header::ContentType, post, web, App, HttpResponse, Responder},
    don_error::*,
};

pub(super) fn add_services<T>(app: App<T>) -> App<T>
where
    T: ComicsApp,
{
    app.service(get_all)
        .service(get_by_id)
        .service(rename)
        .service(update)
        .service(merge)
        .service(search)
}

#[get("/volumes")]
async fn get_all() -> impl Responder {
    try_or_send_err!({
        println!("Getting volumes");
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&volumes::get_all()?)?))
    })
}

#[get("/volume/{id}")]
async fn get_by_id(path: web::Path<i32>) -> impl Responder {
    try_or_send_err!({
        println!("Getting volume {path:?}");
        let id: i32 = path.into_inner();
        println!("Getting volume {id}");
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&volumes::get(id)?)?))
    })
}

#[post("/volumes/rename")]
async fn rename(body: String) -> impl Responder {
    try_or_send_err!({
        let volume = serde_json::from_str::<VolumeRename>(&body)?;
        println!("Renaming {volume:?}");
        volumes::rename(volume)?;
        Ok(HttpResponse::Ok().finish())
    })
}

#[post("/volumes/update")]
async fn update(body: String) -> impl Responder {
    try_or_send_err!({
        let volume = serde_json::from_str::<Volume>(&body)?;
        println!("Updating {volume:?}");
        volumes::update(volume)?;
        Ok(HttpResponse::Ok().finish())
    })
}

#[post("/volumes/merge")]
async fn merge(body: String) -> impl Responder {
    try_or_send_err!({
        let ids = serde_json::from_str::<Vec<i32>>(&body)?;
        println!("Merging volumes {ids:?}");
        volumes::merge(ids)?;
        Ok(HttpResponse::Ok().finish())
    })
}

#[get("/volumes/search/{query}")]
async fn search(path: web::Path<String>) -> impl Responder {
    async_try_or_set_err!({
        println!("Searching volumes");
        let query: String = path.into_inner();
        <DonResult<_>>::Ok(HttpResponse::Ok().content_type(ContentType::json()).body(
            serde_json::to_string(&comic_vine_sdk::volumes::search(&query).await?)?,
        ))
    })
}
