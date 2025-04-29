use serde::{Deserialize, Serialize};

use crate::constraints::{
    BooleanConstraints, BytesConstraints, EnumConstraints, NumericConstraints,
    NumericFlagsConstraints, StringConstraints,
};
use crate::types::{u16_flags::U16ValueFlags, u32_flags::U32ValueFlags, u64_flags::U64ValueFlags};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RegisterValueType {
    Coils(BooleanConstraints),
    Discrete(BooleanConstraints),
    U8(NumericConstraints<u8>),
    U16(NumericConstraints<u16>),
    U32(NumericConstraints<u32>),
    U64(NumericConstraints<u64>),
    U16Flags(NumericFlagsConstraints<U16ValueFlags, u16>),
    U32Flags(NumericFlagsConstraints<U32ValueFlags, u32>),
    U64Flags(NumericFlagsConstraints<U64ValueFlags, u64>),
    Bytes(BytesConstraints),
    String(StringConstraints),
    Enum(EnumConstraints<String>),
}
