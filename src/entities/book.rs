use super::{repo::establish_connection, schema::books};
use crate::Result;
use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Debug, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Book {
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    pub is_read: bool,
    pub title: String,
    pub cover_date: Option<NaiveDate>,
    pub thumbnail: Option<Vec<u8>>,
    pub comic_vine_id: Option<i32>,
    pub is_tpb: bool,
    author_id: Option<i32>,
    artist_id: Option<i32>,
    #[diesel(column_name = "path")]
    path_as_string: Option<String>,
}

impl Book {
    pub fn fetch_by_id(id: i32) -> Result<Option<Book>> {
        repo_book::fetch_by_id(id)
    }

    pub fn with_thumbnail(&mut self, thumbnail: Vec<u8>) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn save(self) -> Result<Book> {
        repo_book::save(self)
    }

    pub fn delete(&self) -> Result<()> {
        repo_book::delete(self)
    }

    pub async fn fetch_from_comic_vine_with_thumbnail(id: i32) -> Result<Book> {
        let response = api_book::fetch_data_from_comic_vine(id).await?;
        let thumbnail = response.fetch_thumbnail_from_comic_vine().await?;
        let mut book = response.into_entity();
        book.with_thumbnail(thumbnail);
        Ok(book)
    }

    pub async fn fetch_from_comic_vine_without_thumbnail(id: i32) -> Result<Book> {
        let response = api_book::fetch_data_from_comic_vine(id).await?;
        Ok(response.into_entity())
    }
}

mod repo_book {
    use super::{books, establish_connection, Book};
    use crate::{ComicsError, Result};
    use diesel::prelude::*;

    pub fn fetch_by_id(id: i32) -> Result<Option<Book>> {
        let mut connection = establish_connection()?;
        Ok(books::table
            .filter(books::id.eq(id))
            .first::<Book>(&mut connection)
            .optional()?)
    }

    pub fn save(book: Book) -> Result<Book> {
        match book.id {
            Some(_) => update(book),
            None => insert(book),
        }
    }

    fn insert(mut book: Book) -> Result<Book> {
        let mut connection = establish_connection()?;
        let id = diesel::insert_into(books::table)
            .values(&book)
            .returning(books::id)
            .get_result(&mut connection)?;
        book.id = Some(id);
        Ok(book)
    }

    fn update(book: Book) -> Result<Book> {
        let mut connection = establish_connection()?;
        let id = book.id.unwrap();
        diesel::update(books::table.find(id))
            .set(&book)
            .execute(&mut connection)?;
        Ok(book)
    }

    pub fn delete(book: &Book) -> Result<()> {
        let id = book.id.ok_or(ComicsError::NoIdError)?;
        let mut connection = establish_connection()?;
        diesel::delete(books::table.find(id)).execute(&mut connection)?;
        Ok(())
    }
}

mod api_book {
    use super::Book;
    use crate::api::{get_json, get_thumbnail, ImageMap, API_KEY, API_ROOT};
    use crate::Result;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct BookResponse {
        results: BookResult,
    }

    #[derive(Deserialize, Debug)]
    struct BookResult {
        // Todo correct fields
        id: i32,
        name: String,
        image: ImageMap,
    }
    impl BookResponse {
        pub fn into_entity(self) -> Book {
            // Todo correct mapping
            Book::default()
        }

        pub async fn fetch_thumbnail_from_comic_vine(&self) -> Result<Vec<u8>> {
            let url = &self.results.image.thumb_url;
            Ok(get_thumbnail(url).await?)
        }
    }

    pub async fn fetch_data_from_comic_vine(id: i32) -> Result<BookResponse> {
        let url = format!(
            //Todo insert correct url
            "{API_ROOT}/person/4040-{id}/?api_key={API_KEY}&format=json&field_list=name,id,image"
        );
        let response: BookResponse = get_json(&url).await?;
        Ok(response)
    }
}
