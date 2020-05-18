// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use core::any::Any;
use core::convert::TryInto;
use std::collections::BTreeMap;
use std::io;
use std::sync::Arc;

use super::tlv;
use super::{Error, EvenOdd, Unmarshall, UnmarshallFn};
use crate::common::AsAny;
use crate::lnp::presentation::tlv::Stream;
use crate::strict_encoding::StrictDecode;

wrapper!(
    Type,
    u16,
    doc = "Message type field value",
    derive = [Copy, PartialEq, Eq, PartialOrd, Ord, Hash]
);

impl EvenOdd for Type {}

pub struct Payload(Vec<Arc<dyn Any>>);

pub trait Message: AsAny {
    fn get_type(&self) -> Type;

    fn to_type<T>(&self) -> T
    where
        Self: Sized,
        Type: Into<T>,
    {
        self.get_type().into()
    }

    fn try_to_type<T>(&self) -> Result<T, <Type as TryInto<T>>::Error>
    where
        Self: Sized,
        Type: TryInto<T>,
    {
        self.get_type().try_into()
    }

    fn get_payload(&self) -> Payload;

    fn get_tlvs(&self) -> tlv::Stream;
}

pub struct RawMessage {
    pub type_id: Type,
    pub payload: Vec<u8>,
}

impl AsAny for RawMessage {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Message for RawMessage {
    fn get_type(&self) -> Type {
        self.type_id
    }

    fn get_payload(&self) -> Payload {
        Payload(vec![Arc::new(self.payload.clone())])
    }

    fn get_tlvs(&self) -> Stream {
        Stream::new()
    }
}

pub struct Unmarshaller<R>
where
    R: io::Read,
{
    known_types: BTreeMap<Type, UnmarshallFn<R, Error>>,
}

impl<R> Unmarshall<R, Arc<dyn Any>> for Unmarshaller<R>
where
    R: io::Read,
{
    type Error = Error;

    fn unmarshall(&self, mut reader: R) -> Result<Arc<dyn Any>, Self::Error> {
        let type_id = Type(u16::strict_decode(&mut reader).map_err(|_| Error::NoData)?);
        match self.known_types.get(&type_id) {
            None if type_id.is_even() => Err(Error::MessageEvenType),
            None => {
                let mut payload = Vec::new();
                reader.read_to_end(&mut payload)?;
                Ok(Arc::new(RawMessage { type_id, payload }))
            }
            Some(parser) => parser(&mut reader),
        }
    }
}

impl<R> Unmarshaller<R>
where
    R: io::Read,
{
    pub fn new() -> Self {
        Self {
            known_types: BTreeMap::new(),
        }
    }
}