use std::borrow::Cow;
use std::{io, mem, str};

use serde::{Serialize, ser};

use super::{Error, percent_encode};

pub struct FlattenSerializer<'a, W> {
    writer: &'a mut W,

    key: Option<Cow<'a, str>>,
    top_level: bool,
}

impl<'a, W> FlattenSerializer<'a, W>
where
    W: io::Write,
{
    pub(crate) fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
            key: None,
            top_level: true,
        }
    }

    fn serialize_integer<I>(&'a mut self, value: I) -> Result<(), Error>
    where
        I: itoa::Integer,
    {
        let mut buf = itoa::Buffer::new();
        let part = buf.format(value);
        ser::Serializer::serialize_str(self, part)
    }

    fn serialize_floating<RF>(&'a mut self, value: RF) -> Result<(), Error>
    where
        RF: ryu::Float,
    {
        let mut buf = ryu::Buffer::new();
        let part = buf.format(value);
        ser::Serializer::serialize_str(self, part)
    }

    fn serialize_key(&mut self) -> Result<(), Error> {
        if let Some(key) = &self.key {
            self.writer.write_all(b"\"")?;
            self.writer.write_all(key.as_bytes())?;
            self.writer.write_all(b"\":")?;
        }
        Ok(())
    }
}

impl<'a, W> ser::Serializer for &'a mut FlattenSerializer<'a, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = FlattenSeqSerializer<'a, W>;
    type SerializeTuple = FlattenTupleSerializer<'a, W>;
    type SerializeMap = FlattenMapSerializer<'a, W>;
    type SerializeStruct = FlattenStructSerializer<'a, W>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(if v { "true" } else { "false" })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        self.serialize_integer(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_floating(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_floating(v)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let percent_encoding_v = percent_encode(v);
        self.serialize_key()?;
        if !self.top_level {
            self.writer.write_all(b"\"")?;
        }
        self.writer.write_all(percent_encoding_v.as_bytes())?;
        if !self.top_level {
            self.writer.write_all(b"\"")?;
        }
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut char_buf = [0; 4];
        let char_bytes = v.encode_utf8(&mut char_buf);
        self.serialize_str(char_bytes)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(str::from_utf8(v)?)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::Empty)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_key()?;
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_key()?;
        if self.key.is_some() {
            self.writer.write_all(b"\"\"")?;
        }
        Ok(())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        self.serialize_key()?;
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        if self.top_level {
            self.writer.write_all(b"{")?;
        }
        Ok(FlattenSeqSerializer {
            writer: self.writer,
            pre_key: self.key.clone(),
            first: true,
            index: 0,
            top_level: self.top_level,
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        if self.top_level {
            self.writer.write_all(b"{")?;
        }
        Ok(FlattenTupleSerializer {
            writer: self.writer,
            pre_key: self.key.clone(),
            first: true,
            index: 0,
            top_level: self.top_level,
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(FlattenMapSerializer {
            writer: self.writer,
            pre_key: self.key.clone(),
            entries: Vec::new(),
            top_level: self.top_level,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(FlattenStructSerializer {
            writer: self.writer,
            pre_key: self.key.clone(),
            entries: Vec::new(),
            top_level: self.top_level,
        })
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::UnsupportedType("newtype variant"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::UnsupportedType("tuple struct"))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::UnsupportedType("tuple variant"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::UnsupportedType("struct variant"))
    }
}

pub struct FlattenSeqSerializer<'a, W> {
    writer: &'a mut W,

    first: bool,
    index: usize,
    top_level: bool,
    pre_key: Option<Cow<'a, str>>,
}

impl<W> ser::SerializeSeq for FlattenSeqSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let first = mem::replace(&mut self.first, false);
        if !first {
            self.writer.write_all(b",")?;
        }

        self.index += 1;
        let mut buf = itoa::Buffer::new();
        let part = buf.format(self.index);
        let new_key: Cow<'_, str> = if let Some(pre_key) = &self.pre_key {
            format!("{pre_key}.{part}").into()
        } else {
            part.into()
        };

        let mut value_serializer = FlattenSerializer {
            writer: self.writer,
            key: Some(new_key),
            top_level: false,
        };
        value.serialize(&mut value_serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.top_level {
            self.writer.write_all(b"}")?;
        }
        Ok(())
    }
}

pub struct FlattenTupleSerializer<'a, W> {
    writer: &'a mut W,

    pre_key: Option<Cow<'a, str>>,
    first: bool,
    index: usize,
    top_level: bool,
}

impl<W> ser::SerializeTuple for FlattenTupleSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let first = mem::replace(&mut self.first, false);
        if !first {
            self.writer.write_all(b",")?;
        }

        self.index += 1;
        let mut buf = itoa::Buffer::new();
        let part = buf.format(self.index);
        let new_key: Cow<'_, str> = if let Some(pre_key) = &self.pre_key {
            format!("{pre_key}.{part}").into()
        } else {
            part.into()
        };

        let mut value_serializer = FlattenSerializer {
            writer: self.writer,
            key: Some(new_key),
            top_level: false,
        };
        value.serialize(&mut value_serializer)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        if self.top_level {
            self.writer.write_all(b"}")?;
        }
        Ok(())
    }
}

