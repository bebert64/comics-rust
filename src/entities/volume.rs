use super::{repo::establish_connection, schema::volumes};
use crate::Result;
pub use api_volume::{search_by_name, SearchResult};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Volume {
    pub id: i32,
    pub thumbnail: Option<Vec<u8>>,
    pub title: String,
    pub publisher_id: Option<i32>,
    pub year_start: Option<i32>,
}

impl Volume {
    pub fn fetch_by_id(id: i32) -> Result<Option<Volume>> {
        repo_volume::fetch_by_id(id)
    }

    pub fn with_thumbnail(&mut self, thumbnail: Vec<u8>) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    pub fn save(&self) -> Result<()> {
        repo_volume::save(self)
    }

    pub fn delete(&self) -> Result<()> {
        repo_volume::delete(self)
    }

    pub async fn fetch_from_comic_vine(id: i32) -> Result<(Volume, Vec<i32>)> {
        let response = api_volume::fetch_data_from_comic_vine(id).await?;
        let thumbnail = response.fetch_thumbnail_from_comic_vine().await?;
        let issues = response.issues();
        let mut volume = response.into_entity();
        volume.with_thumbnail(thumbnail);
        Ok((volume, issues))
    }
}

mod repo_volume {
    use super::{establish_connection, volumes, Volume};
    use crate::Result;
    use diesel::prelude::*;

    pub fn fetch_by_id(id: i32) -> Result<Option<Volume>> {
        let mut connection = establish_connection()?;
        Ok(volumes::table
            .filter(volumes::id.eq(id))
            .first::<Volume>(&mut connection)
            .optional()?)
    }

    pub fn save(volume: &Volume) -> Result<()> {
        match fetch_by_id(volume.id)? {
            Some(_) => update(volume),
            None => insert(volume),
        }
    }

    fn insert(volume: &Volume) -> Result<()> {
        let mut connection = establish_connection()?;
        diesel::insert_into(volumes::table)
            .values(volume)
            .execute(&mut connection)?;
        Ok(())
    }

    fn update(volume: &Volume) -> Result<()> {
        let mut connection = establish_connection()?;
        diesel::update(volume)
            .set(volume)
            .execute(&mut connection)?;
        Ok(())
    }

    pub fn delete(creator: &Volume) -> Result<()> {
        let mut connection = establish_connection()?;
        diesel::delete(creator).execute(&mut connection)?;
        Ok(())
    }
}

mod api_volume {
    use super::Volume;
    use crate::api::{get_json, get_thumbnail, ImageMap, API_KEY, API_ROOT};
    use crate::Result;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Debug)]
    pub struct VolumeResponse {
        results: VolumeResult,
    }

    #[derive(Deserialize, Debug)]
    struct VolumeResult {
        deck: Option<String>,
        id: i32,
        image: ImageMap,
        issues: Vec<IssueResult>,
        name: String,
        publisher: PublisherResult,
        start_year: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    struct IssueResult {
        id: i32,
    }

    #[derive(Deserialize, Debug)]
    struct PublisherResult {
        id: i32,
    }

    impl VolumeResponse {
        pub fn into_entity(self) -> Volume {
            Volume {
                id: self.results.id,
                title: format!("{}{}", self.results.name, self.deck_as_name()),
                thumbnail: None,
                publisher_id: Some(self.results.publisher.id),
                year_start: self
                    .results
                    .start_year
                    .map(|s| s.parse::<i32>().unwrap_or_default()),
            }
        }

        pub async fn fetch_thumbnail_from_comic_vine(&self) -> Result<Vec<u8>> {
            let url = &self.results.image.thumb_url;
            Ok(get_thumbnail(url).await?)
        }

        fn deck_as_name(&self) -> String {
            // TODO : correct deck
            println!("deck : {:?}", self.results.deck);
            let deck = self.results.deck.clone().unwrap_or_default();
            if deck.trim().is_empty() {
                "".to_string()
            } else {
                format!(" v{}", deck)
            }
        }

        pub fn issues(&self) -> Vec<i32> {
            self.results.issues.iter().map(|issue| issue.id).collect()
        }
    }

    pub async fn fetch_data_from_comic_vine(id: i32) -> Result<VolumeResponse> {
        let url = format!(
            "{API_ROOT}/volume/4050-{id}/?api_key={API_KEY}&format=json&field_list=deck,id,image,issues,name,publisher,start_year"
        );
        println!("{url}");
        let response: VolumeResponse = get_json(&url).await?;
        Ok(response)
    }

    #[derive(Deserialize, Debug)]
    struct SearchResponse {
        results: Vec<SearchResult>,
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct SearchResult {
        pub count_of_issues: i32,
        pub id: i32,
        pub name: String,
        pub description: Option<String>,
    }

    pub async fn search_by_name(name: &str) -> Result<Vec<SearchResult>> {
        let url = format!("{API_ROOT}/volumes/?api_key={API_KEY}&format=json&filter=name:{name}");
        println!("{url}");
        let response: SearchResponse = get_json(&url).await?;
        let mut results = response.results;
        results.sort_by(|a, b| a.count_of_issues.partial_cmp(&b.count_of_issues).unwrap());
        results.reverse();
        Ok(results)
    }
}
