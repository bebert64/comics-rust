extern crate chrono;

mod comics_error;
mod entities;

pub use comics_error::{ComicsError, Result};
// pub use entities::{Issue, Comic, Book, Volume, Creator, Publisher};
pub use entities::{Creator, Issue};

pub fn run() -> Result<()> {
    let mut issue = Issue::default();
    issue.with_id(1).save()?;
    let mut issue = Issue::fetch_by_id(1)?.unwrap();
    println!("{:?}", issue);

    let mut creator = Creator::default();
    creator.with_id(1).save()?;
    let creator = Creator::fetch_by_id(1)?.unwrap();
    println!("{:?}", creator);

    issue.with_author(&creator).save()?;
    let issue = Issue::fetch_by_id(1)?.unwrap();
    println!("{:?}", issue);

    let creator = issue.artist()?;
    println!("{:?}", creator);

    let creator = issue.author()?.unwrap();
    println!("{:?}", creator);

    creator.delete()?;
    let creator = Creator::fetch_by_id(1)?;
    println!("{:?}", creator);

    issue.delete()?;
    let issue = Issue::fetch_by_id(1)?;
    println!("{:?}", issue);
    Ok(())
}
