use super::{try_or_send_err, ComicsApp};

use crate::books::{self};

use {
    actix_web::{get, http::header::ContentType, web, App, HttpResponse, Responder},
    don_error::*,
};

pub(super) fn add_services<T>(app: App<T>) -> App<T>
where
    T: ComicsApp,
{
    app.service(get_all)
        .service(get_by_id)
        .service(get_graphic_novels)
}

#[get("/api/books")]
async fn get_all() -> impl Responder {
    try_or_send_err!({
        println!("Getting books");
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&books::get_all()?)?))
    })
}

#[get("/api/graphic_novels")]
async fn get_graphic_novels() -> impl Responder {
    try_or_send_err!({
        println!("Getting graphic novels");
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&books::get_graphic_novels()?)?))
    })
}

#[get("/api/book/{id}")]
async fn get_by_id(path: web::Path<i32>) -> impl Responder {
    try_or_send_err!({
        println!("Getting book {path:?}");
        let id: i32 = path.into_inner();
        println!("Getting book {id}");
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&books::get(id)?)?))
    })
}
