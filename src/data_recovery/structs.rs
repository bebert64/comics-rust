use diesel_derive_enum::DbEnum;

use crate::{comics_root_path, schema::archives};

use {diesel::prelude::*, don_error::*, std::path::PathBuf};

#[derive(Debug, Serialize)]
pub(crate) struct Issue {
    pub(crate) volume_name: String,
    pub(crate) number: usize,
    pub(crate) path: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub(crate) struct Book {
    pub(crate) name: BookName,
    pub(crate) path: Option<PathBuf>,
    pub(crate) book_type: BookType,
    pub(crate) issues_sorted: Option<Vec<Issue>>,
    pub(crate) additional_files_sorted: Option<Vec<PathBuf>>,
}

#[derive(Debug, Serialize)]
pub(crate) enum BookName {
    FromName(String),
    FromVolume(NameFromVolume),
}

#[derive(Debug, Serialize)]
pub(crate) struct NameFromVolume {
    pub(crate) volume: String,
    pub(crate) number: usize,
    pub(crate) title: Option<String>,
}

#[derive(Debug, Serialize)]
pub(crate) enum BookType {
    GraphicNovel,
    SingleVolume,
    MultiVolume,
}

#[derive(Debug, Serialize)]
pub(crate) enum BookOrIssue {
    Issue(Issue),
    Book(Book),
}

#[derive(Debug, Serialize, Clone)]
pub(crate) enum DirectoryType {
    Issue,
    BookWithNoIssue,
    BookWithIssues,
    BookWithIssuesAndBonus,
}

#[derive(Queryable, Selectable, Serialize)]
pub(crate) struct Archive {
    pub(crate) id: i32,
    pub(crate) path: String,
    pub(crate) status: ArchiveStatus,
}

#[derive(Debug, Clone, Copy, DbEnum, PartialEq, Eq, Hash, Serialize)]
pub(crate) enum ArchiveStatus {
    ToUnzip,
    ToParse,
    ToParseIssues,
    ToCompleteIssues,
    ToSearchComicVineId,
    Ok,
}

impl Archive {
    pub(crate) fn into_comics_dir(self: &Self) -> DonResult<PathBuf> {
        let comics_root = comics_root_path(Some("Comics"))?;
        Ok(comics_root.join({
            let mut subdir = self.path.clone();
            subdir.truncate(self.path.len() - 4);
            subdir
        }))
    }
}
