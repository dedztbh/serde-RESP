use crate::{Error, RESPType, Result};

use serde::de::{self, DeserializeOwned, DeserializeSeed, SeqAccess, Visitor};
use serde::Deserialize;

use std::fmt;
use std::io::{BufRead, BufReader, Cursor, Read};

/// Serializer for RESP format
pub struct Deserializer<'de, R: BufRead> {
    reader: &'de mut R,
}

impl<'de, R: BufRead> Deserializer<'de, R> {
    /// Method for building Deserializer
    pub fn from_buf_reader(reader: &'de mut R) -> Deserializer<'de, R> {
        Deserializer { reader }
    }
}

/// Deserialize from str.
///
/// This function simple wraps the `&str` with `Cursor` and calls [from_buf_reader](ser::from_buf_reader).
///
/// # Errors
/// Please refer to [Error](Error)
pub fn from_str<T>(s: &str) -> Result<T>
where
    T: DeserializeOwned,
{
    let mut reader = Cursor::new(s);
    from_buf_reader(&mut reader)
}

/// Deserialize from reader with `Read` trait.
///
/// This function simply wraps the reader with a `BufReader` and calls [from_buf_reader](ser::from_buf_reader).
/// If your reader has `BufRead` trait, use [from_buf_reader](ser::from_buf_reader) instead.
///
/// # Errors
/// Please refer to [Error](Error)
pub fn from_reader<T, R>(reader: &mut R) -> Result<T>
where
    T: DeserializeOwned,
    R: Read,
{
    let mut reader = BufReader::new(reader);
    from_buf_reader(&mut reader)
}

/// Deserialize from reader with `BufRead` trait.
///
/// # Errors
/// Please refer to [Error](Error)
pub fn from_buf_reader<T, R>(reader: &mut R) -> Result<T>
where
    T: DeserializeOwned,
    R: BufRead,
{
    let mut deserializer = Deserializer::from_buf_reader(reader);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl<'de, R: BufRead> Deserializer<'de, R> {
    fn read_isize(&mut self) -> Result<isize> {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        let trimmed = buffer.trim_end();
        match trimmed.parse::<isize>() {
            Ok(x) => Ok(x),
            Err(_) => Err(Error::Syntax),
        }
    }
}

impl<'de, 'a, R: BufRead> de::Deserializer<'de> for &'a mut Deserializer<'de, R> {
    type Error = Error;

    // You see, this is a bit hacky...
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        match buf[0] {
            b'+' => self.deserialize_str(visitor),      // SimpleString
            b'-' => self.deserialize_string(visitor),   // Error
            b':' => self.deserialize_i64(visitor),      // Integer
            b'$' => self.deserialize_byte_buf(visitor), // BulkString
            b'*' => self.deserialize_seq(visitor),      // Array
            _ => return Err(Error::Syntax),
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        match buffer.trim_end().parse::<i64>() {
            Ok(x) => visitor.visit_i64(x),
            Err(_) => Err(Error::Syntax),
        }
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // SimpleString
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        visitor.visit_str(buffer.trim_end())
    }

    // Error
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        visitor.visit_string(buffer.trim_end().to_string())
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let x = self.read_isize()?;
        if x < 0 {
            return visitor.visit_none();
        }
        let mut buffer = vec![0u8; x as usize];
        self.reader.read_exact(&mut buffer)?;
        visitor.visit_byte_buf(buffer)
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let x = self.read_isize()?;
        if x < 0 {
            return visitor.visit_unit();
        }
        visitor.visit_seq(RESPArray::new(&mut self, x as usize))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct RESPArray<'a, 'de: 'a, R: BufRead> {
    de: &'a mut Deserializer<'de, R>,
    remain_len: usize,
}

impl<'a, 'de, R: BufRead> RESPArray<'a, 'de, R> {
    fn new(de: &'a mut Deserializer<'de, R>, len: usize) -> Self {
        RESPArray {
            de,
            remain_len: len,
        }
    }
}

// `SeqAccess` is provided to the `Visitor` to give it the ability to iterate
// through elements of the sequence.
impl<'de, 'a, R: BufRead> SeqAccess<'de> for RESPArray<'a, 'de, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.remain_len == 0 {
            return Ok(None);
        }
        self.remain_len -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remain_len)
    }
}

struct RESPTypeVisitor;

impl<'de> Visitor<'de> for RESPTypeVisitor {
    type Value = RESPType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A RESP value")
    }

    fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::Integer(v))
    }

    // SimpleString
    fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::SimpleString(v.to_string()))
    }

    // Error
    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::Error(v))
    }

    // BulkString
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::BulkString(Some(v)))
    }

    // null BulkString
    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::BulkString(None))
    }

    // null Array
    fn visit_unit<E>(self) -> std::result::Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(RESPType::Array(None))
    }

    fn visit_seq<A>(
        self,
        mut seq: A,
    ) -> std::result::Result<Self::Value, <A as SeqAccess<'de>>::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut arr: Vec<RESPType> = Vec::with_capacity(seq.size_hint().unwrap_or_default());
        loop {
            match seq.next_element()? {
                None => break,
                Some(elem) => arr.push(elem),
            };
        }
        Ok(RESPType::Array(Some(arr)))
    }
}

impl<'de> Deserialize<'de> for RESPType {
    fn deserialize<D>(
        deserializer: D,
    ) -> std::result::Result<Self, <D as de::Deserializer<'de>>::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(RESPTypeVisitor)
    }
}
