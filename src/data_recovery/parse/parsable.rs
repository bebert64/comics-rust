use super::file_name;

use {
    don_error::*,
    regex::{Captures, Regex},
    std::path::Path,
};

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum ParsingMode {
    GraphicNovel,
    SingleVolume,
    SingleVolumeWithIssues,
    SingleVolumeWithTitle,
    // SingleVolumeWithReadingOrder,
    // SingleVolumeWithReadingOrderAndTitle,
}

impl ParsingMode {
    pub fn from(mode: &str) -> DonResult<ParsingMode> {
        match mode {
            "GraphicNovel" => Ok(ParsingMode::GraphicNovel),
            "SingleVolume" => Ok(ParsingMode::SingleVolume),
            "SingleVolumeWithIssues" => Ok(ParsingMode::SingleVolumeWithIssues),
            "SingleVolumeWithTitle" => Ok(ParsingMode::SingleVolumeWithTitle),
            _ => Err(err_msg!("Invalid parsing mode")),
        }
    }
}

pub(super) trait Parsable {
    const REGEX: &'static str;

    fn captures(directory: &Path) -> DonResult<Captures> {
        let dir_name = file_name(directory)?;
        Ok(Regex::new(Self::REGEX)?
            .captures(dir_name)
            .ok_or_don_err(format!(
                "No match found for {dir_name} with regex {}",
                Self::REGEX
            ))?)
    }

    fn parse(directory: &Path) -> DonResult<Self>
    where
        Self: Sized;
}

pub(super) struct Title {
    pub(super) title: String,
}

impl Parsable for Title {
    const REGEX: &'static str = "^(?<title>.*)";

    fn parse(directory: &Path) -> DonResult<Title> {
        Ok(Title {
            title: name_to_string(&Self::captures(directory)?, "title")?,
        })
    }
}

pub(super) struct Issue {
    pub(super) volume: Option<String>,
    pub(super) number: usize,
}

impl Parsable for Issue {
    const REGEX: &'static str = "^(?<title>.*?) (- )?(?<number>[0-9]*)$";

    fn parse(directory: &Path) -> DonResult<Issue> {
        let captures = Self::captures(directory)?;
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
}

pub(super) struct Volume {
    pub(super) volume: String,
    pub(super) volume_number: Option<usize>,
}

impl Parsable for Volume {
    const REGEX: &'static str = "^(?<volume>.*)( v(?<volume_number>[0-9]{2}))?";

    fn parse(directory: &Path) -> DonResult<Volume> {
        let captures = Self::captures(directory)?;
        Ok(Volume {
            volume: name_to_string(&captures, "volume")?,
            volume_number: name_to_int_opt(&captures, "volume_number")?,
        })
    }
}

pub(super) struct VolumeWithIssues {
    pub(super) volume: String,
    pub(super) volume_number: Option<usize>,
    pub(super) first_issue: usize,
    pub(super) last_issue: usize,
}

impl Parsable for VolumeWithIssues {
    const REGEX: &'static str = "^(?<volume>.*)( v(?<volume_number>[0-9]{2}))? (?<first_issue>[0-9]{2})-(?<last_issue>[0-9]{2})";

    fn parse(directory: &Path) -> DonResult<VolumeWithIssues> {
        let captures = Self::captures(directory)?;
        Ok(VolumeWithIssues {
            volume: name_to_string(&captures, "volume")?,
            volume_number: name_to_int_opt(&captures, "volume_number")?,
            first_issue: name_to_int(&captures, "first_issue")?,
            last_issue: name_to_int(&captures, "last_issue")?,
        })
    }
}

pub(super) struct VolumeWithTitle {
    pub(super) volume: String,
    pub(super) volume_number: Option<usize>,
    pub(super) title: String,
}

impl Parsable for VolumeWithTitle {
    const REGEX: &'static str = "^(?<volume>.*?)( v(?<volume_number>[0-9]{2}))? - (?<title>.*)";

    fn parse(directory: &Path) -> DonResult<VolumeWithTitle> {
        let captures = Self::captures(directory)?;
        Ok(VolumeWithTitle {
            volume: name_to_string(&captures, "volume")?,
            volume_number: name_to_int_opt(&captures, "volume_number")?,
            title: name_to_string(&captures, "title")?,
        })
    }
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
