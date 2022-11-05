use std::path::PathBuf;

use super::{repo::establish_connection, schema::issues};
use super::{Book, Creator, Volume};
use crate::{ComicsError, Result};
use chrono::NaiveDate;
use diesel::prelude::*;

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
    #[diesel(column_name = "path")]
    path_as_string: Option<String>,
}

impl Issue {
    pub fn fetch_by_id(id: i32) -> Result<Option<Issue>> {
        repo_issue::fetch_by_id(id)
    }

    pub fn with_volume(&mut self, volume: &Volume) -> Result<&mut Self> {
        self.volume_id = Some(volume.id);
        Ok(self)
    }

    // pub fn with_book(&mut self, book: &Book) -> Result<&mut Self> {
    //     self.book_id = Some(book.id);
    //     Ok(self)
    // }

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

    pub fn save(&mut self) -> Result<()> {
        repo_issue::save(self)
    }

    pub fn delete(&self) -> Result<()> {
        repo_issue::delete(self)
    }

    pub fn path(&self) -> Option<PathBuf> {
        if let Some(path) = &self.path_as_string {
            Some(PathBuf::from(path))
        } else {
            None
        }
    }
}

mod repo_issue {
    use super::{establish_connection, issues, Issue};
    use crate::{ComicsError, Result};
    use diesel::prelude::*;

    pub fn fetch_by_id(id: i32) -> Result<Option<Issue>> {
        let mut connection = establish_connection()?;
        Ok(issues::table
            .filter(issues::id.eq(id))
            .first::<Issue>(&mut connection)
            .optional()?)
    }

    pub fn save(issue: &mut Issue) -> Result<()> {
        match issue.id {
            Some(_) => update(issue),
            None => insert(issue),
        }
    }

    fn insert(issue: &mut Issue) -> Result<()> {
        let mut connection = establish_connection()?;
        let id = diesel::insert_into(issues::table)
            .values(&*issue)
            .returning(issues::id)
            .get_result(&mut connection)?;
        issue.id = Some(id);
        Ok(())
    }

    fn update(issue: &mut Issue) -> Result<()> {
        let mut connection = establish_connection()?;
        let id = issue.id.unwrap();
        diesel::update(issues::table.find(id))
            .set(&*issue)
            .execute(&mut connection)?;
        Ok(())
    }

    pub fn delete(issue: &Issue) -> Result<()> {
        let id = issue.id.ok_or(ComicsError::NoIdError)?;
        let mut connection = establish_connection()?;
        diesel::delete(issues::table.find(id)).execute(&mut connection)?;
        Ok(())
    }
}
