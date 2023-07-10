use {
    std::{
        fs::{create_dir_all, File},
        io::copy,
    },
    walkdir::WalkDir,
};

fn main() -> anyhow::Result<()> {
    println!("Starting");
    let root_dir = "/home/romain/Mnt/NAS/Comics/Fini/";
    let walk_dir = WalkDir::new(root_dir).into_iter();
    let mut counter = 0;
    for entry in walk_dir.filter_entry(|e| {
        !(e.file_type().is_dir()
            && e.file_name()
                .to_str()
                .is_some_and(|s| s == "14 Planet of the Apes issues" || s.ends_with("-unzip")))
    }) {
        let entry = entry?;
        if entry.file_type().is_file() {
            if let Some(file_name) = entry.file_name().to_str().filter(|s| {
                s.ends_with(".cbr")
                    || s.ends_with(".cbz")
                    || s.ends_with(".zip")
                    || s.ends_with(".rar")
            }) {
                println!("{file_name:?}");
                let zip_file = File::open(entry.path())?;
                let mut archive = zip::ZipArchive::new(zip_file).unwrap();

                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();
                    let outpath = match file.enclosed_name() {
                        Some(path) => entry
                            .path()
                            .parent()
                            .expect("Should have a parent")
                            .join(format!("{file_name}-unzip/"))
                            .join(path.to_owned()),
                        None => continue,
                    };
                    if !outpath.exists() {
                        if (*file.name()).ends_with('/') {
                            println!("File {} extracted to \"{}\"", i, outpath.display());
                            create_dir_all(&outpath).unwrap();
                        } else {
                            println!(
                                "File {} extracted to \"{}\" ({} bytes)",
                                i,
                                outpath.display(),
                                file.size()
                            );
                            if let Some(p) = outpath.parent() {
                                if !p.exists() {
                                    create_dir_all(p).unwrap();
                                }
                            }
                            let mut outfile = File::create(&outpath).unwrap();
                            copy(&mut file, &mut outfile).unwrap();
                        }
                    }
                }
                counter += 1;
            }
            if counter % 100 == 0 {
                println!("counter : {counter}");
            }
        }
    }
    println!("Total : {counter}");
    Ok(())
}
