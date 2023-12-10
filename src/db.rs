use crate::config::CONFIG;

use {diesel::prelude::*, don_error::*};

pub(crate) fn db() -> DonResult<PgConnection> {
    Ok(PgConnection::establish(&CONFIG.postgres.to_url())?)
}
