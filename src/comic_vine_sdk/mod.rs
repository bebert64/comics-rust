pub(crate) mod volumes;

use crate::config::CONFIG;

use {
    don_error::*,
    reqwest::{Client, ClientBuilder},
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

fn client() -> DonResult<Client> {
    Ok(ClientBuilder::new().user_agent("RustComicApp").build()?)
}

async fn get<T>(endpoint: &str, query: &str) -> DonResult<T>
where
    T: DeserializeOwned,
{
    let temp = client()?
        .get(&format!(
            "{}/{endpoint}/?api_key={}&format=json&{query}",
            CONFIG.comic_vine.url_root, CONFIG.comic_vine.api_key
        ))
        .send()
        .await?;
    let test = temp.json::<Response<Vec<serde_json::Value>>>().await?;
    println!(
        "{:#?}",
        test.results
            .iter()
            .map(|res| res.get("description"))
            .collect::<Vec<_>>()
    );
    let temp = client()?
        .get(&format!(
            "{}/{endpoint}/?api_key={}&format=json&{query}",
            CONFIG.comic_vine.url_root, CONFIG.comic_vine.api_key
        ))
        .send()
        .await?;
    println!("{temp:#?}");
    Ok(temp.json::<Response<T>>().await?.results)
    // Ok(client()?
    //     .get(&format!(
    //         "{}/{endpoint}/?api_key={}&format=json&{query}",
    //         CONFIG.comic_vine.url_root, CONFIG.comic_vine.api_key
    //     ))
    //     .send()
    //     .await?
    //     .json::<Response<T>>()
    //     .await?
    //     .results)
}

async fn get_thumbnail(url: &str) -> DonResult<Vec<u8>> {
    Ok(client()?.get(url).send().await?.bytes().await?.to_vec())
}
