use bitflags::bitflags;

use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_derive::*;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct U16ValueFlags {
    pub value: u16,
    pub max_flags: u8,
    pub flags: Max16Flags,
}

impl U16ValueFlags {
    pub fn from_u16(data: u16, max_flags: u8) -> Self {
        let value = data & ((1 << max_flags) - 1); // low bits -> value
        let flag = (data >> max_flags) as u16; // high bits -> flags

        if max_flags < 16 {
            return U16ValueFlags {
                value,
                max_flags,
                flags: Max16Flags::from_bits_truncate(flag),
            };
        }

        panic!("Unsupported max_flags for u16, max_flags: {max_flags}, allow: [1, 15]")
    }

    pub fn to_u16(&self) -> u16 {
        let flag_bits = self.flags.bits();
        ((flag_bits & ((1 << (16 - self.max_flags)) - 1)) << self.max_flags)
            | (self.value & ((1 << self.max_flags) - 1))
    }
}

bitflags! {
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
    #[serde(transparent)]
    pub struct Max16Flags: u16 {
        const BIT_STATUS_00 = 0b0000000000000001;
        const BIT_STATUS_01 = 0b0000000000000010;
        const BIT_STATUS_02 = 0b0000000000000100;
        const BIT_STATUS_03 = 0b0000000000001000;
        const BIT_STATUS_04 = 0b0000000000010000;
        const BIT_STATUS_05 = 0b0000000000100000;
        const BIT_STATUS_06 = 0b0000000001000000;
        const BIT_STATUS_07 = 0b0000000010000000;
        const BIT_STATUS_08 = 0b0000000100000000;
        const BIT_STATUS_09 = 0b0000001000000000;
        const BIT_STATUS_10 = 0b0000010000000000;
        const BIT_STATUS_11 = 0b0000100000000000;
        const BIT_STATUS_12 = 0b0001000000000000;
        const BIT_STATUS_13 = 0b0010000000000000;
        const BIT_STATUS_14 = 0b0100000000000000;
        const BIT_STATUS_15 = 0b1000000000000000;
    }
}

pub const U16_FLAG_INDEXES: [Max16Flags; 16] = [
    Max16Flags::BIT_STATUS_00,
    Max16Flags::BIT_STATUS_01,
    Max16Flags::BIT_STATUS_02,
    Max16Flags::BIT_STATUS_03,
    Max16Flags::BIT_STATUS_04,
    Max16Flags::BIT_STATUS_05,
    Max16Flags::BIT_STATUS_06,
    Max16Flags::BIT_STATUS_07,
    Max16Flags::BIT_STATUS_08,
    Max16Flags::BIT_STATUS_09,
    Max16Flags::BIT_STATUS_10,
    Max16Flags::BIT_STATUS_11,
    Max16Flags::BIT_STATUS_12,
    Max16Flags::BIT_STATUS_13,
    Max16Flags::BIT_STATUS_14,
    Max16Flags::BIT_STATUS_15,
];
