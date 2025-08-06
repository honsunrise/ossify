use std::borrow::Cow;
use std::{io, mem};

use serde::ser;

use super::{Error, flatten, key};

enum PairState<'a> {
    WaitingForKey,
    WaitingForValue { key: Cow<'a, str> },
    Done,
}

pub struct PairSerializer<'a, W> {
    writer: &'a mut W,
    state: PairState<'a>,
}

impl<'a, W> PairSerializer<'a, W>
where
    W: io::Write,
{
    pub(crate) fn new(writer: &'a mut W) -> Self {
        Self {
            writer,
            state: PairState::WaitingForKey,
        }
    }
}

impl<W> ser::Serializer for PairSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;
    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_i8(self, _v: i8) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_i16(self, _v: i16) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_i32(self, _v: i32) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_i64(self, _v: i64) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_u8(self, _v: u8) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_u16(self, _v: u16) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_u32(self, _v: u32) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_u64(self, _v: u64) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_f32(self, _v: f32) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_f64(self, _v: f64) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_char(self, _v: char) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_str(self, _value: &str) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_unit(self) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_none(self) -> Result<(), Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(self, value: &T) -> Result<(), Error> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_tuple(self, len: usize) -> Result<Self, Error> {
        if len == 2 {
            Ok(self)
        } else {
            Err(Error::unsupported_pair())
        }
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Error> {
        Err(Error::unsupported_pair())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(Error::unsupported_pair())
    }
}

impl<W> ser::SerializeTuple for PairSerializer<'_, W>
where
    W: io::Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<(), Error> {
        match mem::replace(&mut self.state, PairState::Done) {
            PairState::WaitingForKey => {
                let mut writer = Vec::with_capacity(128);
                let mut key_serializer = key::KeySerializer { writer: &mut writer };
                value.serialize(&mut key_serializer)?;
                self.state = PairState::WaitingForValue {
                    key: unsafe { String::from_utf8_unchecked(writer).into() },
                };
                Ok(())
            },
            PairState::WaitingForValue { key } => {
                let result = {
                    let mut value_serializer = flatten::FlattenSerializer::new(self.writer);
                    value.serialize(&mut value_serializer)
                };
                // recover the state if the value serialization fails
                if result.is_err() {
                    self.state = PairState::WaitingForValue { key };
                    return result;
                }
                Ok(())
            },
            PairState::Done => Err(Error::done()),
        }
    }

    fn end(self) -> Result<(), Error> {
        if let PairState::Done = self.state {
            Ok(())
        } else {
            Err(Error::not_done())
        }
    }
}

impl Error {
    fn done() -> Self {
        Error::Pair("has already been serialized".into())
    }

    fn not_done() -> Self {
        Error::Pair("has not yet been serialized".into())
    }

    fn unsupported_pair() -> Self {
        Error::Pair("unsupported".into())
    }
}
