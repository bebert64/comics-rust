use super::{get, ImageUrls};

use {
    don_error::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Book {
    pub(crate) id: i32,
    #[serde(rename(deserialize = "name"))]
    pub(crate) title: String,
    #[serde(default)]
    pub(crate) description: Option<String>,
    pub(crate) image: ImageUrls,
}

pub(crate) async fn search(book: &str) -> DonResult<Vec<Book>> {
    Ok(get::<Vec<Book>>("books", &[("filter", &format!("name:{book}"))]).await?)
}
