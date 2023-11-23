use super::try_or_send_err;

use crate::{
    data_recovery::{
        parse::{parse_dir, ParsingMode, PARSE_METHODS},
        structs::{Archive, ArchiveStatus, BookType},
    },
    schema,
};

use {
    actix_web::{get, http::header::ContentType, HttpRequest, HttpResponse, Responder},
    diesel::prelude::*,
    diesel_helpers::db,
    don_error::DonResult,
};

#[derive(Deserialize, Debug)]
struct ParseQuery {
    ids: Vec<i32>,
    mode: ParsingMode,
}

#[derive(Serialize)]
struct ParsedArchive {
    id: i32,
    result: BookType,
}

#[get("/api/archives")]
async fn get_archives() -> impl Responder {
    try_or_send_err!({
        let mut db = db()?;
        let archives = schema::archives::table
            .select(Archive::as_select())
            .filter(schema::archives::status.eq(ArchiveStatus::ToParse))
            .get_results(&mut db)?;
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(serde_json::to_string(&archives.iter().collect::<Vec<_>>())?))
    })
}

#[get("/api/archives/parse_methods")]
async fn parse_methods() -> impl Responder {
    try_or_send_err!({
        let body = serde_json::to_string(&PARSE_METHODS.keys().collect::<Vec<_>>())?;
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body))
    })
}

#[get("/api/archives/parse")]
async fn parse(req: HttpRequest) -> impl Responder {
    try_or_send_err!({
        let query: ParseQuery = serde_qs::from_str(req.query_string())?;
        let mut db = db()?;
        let archives = schema::archives::table
            .select(Archive::as_select())
            .filter(schema::archives::id.eq_any(&query.ids))
            .get_results(&mut db)?;
        let parsed_archives = archives
            .into_iter()
            .map(|archive| -> DonResult<_> {
                Ok(ParsedArchive {
                    id: archive.id,
                    result: parse_dir(&archive.to_comics_dir()?, &query.mode)?,
                })
            })
            .collect::<DonResult<Vec<_>>>()?;
        let body = serde_json::to_string(&parsed_archives)?;
        Ok(HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body))
    })
}
