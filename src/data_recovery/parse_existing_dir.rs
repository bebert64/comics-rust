use super::structs::*;

use crate::{comics_root_path, schema};

use {
    diesel::prelude::*,
    diesel_helpers::db,
    don_error::*,
    lazy_static::lazy_static,
    regex::Regex,
    std::{
        collections::HashMap,
        fs::{read_dir, remove_dir, rename},
        path::{Path, PathBuf},
    },
};

// pub fn perform(mode: &ParsingMode) -> DonResult<BookOrIssue> {
pub fn perform(mode: &ParsingMode) -> DonResult<()> {
    let mut db = db()?;
    let archives = schema::archives::table
        .select(Archive::as_select())
        .filter(schema::archives::status.eq(ArchiveStatus::ToParse))
        .get_results(&mut db)?;
    let comics_root = comics_root_path(Some("Comics"))?;
    for archive in archives.into_iter() {
        try_or_report(|| {
            let book_or_issue = parse_dir(&archive.into_comics_dir()?, mode)?;
            Ok(())
        })
    }
    Ok(())
}

pub(crate) fn parse_dir(directory: &Path, mode: &ParsingMode) -> DonResult<BookOrIssue> {
    use {DirectoryType::*, ParsingMode::*};
    match (directory_type(directory)?, mode) {
        (BookWithNoIssue, Title) => {
            let regex = mode.into_regex();
        }
        _ => unimplemented!("Not yet implemented"),
    }

    let regex = mode.into_regex()?;

    Ok(BookOrIssue::Book(Book {
        name: BookName::FromName("()".to_owned()),
        path: None,
        book_type: BookType::GraphicNovel,
        issues_sorted: None,
        additional_files_sorted: None,
    }))
}

pub(crate) fn directory_type(dir: &Path) -> DonResult<DirectoryType> {
    use DirectoryType::*;
    let (files, subdirs): (Vec<_>, Vec<_>) = read_dir(dir)?
        .into_iter()
        .map(|result| -> DonResult<_> { Ok(result?) })
        .collect::<DonResult<Vec<_>>>()?
        .into_iter()
        .partition(|elem| elem.path().is_file());

    fn remove_layers_in_subdirs(subdirs: Vec<std::fs::DirEntry>) -> DonResult<()> {
        subdirs.into_iter().try_for_each(|subdir| -> DonResult<()> {
            remove_extra_layers(&subdir.path())?;
            Ok(())
        })
    }

    Ok(match (subdirs.len(), files.len()) {
        (0, n) if n <= 50 => Issue,
        (0, _) => BookWithNoIssue,
        (1, 0) => {
            remove_extra_layers(dir)?;
            use DirectoryType::*;
            match directory_type(dir)? {
                Issue => Issue,
                BookWithNoIssue => BookWithNoIssue,
                BookWithIssues | BookWithIssuesAndBonus => {
                    bail!(format!("Failed to parse {dir:?}"))
                }
            }
        }
        (1, _) => bail!(format!("Failed to parse {dir:?}")),
        (_, 0) => {
            remove_layers_in_subdirs(subdirs)?;
            BookWithIssues
        }
        (_, _) => {
            remove_layers_in_subdirs(subdirs)?;
            BookWithIssuesAndBonus
        }
    })
}

fn remove_extra_layers(directory: &Path) -> DonResult<()> {
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
                        .ok_or_don_err("Should have a parent")?
                        .to_path_buf()
                        .join(PathBuf::from(
                            old_file
                                .file_name()
                                .ok_or_don_err("Should have a name")?
                                .to_str()
                                .ok_or_don_err("Should have a valid name")?,
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

#[derive(Deserialize, Debug)]
pub enum ParsingMode {
    Title,
}

impl ParsingMode {
    fn into_regex(&self) -> DonResult<Regex> {
        use ParsingMode::*;
        Ok(match self {
            Title => Regex::new("(.*)")?,
        })
    }
}
