mod repo;

use crate::{ComicsError, Result};
use chrono::NaiveDate;
use diesel::prelude::*;
use repo::schema::{creators, issues};

// pub struct Comic {
//     id: i32,
//     pub title: String,
//     pub thumbnail: Option<Vec<u8>>,
//     publisher_id: Option<i32>,
//     comic_vine_id: Option<i32>,
// }

// pub struct Volume {
//     id: i32,
//     pub number: i32,
//     pub thumbnail: Option<Vec<u8>>,
//     comic_id: i32,
//     comic_vine_id: Option<i32>,
// }

// pub struct StoryArc {
//     id: i32,
//     pub title: String,
//     pub thumbnail: Option<Vec<u8>>,
//     comic_vine_id: Option<i32>,
// }

// pub struct Book {
//     id: i32,
//     pub title: String,
//     pub thumbnail: Option<Vec<u8>>,
//     pub is_tpb: bool,
// }

#[derive(Debug, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Issue {
    id: i32,
    pub is_read: bool,
    pub number: i32,
    pub cover_date: Option<NaiveDate>,
    pub thumbnail: Option<Vec<u8>>,
    volume_id: Option<i32>,
    comic_vine_id: Option<i32>,
    book_id: Option<i32>,
    author_id: Option<i32>,
    artist_id: Option<i32>,
}

impl Issue {
    pub fn fetch_by_id(id: i32) -> Result<Option<Issue>> {
        repo::issue::fetch_by_id(id)
    }

    pub fn with_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn with_author(&mut self, creator: &Creator) -> &mut Self {
        self.author_id = Some(creator.id);
        self
    }

    pub fn with_artist(&mut self, creator: &Creator) -> &mut Self {
        self.artist_id = Some(creator.id);
        self
    }

    pub fn author(&self) -> Result<Option<Creator>> {
        Ok(match self.author_id {
            Some(id) => Some(Creator::fetch_by_id(id)?.ok_or(ComicsError::ForeignKeyError)?),
            None => None,
        })
    }

    pub fn artist(&self) -> Result<Option<Creator>> {
        Ok(match self.artist_id {
            Some(id) => Some(Creator::fetch_by_id(id)?.ok_or(ComicsError::ForeignKeyError)?),
            None => None,
        })
    }

    pub fn save(&self) -> Result<()> {
        if self.id == 0 {
            return Err(ComicsError::SavingDefaultError);
        }
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

#[derive(Debug, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Creator {
    id: i32,
    pub name: String,
    pub thumbnail: Option<Vec<u8>>,
    comic_vine_id: Option<i32>,
}

impl Creator {
    pub fn fetch_by_id(id: i32) -> Result<Option<Creator>> {
        repo::creator::fetch_by_id(id)
    }

    pub fn with_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn with_thumbnail(&mut self, thumbnail: Vec<u8>) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn with_comic_vine_id(&mut self, id: i32) -> &mut Self {
        self.comic_vine_id = Some(id);
        self
    }

    pub fn save(&self) -> Result<()> {
        if self.id == 0 {
            return Err(ComicsError::SavingDefaultError);
        }
        repo::creator::save(self)
    }

    pub fn delete(&self) -> Result<()> {
        repo::creator::delete(self)
    }
}

impl Default for Creator {
    fn default() -> Creator {
        Creator {
            id: 0,
            name: "Unkown creator".to_string(),
            thumbnail: None,
            comic_vine_id: None,
        }
    }
}
