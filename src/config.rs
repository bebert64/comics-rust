use {diesel_helpers::Db, serde::Deserialize};

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) postgres: Db,
    pub(crate) comics_dirs: ComicsDirs,
    pub(crate) comic_vine: ComicVine,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ComicsDirs {
    pub(crate) root: String,
    pub(crate) working_dir: String,
    pub(crate) ok: String,
    // pub(crate) zipped: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ComicVine {
    pub(crate) api_key: String,
    pub(crate) url_root: String,
}

config_helpers::config!("comics");
