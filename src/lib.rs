//! # serde-RESP
//! Redis RESP protocol serialization and deserialization with serde.
//! [Read Specification](https://redis.io/topics/protocol)
//!
//! ## Usage
//! IMPORTANT: Do NOT serialize and deserialize with any other types besides [RESPType](RESPType)! You may get panic or incorrect results!
//!
//! Here are the RESP types and their corresponding Rust types for serde.
//!
//! - `SimpleString`
//!     + [RESPType::SimpleString(String)](RESPType::SimpleString)
//! - `Error`
//!     + [RESPType::Error(String)](RESPType::Error)
//! - `Integer`
//!     + [RESPType::Integer(i64)](RESPType::Integer)
//! - `BulkString`
//!     + [RESPType::BulkString(Option<Vec<u8>>)](RESPType::BulkString)
//!         + Use `None` for null bulk strings and `Some` for non-null ones.
//! - `Array`
//!     + [RESPType::Array(Option<Vec<RESPType>>)](RESPType::Array)
//!         + Use `None` for null arrays and `Some` for non-null ones.
//!
//! To serialize, use [ser::to_string](ser::to_string) or [ser::to_writer](ser::to_writer).
//!
//! To deserialize, use [de::from_str](de::from_str) or [de::from_reader](de::from_reader) or [de::from_buf_reader](de::from_buf_reader).
//!
//! For usage examples, refer to [RESPType](RESPType)

pub mod de;
mod error;
pub mod ser;

pub use error::{Error, Result};

