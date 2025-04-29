use bitflags::bitflags;

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_derive::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct U32ValueFlags {
    pub value: u32,
    pub max_flags: u8,
    pub flags: Max32Flags,
}

impl U32ValueFlags {
    pub fn from_u32(data: u32, max_flags: u8) -> Self {
        let value = data & ((1 << max_flags) - 1); // low bits -> value
        let flag = (data >> max_flags) as u32; // high bits -> flags

        if max_flags < 32 {
            return U32ValueFlags {
                value,
                max_flags,
                flags: Max32Flags::from_bits_truncate(flag),
            };
        }

        panic!("Unsupported max_flags for u32, max_flags: {max_flags}, allow: [1, 31]")
    }

    pub fn to_u32(&self) -> u32 {
        let flag_bits = self.flags.bits();
        ((flag_bits & ((1 << (32 - self.max_flags)) - 1)) << self.max_flags)
            | (self.value & ((1 << self.max_flags) - 1))
    }
}

bitflags! {
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    #[serde(transparent)]
    pub struct Max32Flags: u32 {
        const BIT_STATUS_00 = 0b00000000000000000000000000000001;
        const BIT_STATUS_01 = 0b00000000000000000000000000000010;
        const BIT_STATUS_02 = 0b00000000000000000000000000000100;
        const BIT_STATUS_03 = 0b00000000000000000000000000001000;
        const BIT_STATUS_04 = 0b00000000000000000000000000010000;
        const BIT_STATUS_05 = 0b00000000000000000000000000100000;
        const BIT_STATUS_06 = 0b00000000000000000000000001000000;
        const BIT_STATUS_07 = 0b00000000000000000000000010000000;
        const BIT_STATUS_08 = 0b00000000000000000000000100000000;
        const BIT_STATUS_09 = 0b00000000000000000000001000000000;
        const BIT_STATUS_10 = 0b00000000000000000000010000000000;
        const BIT_STATUS_11 = 0b00000000000000000000100000000000;
        const BIT_STATUS_12 = 0b00000000000000000001000000000000;
        const BIT_STATUS_13 = 0b00000000000000000010000000000000;
        const BIT_STATUS_14 = 0b00000000000000000100000000000000;
        const BIT_STATUS_15 = 0b00000000000000001000000000000000;
        const BIT_STATUS_16 = 0b00000000000000010000000000000000;
        const BIT_STATUS_17 = 0b00000000000000100000000000000000;
        const BIT_STATUS_18 = 0b00000000000001000000000000000000;
        const BIT_STATUS_19 = 0b00000000000010000000000000000000;
        const BIT_STATUS_20 = 0b00000000000100000000000000000000;
        const BIT_STATUS_21 = 0b00000000001000000000000000000000;
        const BIT_STATUS_22 = 0b00000000010000000000000000000000;
        const BIT_STATUS_23 = 0b00000000100000000000000000000000;
        const BIT_STATUS_24 = 0b00000001000000000000000000000000;
        const BIT_STATUS_25 = 0b00000010000000000000000000000000;
        const BIT_STATUS_26 = 0b00000100000000000000000000000000;
        const BIT_STATUS_27 = 0b00001000000000000000000000000000;
        const BIT_STATUS_28 = 0b00010000000000000000000000000000;
        const BIT_STATUS_29 = 0b00100000000000000000000000000000;
        const BIT_STATUS_30 = 0b01000000000000000000000000000000;
        const BIT_STATUS_31 = 0b10000000000000000000000000000000;
    }
}

pub const U32_FLAG_INDEXES: [Max32Flags; 32] = [
    Max32Flags::BIT_STATUS_00,
    Max32Flags::BIT_STATUS_01,
    Max32Flags::BIT_STATUS_02,
    Max32Flags::BIT_STATUS_03,
    Max32Flags::BIT_STATUS_04,
    Max32Flags::BIT_STATUS_05,
    Max32Flags::BIT_STATUS_06,
    Max32Flags::BIT_STATUS_07,
    Max32Flags::BIT_STATUS_08,
    Max32Flags::BIT_STATUS_09,
    Max32Flags::BIT_STATUS_10,
    Max32Flags::BIT_STATUS_11,
    Max32Flags::BIT_STATUS_12,
    Max32Flags::BIT_STATUS_13,
    Max32Flags::BIT_STATUS_14,
    Max32Flags::BIT_STATUS_15,
    Max32Flags::BIT_STATUS_16,
    Max32Flags::BIT_STATUS_17,
    Max32Flags::BIT_STATUS_18,
    Max32Flags::BIT_STATUS_19,
    Max32Flags::BIT_STATUS_20,
    Max32Flags::BIT_STATUS_21,
    Max32Flags::BIT_STATUS_22,
    Max32Flags::BIT_STATUS_23,
    Max32Flags::BIT_STATUS_24,
    Max32Flags::BIT_STATUS_25,
    Max32Flags::BIT_STATUS_26,
    Max32Flags::BIT_STATUS_27,
    Max32Flags::BIT_STATUS_28,
    Max32Flags::BIT_STATUS_29,
    Max32Flags::BIT_STATUS_30,
    Max32Flags::BIT_STATUS_31,
];
