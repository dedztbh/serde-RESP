use serde_resp::{ser, RESPType};

#[test]
fn serialize_simple_string() {
    let obj = RESPType::SimpleString("OK".to_owned());
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("+OK\r\n".to_owned(), serialized);
}

#[test]
fn serialize_error() {
    let obj = RESPType::Error("ERR unknown command 'foobar'".to_owned());
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("-ERR unknown command 'foobar'\r\n".to_owned(), serialized);
    let obj = RESPType::Error(
        "WRONGTYPE Operation against a key holding the wrong kind of value".to_owned(),
    );
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!(
        "-WRONGTYPE Operation against a key holding the wrong kind of value\r\n".to_owned(),
        serialized
    );
}

#[test]
fn serialize_integer() {
    let obj = RESPType::Integer(1000);
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!(":1000\r\n".to_owned(), serialized);
}

#[test]
fn serialize_bulk_string() {
    // Regular Example
    let obj = RESPType::BulkString(Some(b"foobar"));
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("$6\r\nfoobar\r\n".to_owned(), serialized);

    // Empty
    let obj = RESPType::BulkString(Some(b""));
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("$0\r\n\r\n".to_owned(), serialized);

    // Null
    let obj = RESPType::BulkString(None);
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("$-1\r\n".to_owned(), serialized);
}

#[test]
fn serialize_array() {
    let obj = RESPType::Array(Some(&[]));
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("*0\r\n".to_owned(), serialized);

    // Regular Example
    let obj = RESPType::Array(Some(&[
        RESPType::BulkString(Some(b"foo")),
        RESPType::BulkString(Some(b"bar")),
    ]));
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n".to_owned(), serialized);

    // Another Regular Example
    let obj = RESPType::Array(Some(&[
        RESPType::Integer(1),
        RESPType::Integer(2),
        RESPType::Integer(3),
    ]));
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("*3\r\n:1\r\n:2\r\n:3\r\n".to_owned(), serialized);

    // Mixed Type
    let obj = RESPType::Array(Some(&[
        RESPType::Integer(1),
        RESPType::Integer(2),
        RESPType::Integer(3),
        RESPType::Integer(4),
        RESPType::BulkString(Some(b"foobar")),
    ]));
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!(
        "*5\r\n:1\r\n:2\r\n:3\r\n:4\r\n$6\r\nfoobar\r\n".to_owned(),
        serialized
    );

    // Null Array
    let obj = RESPType::Array(None);
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!("*-1\r\n".to_owned(), serialized);

    // Arrays of Arrays
    let subarr1 = [
        RESPType::Integer(1),
        RESPType::Integer(2),
        RESPType::Integer(3),
    ];
    let subarr2 = [
        RESPType::SimpleString("Foo".to_owned()),
        RESPType::Error("Bar".to_owned()),
    ];
    let arr = [
        RESPType::Array(Some(&subarr1)),
        RESPType::Array(Some(&subarr2)),
    ];
    let obj = RESPType::Array(Some(&arr));
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!(
        "*2\r\n*3\r\n:1\r\n:2\r\n:3\r\n*2\r\n+Foo\r\n-Bar\r\n".to_owned(),
        serialized
    );

    // Null elements in Arrays
    let arr = [
        RESPType::BulkString(Some(b"foo")),
        RESPType::BulkString(None),
        RESPType::BulkString(Some(b"bar")),
    ];
    let obj = RESPType::Array(Some(&arr));
    let serialized = ser::to_string(&obj).unwrap();
    assert_eq!(
        "*3\r\n$3\r\nfoo\r\n$-1\r\n$3\r\nbar\r\n".to_owned(),
        serialized
    );
}
