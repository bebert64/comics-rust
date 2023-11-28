use super::{file_name, PositionInReadingOrder};

use {
    don_error::*,
    regex::{Captures, Regex},
    std::path::Path,
};

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ParsingMode {
    GraphicNovel,
    Volume,
    VolumeWithIssues,
    VolumeWithTitle,
    VolumeWithIssuesAndTitle,
}

impl ParsingMode {
    pub fn from(mode: &str) -> DonResult<ParsingMode> {
        match mode {
            "GraphicNovel" => Ok(ParsingMode::GraphicNovel),
            "Volume" => Ok(ParsingMode::Volume),
            "VolumeWithIssues" => Ok(ParsingMode::VolumeWithIssues),
            "VolumeWithTitle" => Ok(ParsingMode::VolumeWithTitle),
            "VolumeWithIssuesAndTitle" => Ok(ParsingMode::VolumeWithIssuesAndTitle),
            _ => Err(err_msg!("Invalid parsing mode")),
        }
    }
}

pub(super) struct Issue {
    pub(super) volume: Option<String>,
    pub(super) number: usize,
}

pub(super) fn parse_issue(directory: &Path) -> DonResult<Issue> {
    let captures = captures(directory, "^(?<title>.*?) (- )?(?<number>[0-9]*)$")?;
    Ok(Issue {
        volume: {
            let title = name_to_string_opt(&captures, "title")?;
            match title {
                Some(title) if &title == "Issue" => None,
                title => title,
            }
        },
        number: name_to_int(&captures, "number")?,
    })
}

#[derive(Debug, Serialize, PartialEq, Eq)]
pub(super) struct Book {
    pub(super) volume: Option<String>,
    pub(super) volume_number: Option<usize>,
    pub(super) title: Option<String>,
    pub(super) first_and_last_issue: Option<(usize, usize)>,
    pub(super) position_in_reading_order: Option<PositionInReadingOrder>,
}

pub(super) fn parse_book(directory: &Path, parsing_mode: ParsingMode) -> DonResult<Book> {
    use ParsingMode::*;
    match parsing_mode {
        GraphicNovel => Ok(Book {
            volume: None,
            volume_number: None,
            title: Some(name_to_string(
                &captures(directory, r"^(?<title>.*)$")?,
                "title",
            )?),
            first_and_last_issue: None,
            position_in_reading_order: None,
        }),
        Volume => {
            let captures = captures(
                directory,
                r"^((?<reading_order>[0-9]+)\. )?(?<volume>.*?)( v(?<volume_number>[0-9]{2}))?$",
            )?;
            Ok(Book {
                volume: Some(name_to_string(&captures, "volume")?),
                volume_number: name_to_int_opt(&captures, "volume_number")?,
                title: None,
                first_and_last_issue: None,
                position_in_reading_order: to_reading_order(&captures, &directory)?,
            })
        }
        VolumeWithIssues => {
            let captures = captures(
                directory,
                r"^((?<reading_order>[0-9]+)\. )?(?<volume>.*?)( v(?<volume_number>[0-9]{2}))? (?<first_issue>[0-9]{2})-(?<last_issue>[0-9]{2})$",
            )?;
            Ok(Book {
                volume: Some(name_to_string(&captures, "volume")?),
                volume_number: name_to_int_opt(&captures, "volume_number")?,
                title: None,
                first_and_last_issue: Some((
                    name_to_int(&captures, "first_issue")?,
                    name_to_int(&captures, "last_issue")?,
                )),
                position_in_reading_order: to_reading_order(&captures, &directory)?,
            })
        }
        VolumeWithTitle => {
            let captures = captures(
                directory,
                r"^((?<reading_order>[0-9]+)\. )?(?<volume>.*?)( v(?<volume_number>[0-9]{2}))? - (?<title>.*)$",
            )?;
            Ok(Book {
                volume: Some(name_to_string(&captures, "volume")?),
                volume_number: name_to_int_opt(&captures, "volume_number")?,
                title: Some(name_to_string(&captures, "title")?),
                first_and_last_issue: None,
                position_in_reading_order: to_reading_order(&captures, &directory)?,
            })
        }
        VolumeWithIssuesAndTitle => {
            let captures = captures(
                directory,
                r"^((?<reading_order>[0-9]+)\. )?(?<volume>.*) (?<first_issue>[0-9]{2})-(?<last_issue>[0-9]{2}) - (?<title>.*)$",
            )?;
            Ok(Book {
                volume: Some(name_to_string(&captures, "volume")?),
                volume_number: name_to_int_opt(&captures, "volume_number")?,
                title: Some(name_to_string(&captures, "title")?),
                first_and_last_issue: Some((
                    name_to_int(&captures, "first_issue")?,
                    name_to_int(&captures, "last_issue")?,
                )),
                position_in_reading_order: to_reading_order(&captures, &directory)?,
            })
        }
    }
}

