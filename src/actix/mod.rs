mod archives;

pub use archives::{get_archives, parse, parse_methods};

macro_rules! failable_response (
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

use failable_response;
