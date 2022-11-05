use super::{repo::establish_connection, schema::volumes};
use crate::Result;
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, Queryable, Insertable, Identifiable, AsChangeset)]
pub struct Volume {
    pub id: i32,
    pub number: i32,
    pub thumbnail: Option<Vec<u8>>,
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

    pub async fn fetch_from_comic_vine_with_thumbnail(id: i32) -> Result<Volume> {
        let response = api_volume::fetch_data_from_comic_vine(id).await?;
        let thumbnail = response.fetch_thumbnail_from_comic_vine().await?;
        let mut volume = response.into_entity();
        volume.with_thumbnail(thumbnail);
        Ok(volume)
    }

    pub async fn fetch_from_comic_vine_without_thumbnail(id: i32) -> Result<Volume> {
        let response = api_volume::fetch_data_from_comic_vine(id).await?;
        Ok(response.into_entity())
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
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct VolumeResponse {
        results: VolumeResult,
    }

    #[derive(Deserialize, Debug)]
    struct VolumeResult {
        // Todo correct fields
        id: i32,
        name: String,
        image: ImageMap,
    }

    impl VolumeResponse {
        pub fn into_entity(self) -> Volume {
            // Todo correct mapping
            Volume::default()
        }

        pub async fn fetch_thumbnail_from_comic_vine(&self) -> Result<Vec<u8>> {
            let url = &self.results.image.thumb_url;
            Ok(get_thumbnail(url).await?)
        }
    }

    pub async fn fetch_data_from_comic_vine(id: i32) -> Result<VolumeResponse> {
        let url = format!(
            //Todo insert correct url
            "{API_ROOT}/volume/4050-{id}/?api_key={API_KEY}&format=json&field_list=name,deck,id,image"
        );
        let response: VolumeResponse = get_json(&url).await?;
        Ok(response)
    }
}
