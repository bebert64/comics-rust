mod archives;
mod books;
mod volumes;

use {
    actix_cors::Cors,
    actix_web::{
        body::{BoxBody, EitherBody},
        dev::{ServiceFactory, ServiceRequest, ServiceResponse},
        App,
    },
};

pub trait ComicsApp:
    ServiceFactory<
    ServiceRequest,
    Config = (),
    Response = ServiceResponse<EitherBody<BoxBody>>,
    Error = actix_web::Error,
    InitError = (),
>
{
}

impl<T> ComicsApp for T where
    T: ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
        InitError = (),
    >
{
}

pub fn app_with_services() -> App<impl ComicsApp> {
    let app = App::new().wrap(Cors::permissive());
    let app = archives::add_services(app);
    let app = volumes::add_services(app);
    let app = books::add_services(app);
    app
}

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