pub struct FlattenMapSerializer<'a, W> {
    writer: &'a mut W,

    pre_key: Option<Cow<'a, str>>,
    entries: Vec<(String, Vec<u8>)>,
    top_level: bool,
}

impl<W> ser::SerializeMap for FlattenMapSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_entry<K: ?Sized + ser::Serialize, V: ?Sized + ser::Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<(), Self::Error> {
        // Collect keys for sorting - we need to serialize to string
        let mut key_writer = Vec::new();
        let mut key_serializer = FlattenSerializer {
            writer: &mut key_writer,
            key: None,
            top_level: false,
        };
        key.serialize(&mut key_serializer)?;
        let key_str = String::from_utf8(key_writer).map_err(|_| Error::Custom("invalid UTF-8 key".into()))?;

        let new_key: Cow<'_, str> = if let Some(pre_key) = &self.pre_key {
            format!("{pre_key}.{key_str}").into()
        } else {
            Cow::Borrowed(&key_str)
        };

        let mut value_writer = Vec::new();
        let mut value_serializer = FlattenSerializer {
            writer: &mut value_writer,
            key: Some(new_key),
            top_level: false,
        };
        value.serialize(&mut value_serializer)?;
        self.entries.push((key_str, value_writer));
        Ok(())
    }

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
        // We collect all entries, so this is handled in serialize_entry
        Ok(())
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        // We collect all entries, so this is handled in serialize_entry
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if self.top_level {
            self.writer.write_all(b"{")?;
        }

        self.entries.sort_by(|a, b| a.0.cmp(&b.0));

        for (i, (_, value)) in self.entries.iter().enumerate() {
            self.writer.write_all(value)?;
            if i < self.entries.len() - 1 {
                self.writer.write_all(b",")?;
            }
        }

        if self.top_level {
            self.writer.write_all(b"}")?;
        }
        Ok(())
    }
}

pub struct FlattenStructSerializer<'a, W> {
    writer: &'a mut W,

    pre_key: Option<Cow<'a, str>>,
    entries: Vec<(String, Vec<u8>)>,
    top_level: bool,
}

impl<W> ser::SerializeStruct for FlattenStructSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        let new_key: Cow<'_, str> = if let Some(pre_key) = &self.pre_key {
            format!("{pre_key}.{key}").into()
        } else {
            key.into()
        };

        let mut value_writer = Vec::new();
        let mut value_serializer = FlattenSerializer {
            writer: &mut value_writer,
            key: Some(new_key),
            top_level: false,
        };
        // Print value_writer as string for debugging
        value.serialize(&mut value_serializer)?;
        self.entries.push((key.to_string(), value_writer));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        if self.top_level {
            self.writer.write_all(b"{")?;
        }

        self.entries.sort_by(|a, b| a.0.cmp(&b.0));

        for (i, (_, value)) in self.entries.iter().enumerate() {
            self.writer.write_all(value)?;
            if i < self.entries.len() - 1 {
                self.writer.write_all(b",")?;
            }
        }

        if self.top_level {
            self.writer.write_all(b"}")?;
        }
        Ok(())
    }
}

#[inline]
pub fn to_writer<W, T>(mut writer: W, input: &T) -> Result<(), Error>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = FlattenSerializer::new(&mut writer);
    input.serialize(&mut ser)
}

#[inline]
pub fn to_vec<T>(input: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(512);
    to_writer(&mut writer, input)?;
    Ok(writer)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Serialize)]
    struct TestStruct {
        z: u32,
        y: u32,
        x: TestInnerStruct,
        w: (&'static str, u32),
    }

    #[derive(Serialize)]
    struct TestInnerStruct {
        inner1: &'static str,
        inner2: TestLeafStruct,
        inner3: [u32; 3],
    }

    #[derive(Serialize)]
    struct TestLeafStruct {
        l2: u32,
        l1: &'static str,
    }

    #[test]
    fn test_flatten_serializer() {
        let u = TestStruct {
            z: 1,
            y: 2,
            x: TestInnerStruct {
                inner1: "test",
                inner2: TestLeafStruct { l1: "aaa", l2: 3 },
                inner3: [3, 4, 5],
            },
            w: ("test", 1),
        };
        let result = to_vec(&u).unwrap();
        // Verify keys are sorted alphabetically
        assert_eq!(
            result,
            br#"{"w.1":"test","w.2":"1","x.inner1":"test","x.inner2.l1":"aaa","x.inner2.l2":"3","x.inner3.1":"3","x.inner3.2":"4","x.inner3.3":"5","y":"2","z":"1"}"#
        );
    }
}
