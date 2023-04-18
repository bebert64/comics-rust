use std::path::{Path, PathBuf};

use super::{repo::establish_connection, schema::books, Issue};
use crate::Result;
use chrono::NaiveDate;
use diesel::prelude::*;
pub use repo_book::BookFetcher;

#[derive(Debug, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Book {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub is_read: bool,
    pub title: String,
    pub cover_date: Option<NaiveDate>,
    pub thumbnail: Option<Vec<u8>>,
    pub comic_vine_id: Option<i32>,
    pub is_tpb: bool,
    author_id: Option<i32>,
    artist_id: Option<i32>,
    #[diesel(column_name = "path")]
    pub path_as_string: Option<String>,
}

impl Book {
    pub fn new() -> Book {
        Book::default()
    }

    pub fn fetch_by_id(id: i32) -> Result<Option<Book>> {
        repo_book::fetch_by_id(id)
    }

    pub fn fetch_all() -> Result<Vec<Book>> {
        repo_book::fetch_all()
    }

    pub fn with_thumbnail(&mut self, thumbnail: Vec<u8>) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn with_path(&mut self, path: &Path) -> &mut Self {
        self.path_as_string = path.to_str().map(|s| s.to_string());
        self
    }

    pub fn path(&self) -> Option<PathBuf> {
        if let Some(path) = &self.path_as_string {
            Some(PathBuf::from(path))
        } else {
            None
        }
    }

    pub fn with_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    pub fn save(&mut self) -> Result<()> {
        repo_book::save(self)
    }

    pub fn delete(&self) -> Result<()> {
        repo_book::delete(self)
    }

    pub async fn fetch_from_comic_vine(id: i32) -> Result<Book> {
        let response = api_book::fetch_data_from_comic_vine(id).await?;
        let thumbnail = response.fetch_thumbnail_from_comic_vine().await?;
        let mut book = response.into_entity();
        book.with_thumbnail(thumbnail);
        Ok(book)
    }

    pub async fn update_from_comic_vine(&mut self, comic_vine_id: i32) -> Result<&mut Self> {
        let book_comic_vine = Book::fetch_from_comic_vine(comic_vine_id).await?;
        self.cover_date = book_comic_vine.cover_date;
        self.thumbnail = book_comic_vine.thumbnail;
        self.comic_vine_id = Some(comic_vine_id);
        self.author_id = book_comic_vine.author_id;
        self.artist_id = book_comic_vine.artist_id;
        Ok(self)
    }

    pub fn add_issues(&self, issues: &mut Vec<Issue>) -> Result<()> {
        for issue in issues.iter_mut() {
            issue.with_book_id(self.id.unwrap()).save()?;
        }
        Ok(())
    }

    pub fn add_issues_mut(&self, issues: &mut Vec<&mut Issue>) -> Result<()> {
        for issue in issues.iter_mut() {
            issue.with_book_id(self.id.unwrap()).save()?;
        }
        Ok(())
    }
}

mod repo_book {
    use super::{books, establish_connection, Book};
    use crate::{ComicsError, Result};
    use diesel::{
        prelude::*,
        result::{DatabaseErrorKind, Error::DatabaseError},
    };

    pub fn fetch_by_id(id: i32) -> Result<Option<Book>> {
        let mut connection = establish_connection()?;
        Ok(books::table
            .filter(books::id.eq(id))
            .first::<Book>(&mut connection)
            .optional()?)
    }

    pub fn fetch_all() -> Result<Vec<Book>> {
        let mut connection = establish_connection()?;
        Ok(books::table.load::<Book>(&mut connection)?)
    }

    pub fn save(book: &mut Book) -> Result<()> {
        match book.id {
            Some(_) => update(book),
            None => insert(book),
        }
    }

