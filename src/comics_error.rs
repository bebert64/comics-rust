use thiserror::Error;

#[derive(Error, Debug)]
pub enum ComicsError {
    #[error(transparent)]
    EnvVarError(#[from] std::env::VarError),
    #[error(transparent)]
    DbError(#[from] diesel::result::Error),
    #[error(transparent)]
    SqliteError(#[from] diesel::result::ConnectionError),
    #[error("Trying to save without affecting an id")]
    SavingDefaultError,
    #[error("Trying to retrieve via a missing foreign key")]
    ForeignKeyError,
}

pub type Result<T> = std::result::Result<T, ComicsError>;
