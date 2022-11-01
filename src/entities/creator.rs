use super::{repo::establish_connection, schema::creators};
use crate::Result;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Creator {
    pub id: i32,
    pub name: String,
    pub thumbnail: Option<Vec<u8>>,
}

impl Creator {
    pub fn fetch_by_id(id: i32) -> Result<Option<Creator>> {
        repo_creator::fetch_by_id(id)
    }

    pub fn with_thumbnail(&mut self, thumbnail: Vec<u8>) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn save(&self) -> Result<()> {
        repo_creator::save(self)
    }

    pub fn delete(&self) -> Result<()> {
        repo_creator::delete(self)
    }

    pub async fn fetch_from_comic_vine_with_thumbnail(id: i32) -> Result<Creator> {
        let response = api_creator::fetch_data_from_comic_vine(id).await?;
        let thumbnail = response.fetch_thumbnail_from_comic_vine().await?;
        let mut creator = response.into_entity();
        creator.with_thumbnail(thumbnail);
        Ok(creator)
    }

    pub async fn fetch_from_comic_vine_without_thumbnail(id: i32) -> Result<Creator> {
        let response = api_creator::fetch_data_from_comic_vine(id).await?;
        Ok(response.into_entity())
    }
}

mod repo_creator {
    use super::{creators, establish_connection, Creator};
    use crate::Result;
    use diesel::prelude::*;

    pub fn fetch_by_id(id: i32) -> Result<Option<Creator>> {
        let mut connection = establish_connection()?;
        Ok(creators::table
            .filter(creators::id.eq(id))
            .first::<Creator>(&mut connection)
            .optional()?)
    }

    pub fn save(creator: &Creator) -> Result<()> {
        match fetch_by_id(creator.id)? {
            Some(_) => update(creator),
            None => insert(creator),
        }
    }

    fn insert(creator: &Creator) -> Result<()> {
        let mut connection = establish_connection()?;
        diesel::insert_into(creators::table)
            .values(creator)
            .execute(&mut connection)?;
        Ok(())
    }

    fn update(creator: &Creator) -> Result<()> {
        let mut connection = establish_connection()?;
        diesel::update(creator)
            .set(creator)
            .execute(&mut connection)?;
        Ok(())
    }

    pub fn delete(creator: &Creator) -> Result<()> {
        let mut connection = establish_connection()?;
        diesel::delete(creator).execute(&mut connection)?;
        Ok(())
    }
}

mod api_creator {
    use super::Creator;
    use crate::api::{get_json, get_thumbnail, ImageMap, API_KEY, API_ROOT};
    use crate::Result;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct CreatorResponse {
        results: CreatorResult,
    }

    #[derive(Deserialize, Debug)]
    struct CreatorResult {
        id: i32,
        name: String,
        image: ImageMap,
    }
    impl CreatorResponse {
        pub fn into_entity(self) -> Creator {
            Creator {
                id: self.results.id,
                name: self.results.name,
                thumbnail: None,
            }
        }

        pub async fn fetch_thumbnail_from_comic_vine(&self) -> Result<Vec<u8>> {
            let url = &self.results.image.thumb_url;
            Ok(get_thumbnail(url).await?)
        }
    }

    pub async fn fetch_data_from_comic_vine(id: i32) -> Result<CreatorResponse> {
        let url = format!(
            "{API_ROOT}/person/4040-{id}/?api_key={API_KEY}&format=json&field_list=name,id,image"
        );
        let response: CreatorResponse = get_json(&url).await?;
        Ok(response)
    }
}
