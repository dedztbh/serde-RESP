//! # serde-RESP
//! Redis RESP protocol serialization and deserialization with serde.
//! [Read Specification](https://redis.io/topics/protocol)
//!
//! ## Usage
//! IMPORTANT: Do NOT serialize and deserialize with any other types besides [RESP](RESP)! You may get panic or incorrect results!
//!
//! Here are the RESP types and their corresponding Rust types for serde.
//!
//! - `Simple String`
//!     + [RESP::SimpleString(String)](RESPType::SimpleString)
//! - `Error`
//!     + [RESP::Error(String)](RESPType::Error)
//! - `Integer`
//!     + [RESP::Integer(i64)](RESPType::Integer)
//! - `Bulk String`
//!     + [RESP::BulkString(Option<Vec<u8>>)](RESPType::BulkString)
//!         + Use `None` for null bulk strings and `Some` for non-null ones.
//! - `Array`
//!     + [RESP::Array(Option<Vec<RESP>>)](RESPType::Array)
//!         + Use `None` for null arrays and `Some` for non-null ones.
//!
//! To serialize, use [ser::to_string](ser::to_string) or [ser::to_writer](ser::to_writer).
//!
//! To deserialize, use [de::from_str](de::from_str) or [de::from_reader](de::from_reader) or [de::from_buf_reader](de::from_buf_reader).
//!
//! For usage examples, refer to [RESP](RESP)
//!
//! ## Macros
//!
//! Since 0.3.0, you can start using very handy macros! Here is a demo:
//! ```
//!     use serde_resp::{array, array_null, bulk, bulk_null, de, err_str, int, ser, simple, RESP};
//!     let resp_array = array![
//!         simple!("simple string".to_owned()),
//!         err_str!("error string".to_owned()),
//!         int!(42),
//!         bulk!(b"bulk string".to_vec()),
//!         bulk_null!(),
//!         array![
//!             simple!("arrays of arrays!".to_owned()),
//!             array![simple!("OK ENOUGH!".to_owned())],
//!         ],
//!         array_null!(),
//!     ];
//!     let serialized = ser::to_string(&resp_array).unwrap();
//!     assert_eq!("*7\r\n+simple string\r\n-error string\r\n:42\r\n$11\r\nbulk string\r\n$-1\r\n*2\r\n+arrays of arrays!\r\n*1\r\n+OK ENOUGH!\r\n*-1\r\n", serialized);
//!     let deserialized = de::from_str(&serialized).unwrap();
//!     assert_eq!(resp_array, deserialized);
//! ```

pub mod de;
mod error;
mod macros;
pub mod ser;

pub use error::{Error, Result};

