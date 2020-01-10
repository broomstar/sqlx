//! Traits linking Rust types to SQL types.

use crate::Database;

#[cfg(feature = "uuid")]
pub use uuid::Uuid;

#[cfg(feature = "chrono")]
pub mod chrono {
    pub use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
}

/// Indicates that a SQL type is supported for a database.
pub trait HasSqlType<T: ?Sized>: Database {
    /// Returns the canonical type identifier on the database for the type `T`.
    fn id() -> Self::TypeId {
        Self::compatible()[0].clone()
    }

    /// Returns a list of compatible type identifiers
    /// that can be used in place of `id` but still encode to
    /// or decode from the rust type `T`.
    fn compatible() -> &'static [Self::TypeId];
}

// For references to types in Rust, the underlying SQL type information
// is equivalent
impl<T: ?Sized, DB> HasSqlType<&'_ T> for DB
where
    DB: HasSqlType<T>,
{
    fn id() -> Self::TypeId {
        <DB as HasSqlType<T>>::id()
    }

    fn compatible() -> &'static [Self::TypeId] {
        <DB as HasSqlType<T>>::compatible()
    }
}

// For optional types in Rust, the underlying SQL type information
// is equivalent
impl<T, DB> HasSqlType<Option<T>> for DB
where
    DB: HasSqlType<T>,
{
    fn id() -> Self::TypeId {
        <DB as HasSqlType<T>>::id()
    }

    fn compatible() -> &'static [Self::TypeId] {
        <DB as HasSqlType<T>>::compatible()
    }
}
