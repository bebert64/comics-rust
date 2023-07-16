use crate::ComicsResult;

use {
    diesel::{pg::PgConnection, prelude::*},
    dotenv::dotenv,
    std::env,
};

pub(crate) fn db() -> ComicsResult<PgConnection> {
    dotenv()?;
    let db_url = env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&db_url)?)
}
