//! Types and traits for encoding values to the database.

use crate::database::Database;
use crate::types::HasSqlType;
use std::mem;

/// The return type of [Encode::encode].
pub enum IsNull {
    /// The value is null; no data was written.
    Yes,

    /// The value is not null.
    ///
    /// This does not mean that data was written.
    No,
}

/// Encode a single value to be sent to the database.
pub trait Encode<DB>
where
    DB: Database + ?Sized,
{
    /// Writes the value of `self` into `buf` in the expected format for the database.
    fn encode(&self, buf: &mut Vec<u8>);

    fn encode_nullable(&self, buf: &mut Vec<u8>) -> IsNull {
        self.encode(buf);

        IsNull::No
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        mem::size_of::<Self>()
    }
}

impl<T: ?Sized, DB> Encode<DB> for &'_ T
where
    DB: Database + HasSqlType<T>,
    T: Sized + Encode<DB>,
{
    fn encode(&self, buf: &mut Vec<u8>) {
        (*self).encode(buf)
    }

    fn encode_nullable(&self, buf: &mut Vec<u8>) -> IsNull {
        (*self).encode_nullable(buf)
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        T::size_hint()
    }
}

impl<T, DB> Encode<DB> for Option<T>
where
    DB: Database + HasSqlType<T>,
    T: Encode<DB>,
{
    fn encode(&self, buf: &mut Vec<u8>) {
        // Forward to [encode_nullable] and ignore the result
        let _ = self.encode_nullable(buf);
    }

    fn encode_nullable(&self, buf: &mut Vec<u8>) -> IsNull {
        if let Some(self_) = self {
            self_.encode(buf);

            IsNull::No
        } else {
            IsNull::Yes
        }
    }

    fn size_hint() -> usize
    where
        Self: Sized,
    {
        // size is 0 if None but we assume Some
        mem::size_of::<T>()
    }
}
