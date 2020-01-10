use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::postgres::{PgTypeId, Postgres};
use crate::types::HasSqlType;

impl HasSqlType<f32> for Postgres {
    fn compatible() -> &'static [PgTypeId] {
        &[PgTypeId::FLOAT4]
    }
}

impl Encode<Postgres> for f32 {
    fn encode(&self, buf: &mut Vec<u8>) {
        <i32 as Encode<Postgres>>::encode(&(self.to_bits() as i32), buf)
    }
}

impl Decode<Postgres> for f32 {
    fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(f32::from_bits(
            <i32 as Decode<Postgres>>::decode(buf)? as u32
        ))
    }
}

impl HasSqlType<f64> for Postgres {
    fn compatible() -> &'static [PgTypeId] {
        &[PgTypeId::FLOAT8]
    }
}

impl Encode<Postgres> for f64 {
    fn encode(&self, buf: &mut Vec<u8>) {
        <i64 as Encode<Postgres>>::encode(&(self.to_bits() as i64), buf)
    }
}

impl Decode<Postgres> for f64 {
    fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(f64::from_bits(
            <i64 as Decode<Postgres>>::decode(buf)? as u64
        ))
    }
}