#[derive(Eq, PartialEq, Clone, Debug)]
/// This enum creates a one-to-one type mapping with RESP types.
/// Please only use variants of this type for serde operations.
pub enum RESPType {
    /// Correspond to simple string in RESP.
    /// Also refer to [simple!](simple!) macro.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{de, ser, simple, RESP};
    ///
    /// /// Serialization
    /// let obj = simple!("OK".to_owned()); // equivalent to RESP::SimpleString("OK".to_owned());
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("+OK\r\n".to_owned(), serialized);
    ///
    /// /// Deserialization
    /// let deserialized: RESP = de::from_str("+OK\r\n").unwrap();
    /// assert_eq!(simple!("OK".to_owned()), deserialized);
    /// ```
    SimpleString(String),
    /// Correspond to error string in RESP.
    /// Also refer to [err_str!](err_str!) macro.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{de, err_str, ser, RESP};
    ///
    /// /// Serialization
    /// // Example 1
    /// let obj = err_str!("ERR unknown command 'foobar'".to_owned()); // equivalent to RESP::Error(...)
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("-ERR unknown command 'foobar'\r\n".to_owned(), serialized);
    ///
    /// // Example 2
    /// let obj = err_str!("WRONGTYPE Operation against a key holding the wrong kind of value".to_owned());
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(
    ///     "-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_owned(),
    ///     serialized
    /// );
    ///
    /// /// Deserialization
    /// let deserialized: RESP = de::from_str("-ERR unknown command 'foobar'\r\n").unwrap();
    /// assert_eq!(err_str!("ERR unknown command 'foobar'".to_owned()), deserialized);
    /// ```
    Error(String),
    /// Correspond to integer in RESP.
    /// Also refer to [int!](int!) macro.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{de, int, ser, RESP};
    ///
    /// /// Serialization
    /// // Regular Example
    /// let obj = int!(1000); // equivalent to RESP::Integer(1000);
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(":1000\r\n".to_owned(), serialized);
    ///
    /// /// Deserialization
    /// let deserialized: RESP = de::from_str(":1000\r\n").unwrap();
    /// assert_eq!(int!(1000), deserialized);
    /// ```
    Integer(i64),
    /// Correspond to bulk string in RESP. Use `None` for null bulk string and Some for non-null ones.
    /// Also refer to [bulk!](bulk!) macro and [bulk_null!](bulk_null!) macro.
    ///
    /// According to specification, bulk string is binary-safe so it is NOT recommended to use [ser::to_string](ser::to_string) (may cause [Error::FromUtf8](Error::FromUtf8)).
    /// Use [ser::to_writer](ser::to_writer) to write to a byte buffer instead.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{bulk, bulk_null, de, ser, RESP};
    ///
    /// /// Serialization
    /// // Regular Example
    /// let obj = bulk!(b"foobar".to_vec()); // equivalent to RESP::BulkString(Some(...))
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("$6\r\nfoobar\r\n".to_owned(), serialized);
    ///
    /// // Empty
    /// let obj = bulk!(b"".to_vec());
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("$0\r\n\r\n".to_owned(), serialized);
    ///
    /// // Null
    /// let obj = bulk_null!(); // equivalent to RESP::BulkString(None)
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("$-1\r\n".to_owned(), serialized);
    ///
    /// /// Deserialization
    /// // Regular Example
    /// let deserialized: RESP = de::from_str("$6\r\nfoobar\r\n").unwrap();
    /// assert_eq!(bulk!(b"foobar".to_vec()), deserialized);
    ///
    /// // Empty
    /// let deserialized: RESP = de::from_str("$0\r\n\r\n").unwrap();
    /// assert_eq!(bulk!(b"".to_vec()), deserialized);
    ///
    /// // Null
    /// let deserialized: RESP = de::from_str("$-1\r\n").unwrap();
    /// assert_eq!(bulk_null!(), deserialized);
    /// ```
    BulkString(Option<Vec<u8>>),
    /// Correspond to array in RESP. Use None for null array and Some for non-null ones.
    /// Also refer to [array!](array!) macro and [array_null!](array_null!) macro.
    ///
    /// Mixed type, arrays of arrays are allowed.
    ///
    /// # Examples
    /// ```
    /// use serde_resp::{array, array_null, bulk, bulk_null, de, err_str, int, ser, simple, RESP};
    ///
    /// /// Serialization
    /// // Empty
    /// let obj = array![]; // equivalent to RESP::Array(Some(vec![]))
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("*0\r\n".to_owned(), serialized);
    ///
    /// // Regular Example
    /// let obj = array![
    ///     bulk!(b"foo".to_vec()),
    ///     bulk!(b"bar".to_vec()),
    /// ]; // equivalent to RESP::Array(Some(vec![bulk!(...), bulk!(...)]))
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".to_owned(), serialized);
    ///
    /// // Another Regular Example
    /// let obj = array![
    ///     int!(1),
    ///     int!(2),
    ///     int!(3),
    /// ];
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("*3\r\n:1\r\n:2\r\n:3\r\n".to_owned(), serialized);
    ///
    /// // Mixed Type
    /// let obj = array![
    ///     int!(1),
    ///     int!(2),
    ///     int!(3),
    ///     int!(4),
    ///     bulk!(b"foobar".to_vec()),
    /// ];
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(
    ///     "*5\r\n:1\r\n:2\r\n:3\r\n:4\r\n$6\r\nfoobar\r\n".to_owned(),
    ///     serialized
    /// );
    ///
    /// // Null Array
    /// let obj = array_null!(); // equivalent to RESP::Array(None)
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!("*-1\r\n".to_owned(), serialized);
    ///
    /// // Arrays of Arrays
    /// let obj = array![
    ///     array![
    ///         int!(1),
    ///         int!(2),
    ///         int!(3),
    ///     ],
    ///     array![
    ///         simple!("Foo".to_owned()),
    ///         err_str!("Bar".to_owned()),
    ///     ],
    /// ];
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(
    ///     "*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Foo\r\n-Bar\r\n".to_owned(),
    ///     serialized
    /// );
    ///
    /// // Null elements in Arrays
    /// let obj = array![
    ///     bulk!(b"foo".to_vec()),
    ///     bulk_null!(),
    ///     bulk!(b"bar".to_vec()),
    /// ];
    /// let serialized = ser::to_string(&obj).unwrap();
    /// assert_eq!(
    ///     "*3\r\n$3\r\nfoo\r\n$-1\r\n$3\r\nbar\r\n".to_owned(),
    ///     serialized
    /// );
    ///
    /// /// Deserialization
    /// // Null
    /// let deserialized: RESP = de::from_str("*-1\r\n").unwrap();
    /// assert_eq!(array_null!(), deserialized);
    ///
    /// // Mixed Type
    /// let deserialized: RESP = de::from_str("*2\r\n:1\r\n$6\r\nfoobar\r\n").unwrap();
    /// let expected = array![
    ///     int!(1),
    ///     bulk!(b"foobar".to_vec()),
    /// ];
    /// assert_eq!(expected, deserialized);
    ///
    /// // Arrays of Arrays with Null Bulk String
    /// let deserialized: RESP = de::from_str("*3\r\n*3\r\n:1\r\n:2\r\n:3\r\n$-1\r\n*2\r\n+Foo\r\n-Bar\r\n").unwrap();
    /// let expected = array![
    ///     int!(1),
    ///     bulk!(b"foobar".to_vec()),
    /// ];
    /// let expected = array![
    ///     array![
    ///         int!(1),
    ///         int!(2),
    ///         int!(3),
    ///     ],
    ///     bulk_null!(),
    ///     array![
    ///         simple!("Foo".to_owned()),
    ///         err_str!("Bar".to_owned()),
    ///     ],
    /// ];
    /// assert_eq!(expected, deserialized);
    /// ```
    Array(Option<Vec<RESPType>>),
}

/// Refer to [RESPType](RESPType). This is just an alias.
pub type RESP = RESPType;
