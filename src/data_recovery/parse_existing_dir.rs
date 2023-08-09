use super::{Archive, ArchiveStatus};

use crate::{
    comics_error::{err_msg, try_or_report, ComicsResultOptionExtensions},
    diesel_helpers::db,
    nas_path, schema, ComicsResult,
};

use {
    diesel::prelude::*,
    lazy_static::lazy_static,
    std::{
        collections::HashMap,
        fs::{read_dir, remove_dir, rename},
        path::{Path, PathBuf},
    },
};

pub fn perform() -> ComicsResult<()> {
    // let mut db = db()?;
    // let archives = schema::archives::table
    //     .select(Archive::as_select())
    //     .filter(schema::archives::status.eq(ArchiveStatus::ToParse))
    //     .get_results(&mut db)?;
    // let comics_root = nas_path(Some("Comics"))?;
    // for archive in archives.into_iter() {
    //     try_or_report(|| {
    //         let parsed_dir = parse_dir(&archive.into_comics_dir()?);
    //         Ok(())
    //     })
    // }
    Ok(())
}

struct Issue {
    volume_name: String,
    number: usize,
    path: Option<PathBuf>,
}

struct Book {
    name: Option<String>,
    path: Option<PathBuf>,
    book_type: BookType,
    issues_sorted: Option<Vec<Issue>>,
    additional_files_sorted: Option<Vec<PathBuf>>,
}

enum BookType {
    GraphicNovel,
    SingleVolume,
    MultiVolume,
}

#[derive(Debug, Serialize, Clone)]
pub(crate) enum ParsedDir {
    Issue,
    BookWithNoIssue,
    BookWithIssues,
    BookWithIssuesAndBonus,
}

pub(crate) fn parse_dir(dir: &Path) -> ComicsResult<ParsedDir> {
    let (files, subdirs): (Vec<_>, Vec<_>) = read_dir(dir)?
        .into_iter()
        .map(|result| -> ComicsResult<_> { Ok(result?) })
        .collect::<ComicsResult<Vec<_>>>()?
        .into_iter()
        .partition(|elem| elem.path().is_file());

    fn remove_layers_in_subdirs(subdirs: Vec<std::fs::DirEntry>) -> ComicsResult<()> {
        subdirs
            .into_iter()
            .try_for_each(|subdir| -> ComicsResult<()> {
                remove_extra_layers(&subdir.path())?;
                Ok(())
            })
    }

    Ok(match (subdirs.len(), files.len()) {
        (0, n) if n <= 50 => ParsedDir::Issue,
        (0, _) => ParsedDir::BookWithNoIssue,
        (1, 0) => {
            remove_extra_layers(dir)?;
            use ParsedDir::*;
            match parse_dir(dir)? {
                Issue => Issue,
                BookWithNoIssue => BookWithNoIssue,
                BookWithIssues | BookWithIssuesAndBonus => {
                    return err_msg(format!("Failed to parse {dir:?}"))
                }
            }
        }
        (1, _) => return err_msg(format!("Failed to parse {dir:?}")),
        (_, 0) => {
            remove_layers_in_subdirs(subdirs)?;
            ParsedDir::BookWithIssues
        }
        (_, _) => {
            remove_layers_in_subdirs(subdirs)?;
            ParsedDir::BookWithIssuesAndBonus
        }
    })
}

fn remove_extra_layers(directory: &Path) -> ComicsResult<()> {
    let mut loop_ctrl = true;
    while loop_ctrl {
        loop_ctrl = false;
        let mut files_in_entry = read_dir(directory)?.peekable();
        if let Some(first_file) = files_in_entry.next() {
            let first_file = first_file?.path();
            if first_file.is_dir() && files_in_entry.next().is_none() {
                let dir_to_remove = first_file;
                for file in read_dir(&dir_to_remove)? {
                    let file = file?;
                    let old_file = file.path();
                    let new_file = &dir_to_remove
                        .parent()
                        .ok_or_comics_err("Should have a parent")?
                        .to_path_buf()
                        .join(PathBuf::from(
                            old_file
                                .file_name()
                                .ok_or_comics_err("Should have a name")?
                                .to_str()
                                .ok_or_comics_err("Should have a valid name")?,
                        ));
                    println!("Moving {:#?} to {:#?}", &old_file, &new_file);
                    rename(old_file, new_file)?;
                }
                remove_dir(&dir_to_remove)?;
                loop_ctrl = true;
            }
        }
    }
    Ok(())
}

lazy_static! {
    pub(crate) static ref PARSE_METHODS: HashMap<&'static str, &'static str> =
        HashMap::from([("test", "my_regex"), ("test_2", "my_other_regex")]);
}
