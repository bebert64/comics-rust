pub(crate) mod books;
pub(crate) mod volumes;

use crate::config::CONFIG;

use {
    don_error::*,
    reqwest::{Client, ClientBuilder, Url},
    serde::{de::DeserializeOwned, Deserialize, Serialize},
};

#[derive(Debug, Deserialize)]
struct Response<T> {
    results: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ImageUrls {
    #[serde(rename(deserialize = "thumb_url"))]
    pub(crate) thumbnail_url: String,
    pub(crate) original_url: String,
}

lazy_static::lazy_static! {
    static ref COMMON_QUERY_PARAMS: [(&'static str, &'static str);2] =
        [
            ("api_key", &CONFIG.comic_vine.api_key),
            ("format", "json"),
        ];
}

fn client() -> DonResult<Client> {
    Ok(ClientBuilder::new().user_agent("RustComicApp").build()?)
}

pub(crate) fn url(endpoint: &str, query_params: &[(&str, &str)]) -> DonResult<Url> {
    let mut url = Url::parse(&CONFIG.comic_vine.url_root)?.join(endpoint)?;
    COMMON_QUERY_PARAMS
        .iter()
        .chain(query_params)
        .for_each(|(key, value)| {
            url.query_pairs_mut().append_pair(key, value);
        });
    Ok(url)
}

async fn get<T>(endpoint: &str, query_params: &[(&str, &str)]) -> DonResult<T>
where
    T: DeserializeOwned,
{
    let my_url = url(endpoint, query_params)?;
    println!("{my_url}");
    Ok(client()?
        .get(url(endpoint, query_params)?)
        .send()
        .await?
        .json::<Response<T>>()
        .await?
        .results)
}
