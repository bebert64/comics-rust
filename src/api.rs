mod http_request;

use crate::Creator;
use crate::Result;
use http_request::get_json;
use serde::Deserialize;

use self::http_request::get_thumbnail;

// TODO : move KEY to config
const API_KEY: &str = "227a2e97d111639648ed7187bc54f0970a065444";
const API_ROOT: &str = "https://comicvine.gamespot.com/api";

#[derive(Deserialize, Debug)]
struct CreatorResponse {
    results: CreatorResult,
}

#[derive(Deserialize, Debug)]
struct CreatorResult {
    id: i32,
    name: String,
    image: ImageMap,
}

#[derive(Deserialize, Debug)]
struct ImageMap {
    thumb_url: String,
}

impl CreatorResponse {
    async fn fetch_data_from_comic_vine(id: i32) -> Result<CreatorResponse> {
        let url = format!(
            "{API_ROOT}/person/4040-{id}/?api_key={API_KEY}&format=json&field_list=name,id,image"
        );
        let response: CreatorResponse = get_json(&url).await?;
        Ok(response)
    }

    async fn fetch_thumbnail_from_comic_vine(&self) -> Result<Vec<u8>> {
        let url = &self.results.image.thumb_url;
        Ok(get_thumbnail(url).await?)
    }

    fn into_entity(self) -> Creator {
        Creator {
            id: self.results.id,
            name: self.results.name,
            thumbnail: None,
        }
    }
}

pub async fn fetch_creator_from_comic_vine_with_thumbnail(id: i32) -> Result<Creator> {
    let response = CreatorResponse::fetch_data_from_comic_vine(id).await?;
    let thumbnail = response.fetch_thumbnail_from_comic_vine().await?;
    let mut creator = response.into_entity();
    creator.with_thumbnail(thumbnail);
    Ok(creator)
}

pub async fn fetch_creator_from_comic_vine_without_thumbnail(id: i32) -> Result<Creator> {
    let response = CreatorResponse::fetch_data_from_comic_vine(id).await?;
    Ok(response.into_entity())
}
