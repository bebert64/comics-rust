pub type ComicsResult<T> = std::result::Result<T, ComicsError>;

impl ComicsError {
    pub(crate) fn report(&self) {
        println!("ComicsError : {:?}", self.inner)
    }

    pub(crate) fn new(err_msg: String) -> Self {
        ComicsError {
            inner: anyhow::Error::msg(err_msg),
        }
    }
}

#[derive(Debug)]
pub struct ComicsError {
    inner: anyhow::Error,
}

impl<T: Into<anyhow::Error>> From<T> for ComicsError {
    fn from(err: T) -> Self {
        Self { inner: err.into() }
    }
}

/// Execute lambda and send error to sentry in case of error
pub(crate) fn try_or_report(lambda: impl FnOnce() -> ComicsResult<()>) {
    if let Err(err) = lambda() {
        err.report();
    }
}

pub(crate) fn err_msg<R>(msg: String) -> ComicsResult<R> {
    Err(ComicsError {
        inner: anyhow::Error::msg(msg),
    })
}

pub trait ComicsResultOptionExtensions: Sized {
    type SOME;

    fn ok_or_comics_err(self, err_msg: &str) -> ComicsResult<Self::SOME>;
}

impl<T> ComicsResultOptionExtensions for Option<T> {
    type SOME = T;

    fn ok_or_comics_err(self, err_msg: &str) -> ComicsResult<T> {
        self.ok_or_else(|| ComicsError::new(err_msg.to_owned()))
    }
}