#[derive(Eq, PartialEq, Clone, Debug)]
/// This enum creates a one-to-one type mapping with RESP types.
/// Please only use variants of this type for serde operations.
pub enum RESPType {
    /// Correspond to simple string in RESP.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{ser, de, RESPType};
    ///
    /// /// Serialization
    /// let obj = RESPType::SimpleString("OK".to_owned());
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("+OK\r\n".to_owned(), serialized);
    ///
    /// /// Deserialization
    /// let deserialized: RESPType = de::from_str("+OK\r\n").unwrap();
    /// assert_eq!(RESPType::SimpleString("OK".to_owned()), deserialized);
    /// ```
    SimpleString(String),
    /// Correspond to error string in RESP.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{ser, de, RESPType};
    ///
    /// /// Serialization
    /// // Example 1
    /// let obj = RESPType::Error("ERR unknown command 'foobar'".to_owned());
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("-ERR unknown command 'foobar'\r\n".to_owned(), serialized);
    ///
    /// // Example 2
    /// let obj = RESPType::Error(
    /// "WRONGTYPE Operation against a key holding the wrong kind of value".to_owned(),
    /// );
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(
    ///     "-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_owned(),
    ///     serialized
    /// );
    ///
    /// /// Deserialization
    /// let deserialized: RESPType = de::from_str("-ERR unknown command 'foobar'\r\n").unwrap();
    /// assert_eq!(RESPType::Error("ERR unknown command 'foobar'".to_owned()), deserialized);
    /// ```
    Error(String),
    /// Correspond to integer in RESP.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{ser, de, RESPType};
    ///
    /// /// Serialization
    /// // Regular Example
    /// let obj = RESPType::Integer(1000i64);
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(":1000\r\n".to_owned(), serialized);
    ///
    /// /// Deserialization
    /// let deserialized: RESPType = de::from_str(":1000\r\n").unwrap();
    /// assert_eq!(RESPType::Integer(1000), deserialized);
    /// ```
    Integer(i64),
    /// Correspond to bulk string in RESP. Use `None` for null bulk string and Some for non-null ones.
    ///
    /// According to specification, bulk string is binary-safe so it is NOT recommended to use [ser::to_string](ser::to_string) (may cause [Error::FromUtf8](Error::FromUtf8)).
    /// Use [ser::to_writer](ser::to_writer) to write to a byte buffer instead.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{ser, de, RESPType};
    ///
    /// /// Serialization
    /// // Regular Example
    /// let obj = RESPType::BulkString(Some(b"foobar".to_vec()));
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("$6\r\nfoobar\r\n".to_owned(), serialized);
    ///
    /// // Empty
    /// let obj = RESPType::BulkString(Some(b"".to_vec()));
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("$0\r\n\r\n".to_owned(), serialized);
    ///
    /// // Null
    /// let obj = RESPType::BulkString(None);
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("$-1\r\n".to_owned(), serialized);
    ///
    /// /// Deserialization
    /// // Regular Example
    /// let deserialized: RESPType = de::from_str("$6\r\nfoobar\r\n").unwrap();
    /// assert_eq!(RESPType::BulkString(Some(b"foobar".to_vec())), deserialized);
    ///
    /// // Empty
    /// let deserialized: RESPType = de::from_str("$0\r\n\r\n").unwrap();
    /// assert_eq!(RESPType::BulkString(Some(b"".to_vec())), deserialized);
    ///
    /// // Null
    /// let deserialized: RESPType = de::from_str("$-1\r\n").unwrap();
    /// assert_eq!(RESPType::BulkString(None), deserialized);
    /// ```
    BulkString(Option<Vec<u8>>),
    /// Correspond to array in RESP. Use None for null array and Some for non-null ones.
    ///
    /// Mixed type, arrays of arrays are allowed.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{ser, de, RESPType};
    ///
    /// /// Serialization
    /// // Empty
    /// let obj = RESPType::Array(Some(vec![]));
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("*0\r\n".to_owned(), serialized);
    ///
    /// // Regular Example
    /// let obj = RESPType::Array(Some(vec![
    ///     RESPType::BulkString(Some(b"foo".to_vec())),
    ///     RESPType::BulkString(Some(b"bar".to_vec())),
    /// ]));
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".to_owned(), serialized);
    ///
    /// // Another Regular Example
    /// let obj = RESPType::Array(Some(vec![
    ///     RESPType::Integer(1),
    ///     RESPType::Integer(2),
    ///     RESPType::Integer(3),
    /// ]));
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("*3\r\n:1\r\n:2\r\n:3\r\n".to_owned(), serialized);
    ///
    /// // Mixed Type
    /// let obj = RESPType::Array(Some(vec![
    ///     RESPType::Integer(1),
    ///     RESPType::Integer(2),
    ///     RESPType::Integer(3),
    ///     RESPType::Integer(4),
    ///     RESPType::BulkString(Some(b"foobar".to_vec())),
    /// ]));
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(
    ///     "*5\r\n:1\r\n:2\r\n:3\r\n:4\r\n$6\r\nfoobar\r\n".to_owned(),
    ///     serialized
    /// );
    ///
    /// // Null Array
    /// let obj = RESPType::Array(None);
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("*-1\r\n".to_owned(), serialized);
    ///
    /// // Arrays of Arrays
    /// let obj = RESPType::Array(Some(vec![
    ///     RESPType::Array(Some(vec![
    ///         RESPType::Integer(1),
    ///         RESPType::Integer(2),
    ///         RESPType::Integer(3),
    ///     ])),
    ///     RESPType::Array(Some(vec![
    ///         RESPType::SimpleString("Foo".to_owned()),
    ///         RESPType::Error("Bar".to_owned()),
    ///     ])),
    /// ]));
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(
    ///     "*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Foo\r\n-Bar\r\n".to_owned(),
    ///     serialized
    /// );
    ///
    /// // Null elements in Arrays
    /// let obj = RESPType::Array(Some(vec![
    ///     RESPType::BulkString(Some(b"foo".to_vec())),
    ///     RESPType::BulkString(None),
    ///     RESPType::BulkString(Some(b"bar".to_vec())),
    /// ]));
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(
    ///     "*3\r\n$3\r\nfoo\r\n$-1\r\n$3\r\nbar\r\n".to_owned(),
    ///     serialized
    /// );
    ///
    /// /// Deserialization
    /// // Null
    /// let deserialized: RESPType = de::from_str("*-1\r\n").unwrap();
    /// assert_eq!(RESPType::Array(None), deserialized);
    ///
    /// // Mixed Type
    /// let deserialized: RESPType = de::from_str("*2\r\n:1\r\n$6\r\nfoobar\r\n").unwrap();
    /// let expected = RESPType::Array(Some(vec![
    ///     RESPType::Integer(1),
    ///     RESPType::BulkString(Some(b"foobar".to_vec()))
    /// ]));
    /// assert_eq!(expected, deserialized);
    ///
    /// // Arrays of Arrays with Null Bulk String
    /// let deserialized: RESPType = de::from_str("*3\r\n*3\r\n:1\r\n:2\r\n:3\r\n$-1\r\n*2\r\n+Foo\r\n-Bar\r\n").unwrap();
    /// let expected = RESPType::Array(Some(vec![
    ///     RESPType::Integer(1),
    ///     RESPType::BulkString(Some(b"foobar".to_vec()))
    /// ]));
    /// let expected = RESPType::Array(Some(vec![
    ///     RESPType::Array(Some(vec![
    ///         RESPType::Integer(1),
    ///         RESPType::Integer(2),
    ///         RESPType::Integer(3),
    ///     ])),
    ///     RESPType::BulkString(None),
    ///     RESPType::Array(Some(vec![
    ///         RESPType::SimpleString("Foo".to_owned()),
    ///         RESPType::Error("Bar".to_owned()),
    ///     ])),
    /// ]));
    /// assert_eq!(expected, deserialized);
    /// ```
    Array(Option<Vec<RESPType>>),
}
