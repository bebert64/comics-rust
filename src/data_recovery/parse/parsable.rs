use {
    don_error::*,
    regex::{Captures, Regex},
    std::path::Path,
};

pub(super) trait Parsable {
    const REGEX: &'static str;

    fn parse(directory: &Path) -> DonResult<Self>
    where
        Self: Sized;

    fn regex() -> DonResult<Regex> {
        Ok(Regex::new(Self::REGEX)?)
    }

    fn captures(directory: &Path) -> DonResult<Captures> {
        let dir_name = file_name(directory)?;
        Ok(Self::regex()?.captures(dir_name).ok_or_don_err(format!(
            "No match found for {dir_name} with regex {}",
            Self::REGEX
        ))?)
    }
}

pub(super) struct Title {
    pub(super) title: String,
}

impl Parsable for Title {
    const REGEX: &'static str = "(.*)";

    fn parse(directory: &Path) -> DonResult<Title> {
        Ok(Title {
            title: get_to_string(&Self::captures(directory)?)?,
        })
    }
}

pub(super) struct VolumeAndIssue {
    pub(super) volume: String,
    pub(super) volume_number: Option<usize>,
    pub(super) first_issue: usize,
    pub(super) last_issue: usize,
}

impl Parsable for VolumeAndIssue {
    const REGEX: &'static str = "(?<volume>.*)( v(?<volume_number>[0-9]{2}))? (?<first_issue>[0-9]{2})-(?<last_issue>[0-9]{2})";

    fn parse(directory: &Path) -> DonResult<VolumeAndIssue> {
        let captures = Self::captures(directory)?;
        Ok(VolumeAndIssue {
            volume: name_to_string(&captures, "volume")?,
            volume_number: name_to_int_opt(&captures, "volume_number")?,
            first_issue: name_to_int(&captures, "first_issue")?,
            last_issue: name_to_int(&captures, "last_issue")?,
        })
    }
}

pub(super) fn file_name(directory: &Path) -> DonResult<&str> {
    Ok(directory
        .file_name()
        .ok_or_don_err("directory should have a file_name")?
        .to_str()
        .ok_or_don_err("directory should have a valid OsStr name")?)
}

fn get_to_string(captures: &Captures) -> DonResult<String> {
    Ok(captures
        .get(0)
        .ok_or_don_err("captures should never be empty")?
        .as_str()
        .to_owned())
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
        .parse()?)
}

fn name_to_int_opt(captures: &Captures, name: &str) -> DonResult<Option<usize>> {
    Ok(captures
        .name(name)
        .map(|name| name.as_str().parse())
        .transpose()?)
}