    fn insert(book: &mut Book) -> Result<()> {
        let mut connection = establish_connection()?;
        match diesel::insert_into(books::table)
            .values(&*book)
            .returning(books::id)
            .get_result(&mut connection)
        {
            Ok(id) => {
                book.id = Some(id);
                Ok(())
            }
            Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                diesel::update(books::table.filter(books::path.eq(&book.path_as_string)))
                    .set(&*book)
                    .execute(&mut connection)?;
                Ok(())
            }
            Err(err) => {
                println!("{err:?}");
                Err(ComicsError::ForeignKeyError)
            }
        }
    }

    fn update(book: &mut Book) -> Result<()> {
        let mut connection = establish_connection()?;
        let id = book.id.unwrap();
        diesel::update(books::table.find(id))
            .set(&*book)
            .execute(&mut connection)?;
        Ok(())
    }

    pub fn delete(book: &Book) -> Result<()> {
        let id = book.id.ok_or(ComicsError::NoIdError)?;
        let mut connection = establish_connection()?;
        diesel::delete(books::table.find(id)).execute(&mut connection)?;
        Ok(())
    }

    #[derive(Default)]
    pub struct BookFetcher {
        all: bool,
        ids: Vec<i32>,
        // is_read: bool,
        // titles: Vec<String>,
        // cover_date: Option<NaiveDate>,
        // cover_date_min: Option<NaiveDate>,
        // cover_date_max: Option<NaiveDate>,
        // has_thumbnail: bool,
        comic_vine_ids: Vec<Option<i32>>,
        // is_tpb: bool,
        // author_ids: Vec<i32>,
        // artist_ids: Vec<i32>,
        // paths_as_string: Vec<String>,
    }

    impl BookFetcher {
        pub fn new() -> BookFetcher {
            BookFetcher::default()
        }

        pub fn all(&mut self) -> &mut Self {
            self.all = true;
            self
        }

        pub fn with_id(&mut self, id: i32) -> &mut Self {
            self.ids = vec![id];
            self
        }

        pub fn with_ids(&mut self, ids: Vec<i32>) -> &mut Self {
            self.ids = ids;
            self
        }

        pub fn with_comic_vine_id(&mut self, id: Option<i32>) -> &mut Self {
            self.comic_vine_ids = vec![id];
            self
        }

        pub fn with_comic_vine_ids(&mut self, ids: Vec<Option<i32>>) -> &mut Self {
            self.comic_vine_ids = ids;
            self
        }

        pub fn load(&self) -> Result<Vec<Book>> {
            let mut connection = establish_connection()?;
            if self.all {
                return Ok(books::table.load::<Book>(&mut connection)?);
            }

            let mut query = books::table.into_boxed();

            // if self.ids.len() != 0 {
            for id in self.ids.iter() {
                query = query.or_filter(books::id.eq(id));
            }
            // }

            // if self.comic_vine_ids.len() != 0 {
            for id in self.comic_vine_ids.iter() {
                if let Some(id) = id {
                    query = query.or_filter(books::comic_vine_id.eq(id));
                } else {
                    query = query.or_filter(books::comic_vine_id.is_null());
                }
            }
            // }

            Ok(query.load::<Book>(&mut connection)?)
        }
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
        cover_date: String,
        id: i32,
        image: ImageMap,
        person_credits: Vec<PersonResponse>,
    }

    #[derive(Deserialize, Debug)]
    struct PersonResponse {
        id: i32,
    }

    impl BookResponse {
        pub fn into_entity(self) -> Book {
            Book {
                cover_date: Some(
                    chrono::NaiveDate::parse_from_str(&self.results.cover_date, "%Y-%m-%d")
                        .unwrap(),
                ),
                comic_vine_id: Some(self.results.id),
                ..Book::default()
            }
        }

        pub async fn fetch_thumbnail_from_comic_vine(&self) -> Result<Vec<u8>> {
            let url = &self.results.image.thumb_url;
            Ok(get_thumbnail(url).await?)
        }
    }

    pub async fn fetch_data_from_comic_vine(id: i32) -> Result<BookResponse> {
        let url = format!(
            "{API_ROOT}/issue/4000-{id}/?api_key={API_KEY}&format=json&field_list=cover_date,id,image,person_credits"
        );
        let response: BookResponse = get_json(&url).await?;
        Ok(response)
    }
}
