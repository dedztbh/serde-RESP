//! Here are all the macros for convenience. Refer to [RESPType](crate::RESPType) for usage examples.

#[macro_export]
/// `simple!(...)` is equivalent to `RESP::SimpleString(...)`.
macro_rules! simple {
    ($x:expr) => {
        RESP::SimpleString($x)
    };
}

#[macro_export]
/// `err_str!(...)` is equivalent to `RESP::Error(...)`.
macro_rules! err_str {
    ($x:expr) => {
        RESP::Error($x)
    };
}

#[macro_export]
/// `int!(...)` is equivalent to `RESP::Integer(...)`.
macro_rules! int {
    ($x:expr) => {
        RESP::Integer($x)
    };
}

#[macro_export]
/// `bulk!(...)` is equivalent to `RESP::BulkString(Some(...))`.
macro_rules! bulk {
    ($x:expr) => {
        RESP::BulkString(Some($x))
    };
}

#[macro_export]
/// `bulk_null!()` is equivalent to `RESP::BulkString(None)`.
macro_rules! bulk_null {
    () => {
        RESP::BulkString(None)
    };
}

#[macro_export]
/// `array![...]` is equivalent to `RESP::Array(Some(vec![...]))`.
macro_rules! array {
    ($($x:expr),* $(,)?) => {
        RESP::Array(Some(vec!{$($x),*}))
    };
}
#[macro_export]
/// `array_null!()` is equivalent to `RESP::Array(None)`.
macro_rules! array_null {
    () => {
        RESP::Array(None)
    };
}
