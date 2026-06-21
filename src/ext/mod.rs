mod cmd;
mod fetch;
mod fs;
mod timer;

pub use cmd::*;
pub use fetch::*;
pub use fs::*;
pub use timer::*;

use deno_core::error::CoreError;
use std::fmt;

#[derive(Debug)]
pub struct DenoOpError(pub CoreError);

impl fmt::Display for DenoOpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<DenoOpError> for CoreError {
    fn from(err: DenoOpError) -> Self {
        err.0
    }
}

impl DenoOpError {
    pub fn msg(message: impl Into<std::borrow::Cow<'static, str>>) -> Self {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, message.into().into_owned());

        DenoOpError(CoreError(Box::new(deno_core::error::CoreErrorKind::Io(io_err))))
    }

    pub fn map<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        let io_err = std::io::Error::new(std::io::ErrorKind::Other, err);

        DenoOpError(CoreError(Box::new(deno_core::error::CoreErrorKind::Io(io_err))))
    }
}
