# serde-RESP
Redis RESP protocol serialization and deserialization with serde.

```
[dependencies]
serde-resp = "0.1"
```

## Usage

Here are the RESP types and their supported Rust types for serde.
Do NOT use other types for serialization and deserialization.

- `SimpleString`
    + `RESPType::SimpleString(String)`
    + Do NOT use `String` directly!
- `Error`
    + `RESPType::Error(String)`
- `Integer`
    + `RESPType::Integer(i64)`
    + Primitive integers are supported but make sure the value fits in i64.
- `BulkString`
    + `RESPType::BulkString(Option<&[u8]>)`
        + Use `None` for null bulk strings and `Some` for non-null ones.
- `Array`
    + `RESPType::Array(Option<&[RESPType]>)`
        + Use `None` for null arrays and `Some` for non-null ones.
    + `[RESPType]`
        + Only for non-null arrays.

For usage examples, refer to https://docs.rs/serde_resp/0.1.0/serde_resp/enum.RESPType.html    

## Examples & Documentation
https://docs.rs/serde_resp/0.1.0/serde_resp