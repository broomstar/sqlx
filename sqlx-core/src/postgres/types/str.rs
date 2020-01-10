use std::str;

use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::postgres::protocol::TypeId;
use crate::types::HasSqlType;
use crate::Postgres;

impl HasSqlType<str> for Postgres {
    fn compatible() -> &'static [TypeId] {
        &[TypeId::TEXT]
    }
}

impl HasSqlType<String> for Postgres {
    fn compatible() -> &'static [TypeId] {
        <Postgres as HasSqlType<str>>::compatible()
    }
}

impl Encode<Postgres> for str {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self.as_bytes());
    }
}

impl Encode<Postgres> for String {
    fn encode(&self, buf: &mut Vec<u8>) {
        <str as Encode<Postgres>>::encode(self.as_str(), buf)
    }
}

impl Decode<Postgres> for String {
    fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(str::from_utf8(buf)?.to_owned())
    }
}
