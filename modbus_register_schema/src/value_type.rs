use serde::{Deserialize, Serialize};

use crate::constraints::{
    BooleanConstraints, BytesConstraints, EnumConstraints, NumericConstraints, StringConstraints,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RegisterValueType {
    Coils(BooleanConstraints),
    Discrete(BooleanConstraints),
    U8(NumericConstraints<u8>),
    U16(NumericConstraints<u16>),
    U32(NumericConstraints<u32>),
    U64(NumericConstraints<u64>),
    Bytes(BytesConstraints),
    String(StringConstraints),
    Enum(EnumConstraints<String>),
}
