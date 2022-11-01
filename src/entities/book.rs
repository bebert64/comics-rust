use super::{repo::establish_connection, schema::books};
use crate::Result;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub thumbnail: Option<Vec<u8>>,
    pub is_tpb: bool,
}

impl Book {
    pub fn fetch_by_id(id: i32) -> Result<Option<Book>> {
        repo_book::fetch_by_id(id)
    }

    pub fn with_thumbnail(&mut self, thumbnail: Vec<u8>) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn save(&self) -> Result<()> {
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
    use crate::Result;
    use diesel::prelude::*;

    pub fn fetch_by_id(id: i32) -> Result<Option<Book>> {
        let mut connection = establish_connection()?;
        Ok(books::table
            .filter(books::id.eq(id))
            .first::<Book>(&mut connection)
            .optional()?)
    }

    pub fn save(creator: &Book) -> Result<()> {
        match fetch_by_id(creator.id)? {
            Some(_) => update(creator),
            None => insert(creator),
        }
    }

    fn insert(book: &Book) -> Result<()> {
        let mut connection = establish_connection()?;
        diesel::insert_into(books::table)
            .values(book)
            .execute(&mut connection)?;
        Ok(())
    }

    fn update(book: &Book) -> Result<()> {
        let mut connection = establish_connection()?;
        diesel::update(book).set(book).execute(&mut connection)?;
        Ok(())
    }

    pub fn delete(book: &Book) -> Result<()> {
        let mut connection = establish_connection()?;
        diesel::delete(book).execute(&mut connection)?;
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
