#![allow(non_snake_case)]

mod archives;
mod books;
mod config;
mod data_recovery;
mod db;
mod nas;
pub mod rest;
mod schema;
mod volumes;

use don_error::*;

pub use data_recovery::{
    parse::{perform as parse_existing_dir, ParsingMode},
    rename,
};

#[macro_use]
extern crate serde_derive;

pub fn test() -> DonResult<()> {
    // println!("original: {files:?}");
    // let split = files.split("'").collect::<Vec<_>>();
    // println!("split: {split:?}");
    // let files = split
    //     .into_iter()
    //     .filter(|s| !s.trim().is_empty())
    //     .collect::<Vec<_>>();
    // println!("files: {files:?}");
    // if split.len() != 3 {
    //     panic!("Wrong number of quotes")
    // }
    // let files_inner = split.get(1).clone();
    // println!("inner: {files_inner:?}");
    // let book = data_recovery::parse::parse_dir(
    //     &comics_root_path(Some("Fini/Crosswind 01-06"))?,
    //     ParsingMode::SingleVolumeWithIssues,
    // )?;
    // println!("{book:#?}");

    // //////   remove extension from archive  //////
    // use {diesel::prelude::*, diesel_helpers::db};

    // let ids_and_paths = schema::archives::table
    //     .select((schema::archives::id, schema::archives::path))
    //     .get_results::<(i32, String)>(&mut db()?)?;
    // for (id, path) in ids_and_paths {
    //     let new_path = path.strip_suffix(".zip").unwrap_or(
    //         path.strip_suffix(".cbz")
    //             .unwrap_or(path.strip_suffix(".cbr").unwrap_or("")),
    //     );
    //     if new_path != "" {
    //         diesel::update(schema::archives::table.filter(schema::archives::id.eq(id)))
    //             .set(schema::archives::path.eq(new_path))
    //             .execute(&mut db()?)?;
    //         // ()
    //     } else {
    //         println!("path {path:?} doesn't have a valid extension");
    //     }
    // }
    // //////////////////////////////////////////////////////////////////

    //////   Move all archives from Comics OK back to Comics  //////
    // use {diesel::prelude::*, diesel_helpers::db};
    // let archives_to_move_back = schema::archives::table
    //     .select(schema::archives::path)
    //     .filter(
    //         schema::archives::status.
    // eq(data_recovery::structs::ArchiveStatus::ToSearchComicVineId),     )
    //     .get_results::<String>(&mut db()?)?;
    // println!("archives_to_move_back: {:#?}", archives_to_move_back.len());
    // for archive in archives_to_move_back {
    //     let archive_path = comics_root_path(None)?
    //         .parent()
    //         .unwrap()
    //         .join("Comics OK")
    //         .join(archive[..archive.len() - 4].to_string());
    //     if archive_path.exists() {
    //         println!(
    //             "Moving {archive_path:?} to {:?}",
    //             comics_root_path(None)?.join(archive[..archive.len() - 4].to_string())
    //         );
    //         std::fs::rename(
    //             &archive_path,
    //             comics_root_path(None)?.join(archive[..archive.len() - 4].to_string()),
    //         )?;
    //     }
    // }
    //////////////////////////////////////////////////////////////////

    println!("{}", format!("{:02}", 1));
    Ok(())
}
