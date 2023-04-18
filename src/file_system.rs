use crate::{Book, Result};
use walkdir::{DirEntry, WalkDir};

pub const ROOT_DIR: &str = "/mnt/NAS/Comics";

fn needs_skip(e: &DirEntry) -> bool {
    let shortcuts_dir = "Elseworlds";
    e.file_type().is_dir()
        && e.file_name()
            .to_str()
            .map(|s| s == shortcuts_dir)
            .unwrap_or(false)
}

fn is_comics(e: &DirEntry) -> bool {
    e.file_type().is_file()
        && e.file_name()
            .to_str()
            .map(|s| {
                s.ends_with(".cbr")
                    || s.ends_with(".cbz")
                    || s.ends_with(".zip")
                    || s.ends_with(".rar")
            })
            .unwrap_or(false)
}

pub fn comics_in_dir(dir: &str) -> Result<()> {
    println!("Starting");
    let walk_dir = WalkDir::new(dir).into_iter();
    let mut counter = 0;
    for entry in walk_dir.filter_entry(|e| !needs_skip(e)) {
        let entry = entry?;
        if is_comics(&entry) {
            let mut book = dir_into_book(&entry);
            if counter > 5 {
                book.comic_vine_id = Some(counter);
            }
            book.save()?;
            // println!("{:?}", &entry.file_name().to_str().unwrap_or("Error"));
            // println!("{:?}", &entry.path().to_str().unwrap_or("Error"));
            counter += 1;
        }
        if counter % 100 == 0 {
            println!("counter : {counter}");
        }
    }
    println!("Total : {counter}");
    Ok(())
}

pub fn dir_into_book(entry: &DirEntry) -> Book {
    let mut book = Book::new();
    book.with_path(entry.path())
        .with_title(entry.file_name().to_str().unwrap_or(""));
    book
}
