use serde::{ser, Serialize};

use crate::error::Error::{IntegerOutOfBound, UnsupportedType};
use crate::{Error, RESPType, Result};
use serde::ser::SerializeSeq;
use std::io::Write;
use std::result;

// Implement serialization for RESPType
impl serde::Serialize for RESPType {
    fn serialize<S>(
        &self,
        s: S,
    ) -> result::Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error>
    where
        S: serde::Serializer,
    {
        match self {
            RESPType::SimpleString(str) => s.serialize_str(&("+".to_owned() + str)),
            RESPType::Error(str) => s.serialize_str(&("-".to_owned() + str)),
            RESPType::Integer(i) => s.serialize_i64(*i),
            RESPType::BulkString(bulk_str) => match bulk_str {
                None => s.serialize_unit_variant("BulkString", 0, "Null"),
                Some(val) => s.serialize_bytes(val),
            },
            RESPType::Array(arr) => match arr {
                None => s.serialize_unit_variant("Array", 0, "Null"),
                Some(vals) => {
                    let mut s = s.serialize_seq(Some(vals.len()))?;
                    for v in vals {
                        s.serialize_element(v)?;
                    }
                    s.end()
                }
            },
        }
    }
}

pub struct Serializer<W: Write> {
    writer: W,
}

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut buf: Vec<u8> = Vec::new();
    to_writer(value, &mut buf)?;
    Ok(String::from_utf8(buf)?)
}

pub fn to_writer<T, W>(value: &T, writer: &mut W) -> Result<()>
where
    T: Serialize,
    W: Write,
{
    let mut serializer = Serializer { writer };
    value.serialize(&mut serializer)?;
    Ok(())
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(UnsupportedType)
    }

    // Integers must fit within an i64
    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.writer.write_all(b":")?;
        itoa::write(&mut self.writer, v)?;
        self.writer.write_all(b"\r\n")?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        if v > std::i64::MAX as u64 {
            Err(IntegerOutOfBound)
        } else {
            self.serialize_i64(v as i64)
        }
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(UnsupportedType)
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(UnsupportedType)
    }

    // Serialize a char as a single-character string.
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    // Used by `RESPType::SimpleString` and `RESPType::Error`. Do not use directly!
    fn serialize_str(self, v: &str) -> Result<()> {
        self.writer.write_all(v.as_bytes())?;
        self.writer.write_all(b"\r\n")?;
        Ok(())
    }

    // Bulk string (Not null)
    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.writer.write_all(b"$")?;
        itoa::write(&mut self.writer, v.len() as u64)?;
        self.writer.write_all(b"\r\n")?;
        self.writer.write_all(v)?;
        self.writer.write_all(b"\r\n")?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Err(UnsupportedType)
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(UnsupportedType)
    }

    // Use `RESPBulkString::Null` and `RESPArray::Null` instead
    fn serialize_unit(self) -> Result<()> {
        Err(UnsupportedType)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(UnsupportedType)
    }

    // It might be `RESPType::BulkString::Null` or `RESPType::Array::Null`
    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        match name {
            "BulkString" => self.writer.write_all(b"$-1\r\n")?,
            "Array" => self.writer.write_all(b"*-1\r\n")?,
            _ => return Err(UnsupportedType),
        }
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(UnsupportedType)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(UnsupportedType)
    }

    // Write beginning of array
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        match len {
            None => return Err(UnsupportedType),
            Some(len) => {
                self.writer.write_all(b"*")?;
                itoa::write(&mut self.writer, len as u64)?;
                self.writer.write_all(b"\r\n")?;
            }
        }
        Ok(self)
    }

    // Treat as array
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Treat as array
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    // Treat as array
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(UnsupportedType)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(UnsupportedType)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(UnsupportedType)
    }
}

// The following impls deal with the serialization of compound types like
// sequences. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a, W> ser::SerializeSeq for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a, W> ser::SerializeTuple for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a, W> ser::SerializeTupleStruct for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Tuple variants
impl<'a, W> ser::SerializeTupleVariant for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// The rest are not supported
impl<'a, W> ser::SerializeMap for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(UnsupportedType)
    }
    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(UnsupportedType)
    }
    fn end(self) -> Result<()> {
        Err(UnsupportedType)
    }
}

impl<'a, W> ser::SerializeStruct for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(UnsupportedType)
    }
    fn end(self) -> Result<()> {
        Err(UnsupportedType)
    }
}

impl<'a, W> ser::SerializeStructVariant for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;
    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(UnsupportedType)
    }
    fn end(self) -> Result<()> {
        Err(UnsupportedType)
    }
}
