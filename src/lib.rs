extern crate chrono;

mod api;
mod comics_error;
mod entities;

pub use comics_error::{ComicsError, Result};
// pub use entities::{Issue, Comic, Book, Volume, Creator, Publisher};
pub use entities::{Creator, Issue};

pub async fn run() -> Result<()> {
    // let issue = Issue::default();
    // let issue = issue.save()?;
    // println!("{:?}", issue);

    // let creator = api::fetch_creator_from_comic_vine_with_thumbnail(40382).await?;
    // creator.save()?;
    let creator = Creator::fetch_by_id(40382)?.unwrap();
    println!("{creator:?}");

    // issue.delete()?;
    // let issue = Issue::fetch_by_id(1)?;
    // println!("{:?}", issue);
    Ok(())
}
