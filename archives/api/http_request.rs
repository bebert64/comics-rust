use crate::Result;
use serde::de::DeserializeOwned;

pub async fn get_json<T: DeserializeOwned + std::fmt::Debug>(url: &str) -> Result<T> {
    Ok(build_client()?.get(url).send().await?.json().await?)
}

pub async fn get_thumbnail(url: &str) -> Result<Vec<u8>> {
    Ok(build_client()?
        .get(url)
        .send()
        .await?
        .bytes()
        .await?
        .to_vec())
}

fn build_client() -> Result<reqwest::Client> {
    let client_builder = reqwest::ClientBuilder::new().user_agent("RustComicApp");
    Ok(client_builder.build()?)
}
