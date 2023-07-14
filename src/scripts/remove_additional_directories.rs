use {
    std::{
        fs::{read_dir, remove_dir, rename},
        path::PathBuf,
    },
    walkdir::WalkDir,
};

pub fn perform(root_dir: &str) -> anyhow::Result<()> {
    println!("Removing {root_dir}");
    let walk_dir = WalkDir::new(root_dir).into_iter();
    let mut counter = 0;
    for entry in walk_dir.filter_entry(|e| {
        !(e.file_type().is_dir()
            && e.file_name()
                .to_str()
                .is_some_and(|s| s == "14 Planet of the Apes issues"))
    }) {
        let entry = entry?;
        if entry.file_type().is_dir()
            && entry
                .file_name()
                .to_str()
                .is_some_and(|s| s.ends_with("-unzip"))
        {
            let mut loop_ctrl = true;
            while loop_ctrl {
                loop_ctrl = false;
                let mut files_in_entry = read_dir(entry.path())?;
                if let Some(first_file) = files_in_entry.next() {
                    let first_file = first_file?;
                    if first_file.path().is_dir() && files_in_entry.next().is_none() {
                        let dir_to_remove = first_file.path();
                        for file in read_dir(&dir_to_remove)? {
                            let file = file?;
                            let old_file = file.path();
                            let new_file = &dir_to_remove
                                .parent()
                                .expect("Should have a parent")
                                .to_path_buf()
                                .join(PathBuf::from(
                                    old_file
                                        .file_name()
                                        .expect("Should have a name")
                                        .to_str()
                                        .expect("Should have a valid name"),
                                ));
                            println!("Moving {:#?} to {:#?}", &old_file, &new_file);
                            rename(old_file, new_file)?;
                        }
                        remove_dir(&dir_to_remove)?;
                        loop_ctrl = true;
                    }
                }
            }

            counter += 1;
        }
    }
    println!("Total files treated : {counter}");
    Ok(())
}

// fn perform() -> anyhow::Result<()> {
//     println!("Starting");

//     let args = Args::parse();
//     Ok(())
// }
