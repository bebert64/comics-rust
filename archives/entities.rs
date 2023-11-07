mod book;
mod book_issue;
mod creator;
mod issue;
mod repo;
mod schema;
pub mod volume;

pub use book::{Book, BookFetcher};
pub use creator::Creator;
pub use issue::{Issue, IssueFetcher};
pub use volume::Volume;

// pub struct StoryArc {
//     id: i32,
//     pub title: String,
//     pub thumbnail: Option<Vec<u8>>,
//     comic_vine_id: Option<i32>,
// }

// pub struct Publisher {
//     id: i32,
//     pub name: String,
//     pub thumbnail: Option<Vec<u8>>,
//     comic_vine_id: Option<i32>,
// }
