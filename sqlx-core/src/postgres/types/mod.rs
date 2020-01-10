mod bool;
mod bytes;
mod float;
mod int;
mod str;

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "uuid")]
mod uuid;

use byteorder::NetworkEndian;

use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::io::BufMut;
use crate::postgres::Postgres;
use crate::types::HasSqlType;

// Arrays
// https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=src/include/utils/array.h;h=7f7e744cb12bc872f628f90dad99dfdf074eb314;hb=master#l6

impl<T> Encode<Postgres> for [T]
where
    T: Encode<Postgres>,
    Postgres: HasSqlType<T>,
{
    fn encode(&self, buf: &mut Vec<u8>) {
        let elem_type = <Postgres as HasSqlType<T>>::id();

        todo!()
    }
}

impl<T> Decode<Postgres> for Vec<T>
where
    T: Decode<Postgres>,
    Postgres: HasSqlType<T>,
{
    fn decode(mut buf: &[u8]) -> Result<Self, DecodeError> {
        todo!()
    }
}
