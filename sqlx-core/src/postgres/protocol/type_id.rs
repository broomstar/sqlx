use std::fmt::{self, Display};

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub struct TypeId(pub(crate) u32);

impl TypeId {
    // Scalar

    pub(crate) const BOOL: TypeId = TypeId(16);

    pub(crate) const INT2: TypeId = TypeId(21);
    pub(crate) const INT4: TypeId = TypeId(23);
    pub(crate) const INT8: TypeId = TypeId(20);

    pub(crate) const FLOAT4: TypeId = TypeId(700);
    pub(crate) const FLOAT8: TypeId = TypeId(701);

    pub(crate) const TEXT: TypeId = TypeId(25);

    pub(crate) const DATE: TypeId = TypeId(1082);
    pub(crate) const TIME: TypeId = TypeId(1083);
    pub(crate) const TIMESTAMP: TypeId = TypeId(1114);
    pub(crate) const TIMESTAMPTZ: TypeId = TypeId(1184);

    pub(crate) const BYTEA: TypeId = TypeId(17);

    pub(crate) const UUID: TypeId = TypeId(2950);

    // Arrays

    pub(crate) const ARRAY_BOOL: TypeId = TypeId(1000);

    pub(crate) const ARRAY_INT2: TypeId = TypeId(1005);
    pub(crate) const ARRAY_INT4: TypeId = TypeId(1007);
    pub(crate) const ARRAY_INT8: TypeId = TypeId(1016);

    pub(crate) const ARRAY_FLOAT4: TypeId = TypeId(1021);
    pub(crate) const ARRAY_FLOAT8: TypeId = TypeId(1022);

    pub(crate) const ARRAY_TEXT: TypeId = TypeId(1009);

    pub(crate) const ARRAY_DATE: TypeId = TypeId(1182);
    pub(crate) const ARRAY_TIME: TypeId = TypeId(1183);
    pub(crate) const ARRAY_TIMESTAMP: TypeId = TypeId(1115);
    pub(crate) const ARRAY_TIMESTAMPTZ: TypeId = TypeId(1185);

    pub(crate) const ARRAY_BYTEA: TypeId = TypeId(1001);

    pub(crate) const ARRAY_UUID: TypeId = TypeId(2951);
}

impl Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            // Scalar
            //
            TypeId::BOOL => "BOOL",

            TypeId::INT2 => "INT2",
            TypeId::INT4 => "INT4",
            TypeId::INT8 => "INT8",

            TypeId::FLOAT4 => "FLOAT4",
            TypeId::FLOAT8 => "FLOAT8",

            TypeId::TEXT => "TEXT",

            TypeId::DATE => "DATE",
            TypeId::TIME => "TIME",
            TypeId::TIMESTAMP => "TIMESTAMP",
            TypeId::TIMESTAMPTZ => "TIMESTAMPTZ",

            TypeId::BYTEA => "BYTEA",

            TypeId::UUID => "UUID",

            // Array
            //
            TypeId::ARRAY_BOOL => "BOOL[]",

            TypeId::ARRAY_INT2 => "INT2[]",
            TypeId::ARRAY_INT4 => "INT4[]",
            TypeId::ARRAY_INT8 => "INT8[]",

            TypeId::ARRAY_FLOAT4 => "FLOAT4[]",
            TypeId::ARRAY_FLOAT8 => "FLOAT8[]",

            TypeId::ARRAY_TEXT => "TEXT[]",

            TypeId::ARRAY_DATE => "DATE[]",
            TypeId::ARRAY_TIME => "TIME[]",
            TypeId::ARRAY_TIMESTAMP => "TIMESTAMP[]",
            TypeId::ARRAY_TIMESTAMPTZ => "TIMESTAMPTZ[]",

            TypeId::ARRAY_BYTEA => "BYTEA[]",

            TypeId::ARRAY_UUID => "UUID[]",

            _ => {
                return write!(f, "{}", self.0);
            }
        };

        write!(f, "{}", name)
    }
}
