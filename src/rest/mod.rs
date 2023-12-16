mod archives;
mod volumes;

pub use {
    archives::{get_archives, parse, parse_methods},
    volumes::{get_volumes, merge_volumes, rename_volume},
};

macro_rules! try_or_send_err (
    ($fn: block) => {
        match (|| -> DonResult<_> {
            $fn
        })() {
            Ok(responder) => responder,
            Err(err) => {
                err.report();
                HttpResponse::InternalServerError().body(format!("{err:?}"))
            }
        }
    }
);

use try_or_send_err;
