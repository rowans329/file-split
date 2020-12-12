// Std imports
use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::io::Error as IoError;
use std::process;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: &str) -> Self {
        Error {
            msg: String::from(msg),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error: {}", self.msg)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Self::new(&format!("{}", err))
    }
}

impl StdError for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub trait UnwrapOrExit<T> {
    fn unwrap_or_exit(self) -> T;
}

impl<T> UnwrapOrExit<T> for Result<T> {
    fn unwrap_or_exit(self) -> T {
        self.unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        })
    }
}
