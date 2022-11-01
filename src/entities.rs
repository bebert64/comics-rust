mod book;
mod creator;
mod issue;
mod repo;
mod schema;
mod volume;

pub use book::Book;
pub use creator::Creator;
pub use issue::Issue;
pub use volume::Volume;

// pub struct Comic {
//     id: i32,
//     pub title: String,
//     pub thumbnail: Option<Vec<u8>>,
//     publisher_id: Option<i32>,
//     comic_vine_id: Option<i32>,
// }

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
