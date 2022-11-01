mod http_request;

pub use http_request::{get_json, get_thumbnail};
use serde::Deserialize;

// TODO : move KEY to config
pub const API_KEY: &str = "227a2e97d111639648ed7187bc54f0970a065444";
pub const API_ROOT: &str = "https://comicvine.gamespot.com/api";

#[derive(Deserialize, Debug)]
pub struct ImageMap {
    pub thumb_url: String,
}
