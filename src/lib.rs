pub mod de;
mod error;
pub mod ser;

pub use error::{Error, Result};

pub enum RESPType<'a> {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(BulkString<'a>),
    Array(Array<'a>),
}

pub enum BulkString<'a> {
    Null,
    Value(&'a [u8]),
}

pub enum Array<'a> {
    Null,
    Value(&'a [RESPType<'a>]),
}
