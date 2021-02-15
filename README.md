# serde-RESP
Redis RESP protocol serialization and deserialization with serde.
[Read Specification](https://redis.io/topics/protocol)

```
[dependencies]
serde-resp = "0.1"
```

## Usage

Here are the RESP types and their corresponding Rust types for serde.
Do NOT serialize and deserialize with any other types besides `RESPType`.

- `SimpleString`
    + `RESPType::SimpleString(String)`
- `Error`
    + `RESPType::Error(String)`
- `Integer`
    + `RESPType::Integer(i64)`
- `BulkString`
    + `RESPType::BulkString(Option<&[u8]>)`
        + Use `None` for null bulk strings and `Some` for non-null ones.
- `Array`
    + `RESPType::Array(Option<&[RESPType]>)`
        + Use `None` for null arrays and `Some` for non-null ones.

For usage examples, refer to https://docs.rs/serde_resp/0.1.0/serde_resp/enum.RESPType.html    

## Examples & Documentation
https://docs.rs/serde_resp/0.1.0/serde_resp