use crate::{db::db, schema};

use {diesel::prelude::*, diesel_helpers::GetOnlyResult, don_error::*};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct BookWithIssues {
    id: i32,
    title: Option<String>,
    volume: Option<Volume>,
    volume_number: Option<i32>,
    path: String,
    issues: Vec<Issue>,
}

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = schema::books)]
pub(crate) struct Book {
    id: i32,
    title: Option<String>,
    #[diesel(embed)]
    volume: Option<Volume>,
    volume_number: Option<i32>,
    path: String,
}

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = schema::volumes)]
pub(crate) struct Volume {
    id: i32,
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Queryable, Selectable)]
#[diesel(table_name = schema::issues)]
pub(crate) struct Issue {
    id: i32,
    volume_id: i32,
    number: i32,
    path: Option<String>,
}

pub(crate) fn get_all() -> DonResult<Vec<Book>> {
    Ok(schema::books::table
        .left_join(schema::volumes::table)
        .select(Book::as_select())
        .get_results(&mut db()?)?)
}

pub(crate) fn get(id: i32) -> DonResult<BookWithIssues> {
    let book = schema::books::table
        .find(id)
        .left_join(schema::volumes::table)
        .select(Book::as_select())
        .get_only_result(&mut db()?)?;
    let issues = schema::issues::table
        .inner_join(schema::books__issues::table)
        .filter(schema::books__issues::book_id.eq(id))
        .select(Issue::as_select())
        .get_results(&mut db()?)?;
    Ok(BookWithIssues {
        id: book.id,
        title: book.title,
        volume: book.volume,
        volume_number: book.volume_number,
        path: book.path,
        issues,
    })
}
