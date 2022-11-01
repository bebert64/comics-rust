mod repo;

use crate::{ComicsError, Result};
use chrono::NaiveDate;
use diesel::prelude::*;
use repo::schema::{books, creators, issues, volumes};
use serde::Deserialize;

// pub struct Comic {
//     id: i32,
//     pub title: String,
//     pub thumbnail: Option<Vec<u8>>,
//     publisher_id: Option<i32>,
//     comic_vine_id: Option<i32>,
// }

#[derive(Debug, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Volume {
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    pub number: i32,
    pub thumbnail: Option<Vec<u8>>,
    comic_id: i32,
    comic_vine_id: Option<i32>,
}

// pub struct StoryArc {
//     id: i32,
//     pub title: String,
//     pub thumbnail: Option<Vec<u8>>,
//     comic_vine_id: Option<i32>,
// }

#[derive(Debug, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Book {
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    pub title: String,
    pub thumbnail: Option<Vec<u8>>,
    pub is_tpb: bool,
}

#[derive(Debug, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Issue {
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    pub is_read: bool,
    pub number: i32,
    pub cover_date: Option<NaiveDate>,
    pub thumbnail: Option<Vec<u8>>,
    pub comic_vine_id: Option<i32>,
    volume_id: Option<i32>,
    book_id: Option<i32>,
    author_id: Option<i32>,
    artist_id: Option<i32>,
}

impl Issue {
    pub fn fetch_by_id(id: i32) -> Result<Option<Issue>> {
        repo::issue::fetch_by_id(id)
    }

    pub fn with_volume(&mut self, volume: &Volume) -> Result<&mut Self> {
        self.volume_id = Some(volume.id.ok_or(ComicsError::NoIdError)?);
        Ok(self)
    }

    pub fn with_book(&mut self, book: &Book) -> Result<&mut Self> {
        self.book_id = Some(book.id.ok_or(ComicsError::NoIdError)?);
        Ok(self)
    }

    pub fn with_author(&mut self, creator: &Creator) -> Result<&mut Self> {
        self.author_id = Some(creator.id);
        Ok(self)
    }

    pub fn author(&self) -> Result<Option<Creator>> {
        Ok(match self.author_id {
            Some(id) => Some(Creator::fetch_by_id(id)?.ok_or(ComicsError::ForeignKeyError)?),
            None => None,
        })
    }

    pub fn with_artist(&mut self, creator: &Creator) -> Result<&mut Self> {
        self.artist_id = Some(creator.id);
        Ok(self)
    }

    pub fn artist(&self) -> Result<Option<Creator>> {
        Ok(match self.artist_id {
            Some(id) => Some(Creator::fetch_by_id(id)?.ok_or(ComicsError::ForeignKeyError)?),
            None => None,
        })
    }

    pub fn save(self) -> Result<Issue> {
        repo::issue::save(self)
    }

    pub fn delete(&self) -> Result<()> {
        repo::issue::delete(self)
    }
}

// pub struct Publisher {
//     id: i32,
//     pub name: String,
//     pub thumbnail: Option<Vec<u8>>,
//     comic_vine_id: Option<i32>,
// }

#[derive(Debug, Deserialize, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Creator {
    pub id: i32,
    pub name: String,
    pub thumbnail: Option<Vec<u8>>,
}

impl Creator {
    pub fn fetch_by_id(id: i32) -> Result<Option<Creator>> {
        repo::creator::fetch_by_id(id)
    }

    pub fn with_thumbnail(&mut self, thumbnail: Vec<u8>) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn save(&self) -> Result<()> {
        repo::creator::save(self)
    }

    pub fn delete(&self) -> Result<()> {
        repo::creator::delete(self)
    }
}
