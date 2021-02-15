use crate::{Error, RESPType, Result};

use serde::de::{self, DeserializeSeed, SeqAccess, Visitor};
use serde::Deserialize;

use crate::Error::{Eof, Syntax, UnsupportedType};
use std::fmt;
use std::io::{BufRead, BufReader, Cursor, Read};

enum _RSEPType {
    SimpleString,
    Error,
    Integer,
    BulkString,
    Array,
}

pub struct Deserializer<'de, R: BufRead> {
    reader: &'de mut R,
}

impl<'de, R: BufRead> Deserializer<'de, R> {
    pub fn from_buf_reader(reader: &'de mut R) -> Deserializer<'de, R> {
        Deserializer { reader }
    }
}

// pub fn from_reader<'de, T, R>(reader: &'de mut R) -> Result<T>
// where
//     T: Deserialize<'de>,
//     R: Read,
// {
//     let mut reader = BufReader::new(reader);
//     from_buf_reader(&mut reader)
// }

pub fn from_buf_reader<'de, T, R>(reader: &'de mut R) -> Result<T>
where
    T: Deserialize<'de>,
    R: BufRead,
{
    let mut deserializer = Deserializer::from_buf_reader(reader);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.eof()? {
        Err(Eof)
    } else {
        Ok(t)
    }
}

impl<'de, R: BufRead> Deserializer<'de, R> {
    fn eof(&mut self) -> Result<bool> {
        Ok(self.reader.fill_buf()?.len() == 0)
    }

    fn read_isize(&mut self) -> Result<isize> {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        let trimmed = buffer.trim_end();
        match trimmed.parse::<isize>() {
            Ok(x) => Ok(x),
            Err(_) => Err(Syntax),
        }
    }
}

impl<'de, 'a, R: BufRead> de::Deserializer<'de> for &'a mut Deserializer<'de, R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buf = [0u8; 1];
        self.reader.read_exact(&mut buf)?;
        match buf[0] {
            b'+' => self.deserialize_str(visitor),
            b'-' => self.deserialize_string(visitor),
            b':' => self.deserialize_i64(visitor),
            b'$' => self.deserialize_byte_buf(visitor),
            b'*' => self.deserialize_seq(visitor),
            _ => return Err(Syntax),
        }
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut buffer = String::new();
        self.reader.read_line(&mut buffer)?;
        match buffer.trim_end().parse::<i64>() {
            Ok(x) => visitor.visit_i64(x),
            Err(_) => Err(Syntax),
        }
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
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
        Err(UnsupportedType)
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
        Err(UnsupportedType)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
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

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
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
        Err(UnsupportedType)
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
        Err(UnsupportedType)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(UnsupportedType)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct RESPArray<'a, 'de: 'a, R: BufRead> {
    de: &'a mut Deserializer<'de, R>,
    len: usize,
    remain_len: usize,
}

impl<'a, 'de, R: BufRead> RESPArray<'a, 'de, R> {
    fn new(de: &'a mut Deserializer<'de, R>, len: usize) -> Self {
        RESPArray {
            de,
            len,
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
        Some(self.len)
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
        let visitor = RESPTypeVisitor;

        deserializer.deserialize_any(visitor)
    }
}
