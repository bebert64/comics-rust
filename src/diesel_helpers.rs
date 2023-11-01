use crate::DonResult;

use {
    diesel::{pg::PgConnection, prelude::*},
    std::env,
};

pub fn db() -> DonResult<PgConnection> {
    let mode = match env::var("MODE").ok().as_deref() {
        Some("PROD") => Mode::Prod,
        _ => Mode::Dev,
    };
    let user = env::var("RW_USERNAME")?;
    let password = env::var("RW_PASSWD")?;
    let ip = env::var("DB_IPV6")?;
    let port = env::var("DB_PORT")?;
    let db_name = match mode {
        Mode::Dev => env::var("DB_NAME_DEV")?,
        Mode::Prod => env::var("DB_NAME_PROD")?,
    };
    let db_url = format!("postgres://{user}:{password}@[{ip}]:{port}/{db_name}",);
    Ok(PgConnection::establish(&db_url)?)
}

#[derive(Debug)]
enum Mode {
    Dev,
    Prod,
}
