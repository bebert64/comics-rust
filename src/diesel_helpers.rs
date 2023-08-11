use crate::DonResult;

use {
    diesel::{pg::PgConnection, prelude::*},
    dotenv::dotenv,
    std::env,
};

pub(crate) fn db() -> DonResult<PgConnection> {
    dotenv()?;
    let db_url = env::var("DATABASE_URL")?;
    Ok(PgConnection::establish(&db_url)?)
}
