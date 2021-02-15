use std;
use std::fmt::{self, Display};

use serde::{de, ser};
use std::io;
use std::io::ErrorKind;
use std::string::FromUtf8Error;

/// Result type used by the crate
pub type Result<T> = std::result::Result<T, Error>;

/// Error type used by the crate
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// For custom errors by serde.
    Message(String),
    /// Unexpected EOF.
    Eof,
    /// Syntax error.
    Syntax,
    /// IO error.
    Io(String),
    /// Trying to convert non-utf-8 bytes to string.
    FromUtf8(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            Error::Eof => formatter.write_str("unexpected end of input"),
            Error::Syntax => formatter.write_str("syntax error"),
            Error::Io(e) => formatter.write_str(&format!("an IO error occurred: {}", e)),
            Error::FromUtf8(e) => {
                formatter.write_str(&format!("an string conversion error occurred: {}", e))
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        match e.kind() {
            ErrorKind::UnexpectedEof => Error::Eof,
            _ => Error::Io(format!("{:?}", e)),
        }
    }
}

/// This happens when to_string on non-utf8 bytes
impl From<FromUtf8Error> for Error {
    fn from(e: FromUtf8Error) -> Self {
        Error::FromUtf8(format!("{:?}", e))
    }
}
