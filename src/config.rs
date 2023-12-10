use {diesel_helpers::Db, serde::Deserialize};

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) postgres: Db,
    pub(crate) comics_dirs: ComicsDirs,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ComicsDirs {
    pub(crate) root: String,
    pub(crate) working_dir: String,
    pub(crate) ok: String,
    // pub(crate) zipped: String,
}

config_helpers::config!("comics");
