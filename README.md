# serde-RESP
Redis RESP protocol serialization and deserialization with serde.
[Read Specification](https://redis.io/topics/protocol)

```
[dependencies]
serde-resp = "0.2.0"
```

## Usage
IMPORTANT: Do NOT serialize and deserialize with any other types besides `RESPType`! You may get panic or incorrect results!

Here are the RESP types and their corresponding Rust types for serde.

- `SimpleString`
    + `RESPType::SimpleString(String)`
- `Error`
    + `RESPType::Error(String)`
- `Integer`
    + `RESPType::Integer(i64)`
- `BulkString`
    + `RESPType::BulkString(Option<Vec<u8>>)`
        + Use `None` for null bulk strings and `Some` for non-null ones.
- `Array`
    + `RESPType::Array(Option<Vec<RESPType>>)`
        + Use `None` for null arrays and `Some` for non-null ones.

To serialize, use [ser::to_string](https://docs.rs/serde_resp/0.2.0/serde_resp/ser/fn.to_string.html)
or [ser::to_writer](https://docs.rs/serde_resp/0.2.0/serde_resp/ser/fn.to_writer.html).

To deserialize, use [ser::from_buf_reader](https://docs.rs/serde_resp/0.2.0/serde_resp/de/fn.from_buf_reader.html).

For usage examples, refer to https://docs.rs/serde_resp/0.2.0/serde_resp/enum.RESPType.html

## Examples & Documentation
https://docs.rs/serde_resp/0.2.0/serde_resp