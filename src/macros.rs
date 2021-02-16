//! Here are all the macros for convenience. Refer to [RESPType](crate::RESPType) for usage examples.

/// `simple!(...)` is equivalent to `RESP::SimpleString(...)`.
#[macro_export]
macro_rules! simple {
    ($x:expr) => {
        RESP::SimpleString($x)
    };
}

/// `err_str!(...)` is equivalent to `RESP::Error(...)`.
#[macro_export]
macro_rules! err_str {
    ($x:expr) => {
        RESP::Error($x)
    };
}

/// `int!(...)` is equivalent to `RESP::Integer(...)`.
#[macro_export]
macro_rules! int {
    ($x:expr) => {
        RESP::Integer($x)
    };
}

/// `bulk!(...)` is equivalent to `RESP::BulkString(Some(...))`.
#[macro_export]
macro_rules! bulk {
    ($x:expr) => {
        RESP::BulkString(Some($x))
    };
}

/// `bulk_null!()` is equivalent to `RESP::BulkString(None)`.
#[macro_export]
macro_rules! bulk_null {
    () => {
        RESP::BulkString(None)
    };
}

/// `array![...]` is equivalent to `RESP::Array(Some(vec![...]))`.
#[macro_export]
macro_rules! array {
    ($($x:expr),* $(,)?) => {
        RESP::Array(Some(vec![$($x),*]))
    };
}

/// `array_null!()` is equivalent to `RESP::Array(None)`.
#[macro_export]
macro_rules! array_null {
    () => {
        RESP::Array(None)
    };
}
