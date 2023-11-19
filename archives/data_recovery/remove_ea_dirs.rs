use {don_error::try_or_report, std::fs::remove_dir_all, walkdir::WalkDir};

pub fn perform(root_dir: &str) {
    println!("Removing ea_dirs in {root_dir}");
    let walk_dir = WalkDir::new(root_dir).into_iter();
    for entry in walk_dir.filter_entry(|e| {
        e.file_type().is_dir()
            && !e
                .file_name()
                .to_str()
                .is_some_and(|s| s == "14 Planet of the Apes issues")
    }) {
        try_or_report(|| {
            let entry = entry?;
            if entry.file_name().to_str().is_some_and(|s| s == "@eaDir") {
                remove_dir_all(entry.path())?;
            };
            Ok(())
        })
    }
}
