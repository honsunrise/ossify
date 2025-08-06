pub mod flatten;
mod key;
mod pair;

use std::borrow::Cow;
use std::{fmt, io, mem, str};

use serde::{Serialize, ser};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("invalid UTF-8: {0}")]
    Utf8(#[from] str::Utf8Error),

    #[error("top-level serializer supports only maps, [(k, v),] and structs")]
    TopLevel,

    #[error("unsupported type: {0}")]
    UnsupportedType(&'static str),

    #[error("tried to serialize a value before serializing key")]
    NoKey,

    #[error("pair error: {0}")]
    Pair(Cow<'static, str>),

    #[error("custom error: {0}")]
    Custom(Cow<'static, str>),

    #[error("tried to serialize a unit value")]
    Empty,
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(format!("{msg}").into())
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[inline]
pub fn to_writer<W, T>(writer: W, input: &T) -> Result<()>
where
    W: io::Write,
    T: ?Sized + Serialize,
{
    let mut ser = Serializer::new(writer);
    input.serialize(&mut ser)
}

#[inline]
pub fn to_vec<T>(input: &T) -> Result<Vec<u8>>
where
    T: ?Sized + Serialize,
{
    let mut writer = Vec::with_capacity(128);
    to_writer(&mut writer, input)?;
    Ok(writer)
}

#[inline]
pub fn to_string<T>(input: &T) -> Result<String>
where
    T: ?Sized + Serialize,
{
    let vec = to_vec(input)?;
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

pub struct Serializer<W> {
    writer: W,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    /// Creates a new serializer.
    #[inline]
    pub fn new(writer: W) -> Self {
        Serializer { writer }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

pub struct SeqSerializer<'a, W: io::Write> {
    writer: &'a mut W,
}

pub struct TupleSerializer<'a, W: io::Write> {
    writer: &'a mut W,
}

pub struct MapSerializer<'a, W: io::Write> {
    writer: &'a mut W,
    entries: Vec<(String, Option<Vec<u8>>)>,
}

pub struct StructSerializer<'a, W: io::Write> {
    writer: &'a mut W,
    entries: Vec<(String, Option<Vec<u8>>)>,
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, W>;
    type SerializeTuple = TupleSerializer<'a, W>;
    type SerializeMap = MapSerializer<'a, W>;
    type SerializeStruct = StructSerializer<'a, W>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_str(self, _value: &str) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<Self::Ok> {
        Err(Error::TopLevel)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.writer.write_all(name.as_bytes())?;
        self.writer.write_all(b"=")?;
        self.writer.write_all(variant.as_bytes())?;
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok> {
        let mut key_serializer = key::KeySerializer {
            writer: &mut self.writer,
        };
        variant.serialize(&mut key_serializer)?;
        self.writer.write_all(b"=")?;
        self.writer.write_all(&flatten::to_vec(value)?)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<Self::Ok> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqSerializer {
            writer: &mut self.writer,
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(TupleSerializer {
            writer: &mut self.writer,
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(MapSerializer {
            writer: &mut self.writer,
            entries: Vec::new(),
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(StructSerializer {
            writer: &mut self.writer,
            entries: Vec::new(),
        })
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct> {
        Err(Error::TopLevel)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::TopLevel)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::TopLevel)
    }
}

impl<W> ser::SerializeSeq for SeqSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(pair::PairSerializer::new(self.writer))
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<W> ser::SerializeTuple for TupleSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(pair::PairSerializer::new(self.writer))
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<W> ser::SerializeMap for MapSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_entry<K: ?Sized + ser::Serialize, V: ?Sized + ser::Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<()> {
        // Serialize key to string
        let mut key_writer = Vec::with_capacity(128);
        let mut key_serializer = key::KeySerializer {
            writer: &mut key_writer,
        };
        key.serialize(&mut key_serializer)?;
        let key_str = String::from_utf8(key_writer).map_err(|_| Error::Custom("invalid UTF-8 key".into()))?;

        // Serialize value to check if it's empty (None values produce empty output)
        let serialized_value = match flatten::to_vec(value) {
            Ok(value) => {
                // Skip field entirely if value is empty (i.e., None or empty)
                if value.is_empty() {
                    return Ok(());
                }
                Some(value)
            },
            Err(Error::Empty) => None,
            Err(e) => return Err(e),
        };

        self.entries.push((key_str, serialized_value));
        Ok(())
    }

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, _key: &T) -> Result<()> {
        // We collect all entries, so this is handled in serialize_entry
        Ok(())
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, _value: &T) -> Result<()> {
        // We collect all entries, so this is handled in serialize_entry
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok> {
        // Sort entries by key alphabetically
        self.entries.sort_by(|a, b| a.0.cmp(&b.0));

        // Write sorted entries
        let mut first = true;
        for (key, value) in self.entries {
            if !mem::replace(&mut first, false) {
                self.writer.write_all(b"&")?;
            }
            self.writer.write_all(key.as_bytes())?;
            if let Some(value) = value {
                self.writer.write_all(b"=")?;
                self.writer.write_all(&value)?;
            }
        }
        Ok(())
    }
}

impl<W> ser::SerializeStruct for StructSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, key: &'static str, value: &T) -> Result<()> {
        // Serialize value to check if it's empty (None values produce empty output)
        let serialized_value = match flatten::to_vec(value) {
            Ok(value) => {
                // Skip field entirely if value is empty (i.e., None or empty)
                if value.is_empty() {
                    return Ok(());
                }
                Some(value)
            },
            Err(Error::Empty) => None,
            Err(e) => return Err(e),
        };

        self.entries.push((key.to_string(), serialized_value));
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok> {
        // Sort entries by key alphabetically
        self.entries.sort_by(|a, b| a.0.cmp(&b.0));

        // Write sorted entries
        let mut first = true;
        for (key, value) in self.entries {
            if !mem::replace(&mut first, false) {
                self.writer.write_all(b"&")?;
            }
            self.writer.write_all(key.as_bytes())?;
            if let Some(value) = value {
                self.writer.write_all(b"=")?;
                self.writer.write_all(&value)?;
            }
        }
        Ok(())
    }
}

pub(crate) fn percent_encode(input: &str) -> Cow<'_, str> {
    use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};

    const FRAGMENT: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');

    let mut encode_iter = utf8_percent_encode(input, FRAGMENT);
    match encode_iter.next() {
        None => "".into(),
        Some(first) => match encode_iter.next() {
            None => first.into(),
            Some(second) => {
                let mut string = first.to_owned();
                string.push_str(second);
                string.extend(encode_iter);
                string.into()
            },
        },
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct OnlyKeyField;

impl ser::Serialize for OnlyKeyField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_unit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percent_encode() {
        let test_cases = vec![
            ("<_._>~01abc_-.+", "%3C_._%3E~01abc_-.%2B"),
            ("++++~01abc_-.+", "%2B%2B%2B%2B~01abc_-.%2B"),
            ("++01++", "%2B%2B01%2B%2B"),
            ("+0+", "%2B0%2B"),
            ("0+0", "0%2B0"),
            ("+ *", "%2B%20%2A"),
            ("* +", "%2A%20%2B"),
        ];

        for (uri, expected) in test_cases {
            let canonical: Cow<'_, str> = percent_encode(uri);
            assert_eq!(canonical, expected);
        }
    }

    #[test]
    fn test_struct() {
        #[derive(Serialize)]
        struct TestStruct {
            z: u32,
            y: u32,
            x: TestInnerStruct,
            w: (&'static str, u32),
            v: (),
            t: Option<u32>,
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

        let u = TestStruct {
            z: 1,
            y: 2,
            x: TestInnerStruct {
                inner1: "test",
                inner2: TestLeafStruct { l1: "aaa", l2: 3 },
                inner3: [3, 4, 5],
            },
            w: ("test+*", 1),
            v: (),
            t: None,
        };
        let expected = r#"v&w={"1":"test%2B%2A","2":"1"}&x={"inner1":"test","inner2.l1":"aaa","inner2.l2":"3","inner3.1":"3","inner3.2":"4","inner3.3":"5"}&y=2&z=1"#;
        assert_eq!(to_string(&u).unwrap(), expected);
    }

    #[test]
    fn test_skip_none_values() {
        #[derive(Serialize)]
        struct TestStruct {
            a: u32,
            d: u32,
            b: Option<u32>,
            c: Option<String>,
        }

        let u = TestStruct {
            a: 1,
            b: None,
            c: None,
            d: 2,
        };

        // None values should be completely skipped, not serialized as empty
        let expected = "a=1&d=2";
        assert_eq!(to_string(&u).unwrap(), expected);

        // Test with some values present
        let u2 = TestStruct {
            a: 1,
            b: Some(42),
            c: None,
            d: 2,
        };

        let expected2 = "a=1&b=42&d=2";
        assert_eq!(to_string(&u2).unwrap(), expected2);
    }
}
