use crate::{Book, Result};
use regex::Regex;

pub fn parse_name_for_volume(name: &str) -> Result<Option<String>> {
    let re = Regex::new("(.*)(?:.zip|.rar|.cbr|.cbz)")?;
    Ok(re.captures(name).map(|c| c[1].to_string()))
}

pub fn parse_name_for_volume_and_tpb_number(name: &str) -> Result<Option<(String, i32)>> {
    let re = Regex::new(r"(.*) v([0-9]{2}) - .*").unwrap();
    Ok(re
        .captures(name)
        .map(|c| (c[1].to_string(), c[2].to_string().parse().unwrap())))
}

pub fn parse_description_for_collected_volume_id(description: &str) -> Option<i32> {
    println!("{description}");
    let re = Regex::new("(?:C|c)ollected in .*data-ref-id=\"4050-([0-9]*)\"").unwrap();
    re.captures(description)
        .map(|c| c[1].to_string().parse().expect("error parsing match"))
}

pub fn parse_description_for_collected_editions(description: &str) -> Vec<(i32, Vec<i32>)> {
    // println!("{description}");
    let re = Regex::new("(?:C|c)ollected (?:e|E)ditions.*").unwrap();
    let collected_editions = re.captures(description);
    // println!("From search_collected_volumes : collected_editions = {collected_editions:?}");
    if let Some(capture) = collected_editions {
        let description = capture[0].to_string();
        // println!("{description}");
        // let re = regex::escape("data-ref-id=\"4050-([0-9]+)\".*?\x28#([0-9]+)-([0-9]+)");
        // println!("regex : {re}");
        // let re = "data-ref-id=\"4050-([0-9]+)\".*?\\x28#([0-9]+)-([0-9]+)";
        // println!("{re}");
        let re = Regex::new("data-ref-id=\"4050-([0-9]+)\".*?\\x28#([0-9]+)-([0-9]+)").unwrap();
        let mut results = vec![];
        // let test = re.captures(&description);
        // println!("test : {test:?}");
        for capture in re.captures_iter(&description) {
            // println!("capture : {capture:?}");
            let volume_id: i32 = capture[1].to_string().parse().unwrap();
            let issue_first: i32 = capture[2].to_string().parse().unwrap();
            let issue_last: i32 = capture[3].to_string().parse().unwrap();
            let issues: Vec<i32> = (issue_first..issue_last + 1).collect();
            results.push((volume_id, issues));
        }
        results
    } else {
        vec![]
    }
}

impl Book {
    pub fn get_folders(&self) -> Vec<String> {
        let mut path: Vec<String> = self
            .path()
            .unwrap_or_default()
            .into_os_string()
            .into_string()
            .unwrap_or_default()
            .split("/")
            .map(|s| s.to_string())
            .collect();
        path.pop();
        path.remove(0);
        path.remove(0);
        path.remove(0);
        path.remove(0);
        path
    }
}
