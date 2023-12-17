use super::{try_or_send_err, ComicsApp};

use crate::archives;

use {
    actix_web::{get, http::header::ContentType, App, HttpResponse, Responder},
    don_error::DonResult,
};

pub(super) fn add_services<T>(app: App<T>) -> App<T>
where
    T: ComicsApp,
{
    app.service(get_to_parse)
}

#[get("/api/archives")]
async fn get_to_parse() -> impl Responder {
    try_or_send_err!({
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&archives::get_to_parse()?)?))
    })
}
