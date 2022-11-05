extern crate chrono;

mod api;
mod comics_error;
mod entities;
mod file_system;

pub use comics_error::{ComicsError, Result};
// pub use entities::{Issue, Comic, Book, Volume, Creator, Publisher};
pub use entities::{Creator, Issue};

pub async fn run() -> Result<()> {
    // let issue = Issue::default();
    // let issue = issue.save()?;
    // println!("{:?}", issue);

    // let creator = Creator::fetch_from_comic_vine_with_thumbnail(40382).await?;
    // creator.save()?;
    // let creator = Creator::fetch_by_id(40382)?.unwrap();
    // println!("{:?}", creator.name);

    file_system::comics_in_dir(file_system::ROOT_DIR)?;

    // issue.delete()?;
    // let issue = Issue::fetch_by_id(1)?;
    // println!("{:?}", issue);
    Ok(())
}
