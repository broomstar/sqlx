use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::postgres::protocol::TypeId;
use crate::postgres::Postgres;
use crate::types::HasSqlType;

impl HasSqlType<[u8]> for Postgres {
    fn compatible() -> &'static [TypeId] {
        &[TypeId::BYTEA]
    }
}

impl HasSqlType<Vec<u8>> for Postgres {
    fn compatible() -> &'static [TypeId] {
        <Postgres as HasSqlType<[u8]>>::compatible()
    }
}

impl Encode<Postgres> for [u8] {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(self);
    }
}

impl Encode<Postgres> for Vec<u8> {
    fn encode(&self, buf: &mut Vec<u8>) {
        <[u8] as Encode<Postgres>>::encode(self, buf);
    }
}

impl Decode<Postgres> for Vec<u8> {
    fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(buf.to_vec())
    }
}