fn captures<'l>(directory: &'l Path, regex: &'l str) -> DonResult<Captures<'l>> {
    let dir_name = file_name(directory)?;
    Ok(Regex::new(regex)?
        .captures(dir_name)
        .ok_or_don_err(format!(
            "No match found for {dir_name} with regex {}",
            regex
        ))?)
}

fn name_to_string(captures: &Captures, name: &str) -> DonResult<String> {
    Ok(captures
        .name(name)
        .ok_or_don_err(format!("No group named {name} captured"))?
        .as_str()
        .to_owned())
}

fn name_to_string_opt(captures: &Captures, name: &str) -> DonResult<Option<String>> {
    Ok(captures.name(name).map(|name| name.as_str().to_owned()))
}

fn name_to_int(captures: &Captures, name: &str) -> DonResult<usize> {
    Ok(captures
        .name(name)
        .ok_or_don_err("No group named {name} captured")?
        .as_str()
        .parse()
        .map_err(|err| err_msg!("{err}. captures: {captures:#?}"))?)
}

fn name_to_int_opt(captures: &Captures, name: &str) -> DonResult<Option<usize>> {
    Ok(captures
        .name(name)
        .map(|name| name.as_str().parse())
        .transpose()?)
}

fn to_reading_order(
    captures: &Captures,
    directory: &Path,
) -> DonResult<Option<PositionInReadingOrder>> {
    name_to_int_opt(&captures, "reading_order")?
        .map(|position| {
            Ok(PositionInReadingOrder {
                position,
                reading_order: file_name(
                    directory
                        .parent()
                        .ok_or_don_err("All dirs should have a parent")?,
                )?
                .to_owned(),
            })
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;
    const PARENT_DIR: &'static str = "/My Reading Order";

    #[test]
    fn test_graphic_novel() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/Graphic Novel")),
                ParsingMode::GraphicNovel
            )
            .unwrap(),
            Book {
                volume: None,
                volume_number: None,
                title: Some("Graphic Novel".to_owned()),
                first_and_last_issue: None,
                position_in_reading_order: None,
            }
        );
    }

    #[test]
    fn test_volume_alone() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/My Volume v2")),
                ParsingMode::Volume
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: None,
                title: None,
                first_and_last_issue: None,
                position_in_reading_order: None,
            }
        );
    }

    #[test]
    fn test_volume_with_reading_order() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/01. My Volume v2")),
                ParsingMode::Volume
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: None,
                title: None,
                first_and_last_issue: None,
                position_in_reading_order: Some(PositionInReadingOrder {
                    reading_order: "My Reading Order".to_owned(),
                    position: 1,
                }),
            }
        );
    }

    #[test]
    fn test_volume_with_volume_number() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/My Volume v2 v03")),
                ParsingMode::Volume
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: Some(3),
                title: None,
                first_and_last_issue: None,
                position_in_reading_order: None,
            }
        );
    }

    #[test]
    fn test_volume_with_reading_order_and_volume_number() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/01. My Volume v2 v03")),
                ParsingMode::Volume
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: Some(3),
                title: None,
                first_and_last_issue: None,
                position_in_reading_order: Some(PositionInReadingOrder {
                    reading_order: "My Reading Order".to_owned(),
                    position: 1,
                }),
            }
        );
    }

    #[test]
    fn test_volume_with_issues() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/My Volume v2 04-05")),
                ParsingMode::VolumeWithIssues
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: None,
                title: None,
                first_and_last_issue: Some((4, 5)),
                position_in_reading_order: None,
            }
        );
    }

    #[test]
    fn test_volume_with_issues_and_reading_order() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/01. My Volume v2 04-05")),
                ParsingMode::VolumeWithIssues
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: None,
                title: None,
                first_and_last_issue: Some((4, 5)),
                position_in_reading_order: Some(PositionInReadingOrder {
                    reading_order: "My Reading Order".to_owned(),
                    position: 1,
                }),
            }
        );
    }

    #[test]
    fn test_volume_with_issues_and_volume_number() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/My Volume v2 v03 04-05")),
                ParsingMode::VolumeWithIssues
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: Some(3),
                title: None,
                first_and_last_issue: Some((4, 5)),
                position_in_reading_order: None,
            }
        );
    }

    #[test]
    fn test_volume_with_issues_and_reading_order_and_volume_number() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/01. My Volume v2 v03 04-05")),
                ParsingMode::VolumeWithIssues
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: Some(3),
                title: None,
                first_and_last_issue: Some((4, 5)),
                position_in_reading_order: Some(PositionInReadingOrder {
                    reading_order: "My Reading Order".to_owned(),
                    position: 1,
                }),
            }
        );
    }

    #[test]
    fn test_volume_with_title() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/My Volume v2 - My Title")),
                ParsingMode::VolumeWithTitle
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: None,
                title: Some("My Title".to_owned()),
                first_and_last_issue: None,
                position_in_reading_order: None,
            }
        );
    }

    #[test]
    fn test_volume_with_title_and_reading_order() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/01. My Volume v2 - My Title")),
                ParsingMode::VolumeWithTitle
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: None,
                title: Some("My Title".to_owned()),
                first_and_last_issue: None,
                position_in_reading_order: Some(PositionInReadingOrder {
                    reading_order: "My Reading Order".to_owned(),
                    position: 1,
                }),
            }
        );
    }

    #[test]
    fn test_volume_with_title_and_volume_number() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/My Volume v2 v03 - My Title")),
                ParsingMode::VolumeWithTitle
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: Some(3),
                title: Some("My Title".to_owned()),
                first_and_last_issue: None,
                position_in_reading_order: None,
            }
        );
    }

    #[test]
    fn test_volume_with_title_and_reading_order_and_volume_number() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/01. My Volume v2 v03 - My Title")),
                ParsingMode::VolumeWithTitle
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: Some(3),
                title: Some("My Title".to_owned()),
                first_and_last_issue: None,
                position_in_reading_order: Some(PositionInReadingOrder {
                    reading_order: "My Reading Order".to_owned(),
                    position: 1,
                }),
            }
        );
    }

    #[test]
    fn test_volume_with_issues_and_title() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/My Volume v2 04-05 - My Title")),
                ParsingMode::VolumeWithIssuesAndTitle
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: None,
                title: Some("My Title".to_owned()),
                first_and_last_issue: Some((4, 5)),
                position_in_reading_order: None,
            }
        );
    }

    #[test]
    fn test_volume_with_issues_and_title_and_reading_order() {
        assert_eq!(
            parse_book(
                Path::new(&format!("{PARENT_DIR}/01. My Volume v2 04-05 - My Title")),
                ParsingMode::VolumeWithIssuesAndTitle
            )
            .unwrap(),
            Book {
                volume: Some("My Volume v2".to_owned()),
                volume_number: None,
                title: Some("My Title".to_owned()),
                first_and_last_issue: Some((4, 5)),
                position_in_reading_order: Some(PositionInReadingOrder {
                    reading_order: "My Reading Order".to_owned(),
                    position: 1,
                }),
            }
        );
    }
}
