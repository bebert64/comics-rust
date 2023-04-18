use std::path::PathBuf;

use super::{repo::establish_connection, schema::issues};
use super::{Creator, Volume};
use crate::{ComicsError, Result};
use chrono::NaiveDate;
use diesel::prelude::*;

pub use repo_issue::IssueFetcher;

#[derive(Debug, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Issue {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub is_read: bool,
    pub number: i32,
    pub cover_date: Option<NaiveDate>,
    pub thumbnail: Option<Vec<u8>>,
    pub volume_id: Option<i32>,
    pub comic_vine_id: Option<i32>,
    pub book_id: Option<i32>,
    author_id: Option<i32>,
    artist_id: Option<i32>,
    #[diesel(column_name = "path")]
    path_as_string: Option<String>,
}

impl Issue {
    pub fn fetch_by_id(id: i32) -> Result<Option<Issue>> {
        repo_issue::fetch_by_id(id)
    }

    pub fn with_volume(&mut self, volume: &Volume) -> &mut Self {
        self.volume_id = Some(volume.id);
        self
    }

    pub fn with_thumbnail(&mut self, thumbnail: Vec<u8>) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn with_book_id(&mut self, book_id: i32) -> &mut Self {
        self.book_id = Some(book_id);
        self
    }

    // pub fn with_book(&mut self, book: &Book) -> Result<&mut Self> {
    //     self.book_id = Some(book.id);
    //     Ok(self)
    // }

    pub fn with_author(&mut self, creator: &Creator) -> &mut Self {
        self.author_id = Some(creator.id);
        self
    }

    pub fn author(&self) -> Result<Option<Creator>> {
        Ok(match self.author_id {
            Some(id) => Some(Creator::fetch_by_id(id)?.ok_or(ComicsError::ForeignKeyError)?),
            None => None,
        })
    }

    pub fn with_artist(&mut self, creator: &Creator) -> &mut Self {
        self.artist_id = Some(creator.id);
        self
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

    pub async fn fetch_from_comic_vine(id: i32) -> Result<Issue> {
        let response = api_issue::fetch_data_from_comic_vine(id).await?;
        let thumbnail = response.fetch_thumbnail_from_comic_vine().await?;
        let mut issue = response.into_entity();
        issue.with_thumbnail(thumbnail);
        Ok(issue)
    }
}

mod repo_issue {
    use super::{establish_connection, issues, Issue};
    use crate::{ComicsError, Result, Volume};
    use diesel::{
        prelude::*,
        result::{DatabaseErrorKind, Error::DatabaseError},
    };

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
        match diesel::insert_into(issues::table)
            .values(&*issue)
            .returning(issues::id)
            .get_result(&mut connection)
        {
            Ok(id) => {
                issue.id = Some(id);
                Ok(())
            }
            Err(DatabaseError(DatabaseErrorKind::UniqueViolation, other)) => {
                let msg = format!("{other:?}");
                println!("{msg}");
                println!(
                    "msges are equal : {}",
                    msg == "\"UNIQUE constraint failed: issues.comic_vine_id\"".to_string()
                );
                match format!("{other:?}") {
                    msg if msg == "\"UNIQUE constraint failed: issues.comic_vine_id\"" => {
                        diesel::update(
                            issues::table.filter(issues::comic_vine_id.eq(&issue.comic_vine_id)),
                        )
                        .set(&*issue)
                        .execute(&mut connection)?;
                        println!("update issue");
                        Ok(())
                    }
                    msg if msg == "\"UNIQUE constraint failed: issues.path\"" => {
                        diesel::update(
                            issues::table.filter(issues::path.eq(&issue.path_as_string)),
                        )
                        .set(&*issue)
                        .execute(&mut connection)?;
                        Ok(())
                    }
                    _ => Err(ComicsError::ForeignKeyError),
                }
            }
            Err(err) => {
                println!("{err:?}");
                Err(ComicsError::ForeignKeyError)
            }
        }
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

    #[derive(Default)]
    pub struct IssueFetcher {
        volume_id: Option<i32>,
    }

    impl IssueFetcher {
        pub fn new() -> Self {
            IssueFetcher::default()
        }

        pub fn with_volume(&mut self, volume: &Volume) -> &mut Self {
            self.volume_id = Some(volume.id);
            self
        }

        pub fn with_volume_id(&mut self, volume_id: i32) -> &mut Self {
            self.volume_id = Some(volume_id);
            self
        }

        pub fn load(&self) -> Result<Vec<Issue>> {
            let mut connection = establish_connection()?;

            let mut query = issues::table.into_boxed();

            if let Some(volume_id) = self.volume_id {
                query = query.filter(issues::volume_id.eq(volume_id))
            }

            Ok(query.load::<Issue>(&mut connection)?)
        }
    }
}

mod api_issue {
    use super::Issue;
    use crate::api::{get_json, get_thumbnail, ImageMap, API_KEY, API_ROOT};
    use crate::Result;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Debug)]
    pub struct IssueResponse {
        results: IssueResult,
    }

    #[derive(Deserialize, Debug)]
    struct IssueResult {
        cover_date: String,
        id: i32,
        image: ImageMap,
        issue_number: String,
        person_credits: Vec<PersonResponse>,
        volume: VolumeResponse,
    }

    #[derive(Deserialize, Debug)]
    struct PersonResponse {
        id: i32,
        role: String,
    }

    #[derive(Deserialize, Debug)]
    struct VolumeResponse {
        id: i32,
    }

    impl IssueResponse {
        pub fn into_entity(self) -> Issue {
            Issue {
                id: None,
                is_read: false,
                number: self.results.issue_number.parse::<i32>().unwrap_or_default(),
                thumbnail: None,
                cover_date: Some(
                    chrono::NaiveDate::parse_from_str(&self.results.cover_date, "%Y-%m-%d")
                        .unwrap(),
                ),
                comic_vine_id: Some(self.results.id),
                volume_id: Some(self.results.volume.id),
                book_id: None,
                author_id: None,
                artist_id: None,
                path_as_string: None,
            }
        }

        pub async fn fetch_thumbnail_from_comic_vine(&self) -> Result<Vec<u8>> {
            let url = &self.results.image.thumb_url;
            Ok(get_thumbnail(url).await?)
        }
    }

    pub async fn fetch_data_from_comic_vine(id: i32) -> Result<IssueResponse> {
        let url = format!(
            "{API_ROOT}/issue/4000-{id}/?api_key={API_KEY}&format=json&field_list=cover_date,id,image,issue_number,person_credits,volume"
        );
        println!("{url}");
        let response: IssueResponse = get_json(&url).await?;
        Ok(response)
    }
}
