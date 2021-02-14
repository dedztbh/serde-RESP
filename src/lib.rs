pub mod de;
mod error;
pub mod ser;

pub use error::{Error, Result};

pub enum RESPType<'a> {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Option<&'a [u8]>),
    Array(Option<&'a [RESPType<'a>]>),
}
