use crate::config::{ComicsDirs, CONFIG};

use {
    don_error::*,
    std::{path::PathBuf, process::Command},
};

fn nas_path(subdir: &str) -> DonResult<PathBuf> {
    let mut nas_path = PathBuf::from(&CONFIG.comics_dirs.root);
    nas_path.push(subdir);
    if !nas_path.exists() {
        mount()?;
        if !nas_path.exists() {
            return Err(err_msg!(
                "nas dir doens't exist even after mount was successtul"
            ));
        }
    }
    Ok(nas_path.to_owned())
}

pub(crate) fn mount() -> DonResult<()> {
    Command::new("mount-NAS").output()?;
    Ok(())
}

impl ComicsDirs {
    pub(crate) fn as_working_dir_path(&self) -> DonResult<PathBuf> {
        nas_path(&self.working_dir)
    }

    pub(crate) fn as_ok_path(&self) -> DonResult<PathBuf> {
        nas_path(&self.ok)
    }

    // pub(crate) fn as_zipped_tmp_path(&self) -> PathBuf {
    //     nas_path(Some(&self.zipped))
    // }
}
