use diesel_derive_enum::DbEnum;

use crate::{comics_root_path, schema::archives};

use {diesel::prelude::*, don_error::*, std::path::PathBuf};

#[derive(Debug, Serialize)]
pub(crate) enum BookTypeOther {
    // Issue(Issue),
    GraphicNovel(GraphicNovel),
    SingleVolume(SingleVolume),
    // MultiVolume(MultiVolume),
}

// #[derive(Debug, DbEnum, Serialize)]
// pub(crate) enum BookType {
//     // Issue(Issue),
//     GraphicNovel,
//     SingleVolume,
//     // MultiVolume(MultiVolume),
// }

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
pub(crate) struct SingleVolume {
    pub(crate) volume: String,
    pub(crate) volume_number: Option<usize>,
    pub(crate) title: Option<String>,
    pub(crate) issues_sorted: Vec<Issue>,
    pub(crate) additional_files_sorted: Vec<PathBuf>,
    pub(crate) path: PathBuf,
}

#[derive(Debug, Serialize)]
pub(crate) struct MultiVolume {
    pub(crate) title: Option<String>,
    pub(crate) issues_sorted: Vec<Issue>,
    pub(crate) additional_files_sorted: Option<Vec<PathBuf>>,
    pub(crate) path: PathBuf,
}

#[derive(Debug, Serialize)]
pub(crate) struct FilesAndSubdirs {
    pub(crate) files: Vec<PathBuf>,
    pub(crate) subdirs: Vec<PathBuf>,
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

impl Archive {
    pub(crate) fn to_comics_dir(&self) -> DonResult<PathBuf> {
        let comics_root = comics_root_path(Some("Comics"))?;
        Ok(comics_root.join({
            let mut subdir = self.path.clone();
            subdir.truncate(self.path.len() - 4);
            subdir
        }))
    }
}
