use std::io::Write;

use super::fmt::*;

use log::key_values::{Key, Value, KeyValueSource, Visitor, Error};
use serde::ser::{self, Serialize};

struct WriteKeyValueSource<'a>(&'a mut Formatter);

impl<'a, 'kvs> Visitor<'kvs> for WriteKeyValueSource<'a> {
    fn visit_pair<'vis>(&'vis mut self, k: Key<'kvs>, v: Value<'kvs>) -> Result<(), Error> {
        let property_style = self.0.property_style();
        write!(self.0, "{}", property_style.value(k))?;

        v.serialize(&mut Serializer::begin_value(&mut self.0))?;

        Ok(())
    }
}

impl Formatter {
    fn property_style(&self) -> Style {
        let mut property_style = self.style();
        property_style.set_bold(true);

        property_style
    }

    /// Write key value pairs.
    pub fn write_key_values<KVS>(&mut self, kvs: KVS)
    where
        KVS: KeyValueSource,
    {
        let _ = writeln!(self);
        let _ = kvs.visit(&mut WriteKeyValueSource(self));
    }
}

#[derive(Debug)]
struct Serializer<'a> {
    dst: &'a mut Formatter,
    path: Vec<Expecting>,
    expecting: Expecting,
    indent: String,
}

impl<'a> Serializer<'a> {
    fn begin_value(dst: &'a mut Formatter) -> Self {
        Serializer {
            dst,
            path: Vec::new(),
            expecting: Expecting::Value,
            indent: String::new(),
        }
    }

    fn write_value<T>(&mut self, value: T) -> Result<(), Invalid>
    where
        T: std::fmt::Display,
    {
        match self.expecting {
            Expecting::Key => {
                let property_style = self.dst.property_style();

                write!(
                    self.dst,
                    "{indent}{key}",
                    indent = self.indent,
                    key = property_style.value(value))?;

                self.expecting = Expecting::Value;
            }
            Expecting::Value => {
                writeln!(
                    self.dst,
                    ": {value}",
                    value = value)?;

                self.expecting = Expecting::Key;
            }
            Expecting::Elem => {
                writeln!(
                    self.dst,
                    "{indent}{value}",
                    indent = self.indent,
                    value = value)?;
            }
        }

        Ok(())
    }

    fn begin_struct(&mut self) -> Result<(), Invalid> {
        match self.expecting {
            Expecting::Value => {
                writeln!(self.dst, ":")?;

                self.indent.push_str("  ");
                self.path.push(Expecting::Key);
            }
            _ => {
                self.path.push(self.expecting);
            }
        }

        self.expecting = Expecting::Key;

        Ok(())
    }

    fn begin_seq(&mut self) -> Result<(), Invalid> {
        match self.expecting {
            Expecting::Value => {
                writeln!(self.dst, ":")?;

                self.indent.push_str("  ");
                self.path.push(Expecting::Key);
            }
            _ => {
                self.path.push(self.expecting);
            }
        }

        self.expecting = Expecting::Elem;
        
        Ok(())
    }

    fn end_struct(&mut self) -> Result<(), Invalid> {
        self.pop_path();

        Ok(())
    }

    fn end_seq(&mut self) -> Result<(), Invalid> {
        self.pop_path();

        Ok(())
    }

    fn pop_path(&mut self) {
        let expecting = self.path.pop().unwrap_or(Expecting::Key);

        if let Expecting::Key = expecting {
            let new_len = self.indent.len().saturating_sub(2);
            self.indent.truncate(new_len);
        }

        self.expecting = expecting;
    }
}

impl<'a, 'b> ser::Serializer for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Invalid;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<(), Invalid> {
        self.write_value(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> Result<(), Invalid> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<(), Invalid> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<(), Invalid> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<(), Invalid> {
        self.write_value(v)
    }

    fn serialize_u8(self, v: u8) -> Result<(), Invalid> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<(), Invalid> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<(), Invalid> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<(), Invalid> {
        self.write_value(v)
    }

    fn serialize_f32(self, v: f32) -> Result<(), Invalid> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<(), Invalid> {
        self.write_value(v)
    }

    fn serialize_char(self, v: char) -> Result<(), Invalid> {
        self.write_value(v)
    }

    fn serialize_str(self, v: &str) -> Result<(), Invalid> {
        self.write_value(v)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Invalid> {
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()?;

        Ok(())
    }

    fn serialize_none(self) -> Result<(), Invalid> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Invalid> {
        self.write_value("null")
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Invalid> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<(), Invalid> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        self.begin_struct()?;
        variant.serialize(&mut *self)?;
        value.serialize(&mut *self)?;
        self.end_struct()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Invalid> {
        self.begin_seq()?;

        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Invalid> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Invalid> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Invalid> {
        self.begin_struct()?;
        variant.serialize(&mut *self)?;
        self.begin_seq()?;

        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Invalid> {
        self.begin_struct()?;

        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Invalid> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Invalid> {
        self.begin_struct()?;
        variant.serialize(&mut *self)?;
        self.begin_struct()?;

        Ok(self)
    }
}

impl<'a, 'b> ser::SerializeSeq for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Invalid;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Invalid> {
        self.end_seq()
    }
}

impl<'a, 'b> ser::SerializeTuple for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Invalid;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Invalid> {
        self.end_seq()
    }
}

impl<'a, 'b> ser::SerializeTupleStruct for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Invalid;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Invalid> {
        self.end_seq()
    }
}

impl<'a, 'b> ser::SerializeTupleVariant for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Invalid;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Invalid> {
        self.end_seq()?;
        self.end_struct()?;

        Ok(())
    }
}

impl<'a, 'b> ser::SerializeMap for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Invalid;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Invalid> {
        self.end_struct()
    }
}

impl<'a, 'b> ser::SerializeStruct for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Invalid;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Invalid> {
        self.end_struct()
    }
}

impl<'a, 'b> ser::SerializeStructVariant for &'a mut Serializer<'b> {
    type Ok = ();
    type Error = Invalid;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Invalid>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Invalid> {
        self.end_struct()?;
        self.end_struct()
    }
}

#[derive(Debug)]
struct Invalid(String);

impl ser::Error for Invalid {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display
    {
        Invalid(msg.to_string())
    }
}

impl std::fmt::Display for Invalid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Invalid {
    fn cause(&self) -> Option<&std::error::Error> {
        None
    }

    fn description(&self) -> &str {
        &self.0
    }
}

impl From<std::io::Error> for Invalid {
    fn from(err: std::io::Error) -> Self {
        Invalid(err.to_string())
    }
}

#[derive(Debug, Clone, Copy)]
enum Expecting {
    Key,
    Value,
    Elem,
}
