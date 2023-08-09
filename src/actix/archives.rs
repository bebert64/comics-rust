use crate::{
    data_recovery::{
        parse_existing_dir::{parse_dir, PARSE_METHODS},
        Archive, ArchiveStatus, ParsedDir,
    },
    diesel_helpers::db,
    schema,
};

use {
    actix_web::{get, http::header::ContentType, HttpRequest, HttpResponse, Responder},
    diesel::prelude::*,
};

#[get("/api/archives")]
async fn get_archives() -> impl Responder {
    let mut db = db().expect("DB available");
    let archives = schema::archives::table
        .select(Archive::as_select())
        .filter(schema::archives::status.eq(ArchiveStatus::ToParse))
        .get_results(&mut db)
        .unwrap();
    let body = serde_json::to_string(&archives.iter().collect::<Vec<_>>()).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[get("/api/archives/parse_methods")]
async fn parse_methods() -> impl Responder {
    let body = serde_json::to_string(&PARSE_METHODS.keys().collect::<Vec<_>>()).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}

#[derive(Deserialize, Debug)]
struct ParseQuery {
    ids: Vec<i32>,
}

#[derive(Serialize)]
struct ParsedArchive {
    id: i32,
    result: ParsedDir,
}

#[get("/api/archives/parse")]
async fn parse(req: HttpRequest) -> impl Responder {
    let query: ParseQuery = serde_qs::from_str(req.query_string()).unwrap();
    let mut db = db().unwrap();
    let archives = schema::archives::table
        .select(Archive::as_select())
        .filter(schema::archives::id.eq_any(&query.ids))
        .get_results(&mut db)
        .unwrap();
    let parsed_archives = archives
        .into_iter()
        .map(|archive| ParsedArchive {
            id: archive.id,
            result: parse_dir(&archive.into_comics_dir().unwrap()).unwrap(),
        })
        .collect::<Vec<_>>();
    let body = serde_json::to_string(&parsed_archives).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(body)
}
