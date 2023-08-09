mod archives;

pub use archives::{get_archives, parse, parse_methods};

macro_rules! try_or_send_err (
    ($fn: block) => {
        match (|| -> ComicsResult<_> {
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