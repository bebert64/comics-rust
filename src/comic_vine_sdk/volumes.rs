use super::{get, ImageUrls};

use {
    don_error::*,
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Volume {
    pub(crate) id: i32,
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) description: Option<String>,
    pub(crate) image: ImageUrls,
}

pub(crate) async fn search(volume: &str) -> DonResult<Vec<Volume>> {
    Ok(get::<Vec<Volume>>("search", &format!("query={volume}&resources=volume")).await?)
}
