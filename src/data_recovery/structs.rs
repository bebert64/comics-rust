use diesel_derive_enum::DbEnum;

use crate::schema::archives;

use {diesel::prelude::*, std::path::PathBuf};

#[derive(Debug, Serialize)]
pub(crate) struct Issue {
    pub(crate) volume: String,
    pub(crate) number: usize,
    pub(crate) path: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
pub(crate) struct GraphicNovel {
    pub(crate) title: String,
    pub(crate) path: PathBuf,
}

#[derive(Debug, Serialize)]
pub(crate) struct Book {
    pub(crate) volume: Option<String>,
    pub(crate) volume_number: Option<usize>,
    pub(crate) title: Option<String>,
    pub(crate) issues_sorted: Vec<Issue>,
    pub(crate) additional_files_sorted: Vec<PathBuf>,
    pub(crate) position_in_reading_order: Option<PositionInReadingOrder>,
    pub(crate) path: PathBuf,
}

#[derive(Debug, Serialize)]
pub(crate) struct FilesAndSubdirs {
    pub(crate) files: Vec<PathBuf>,
    pub(crate) subdirs: Vec<PathBuf>,
}

#[derive(PartialEq, Eq, Debug, Serialize)]
pub(crate) struct PositionInReadingOrder {
    pub(crate) position: usize,
    pub(crate) reading_order: String,
}

#[derive(Queryable, Selectable, Serialize)]
pub(crate) struct Archive {
    pub(crate) id: i32,
    pub(crate) path: String,
    pub(crate) status: ArchiveStatus,
}

#[derive(Debug, Clone, Copy, DbEnum, PartialEq, Eq, Hash, Serialize)]
pub(crate) enum ArchiveStatus {
    ToParse,
    ToParseIssues,
    ToCompleteIssues,
    ToSearchComicVineId,
    Ok,
}
