use std::collections::HashMap;

use config_file_derives::ConfigFile;
use config_file_types;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("toml")]
pub struct RegisterSchema {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_registers: Vec<RegisterDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holding_registers: Vec<RegisterDescription>,

    #[serde(skip)]
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterDescription {
    pub name: String,
    pub address: u16,
    pub count: u16,
    pub value: RegisterValueType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum RegisterValueType {
    U8(NumericConstraints<u8>),
    U16(NumericConstraints<u16>),
    U32(NumericConstraints<u32>),
    U64(NumericConstraints<u64>),
    Bytes(BytesConstraints),
    String(StringConstraints),
    Enum(EnumConstraints<String>),
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NumericConstraints<T> {
    pub val: Option<T>,
    pub default: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub endianness: Option<Endianness>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BytesConstraints {
    pub val: Option<Vec<u8>>,
    pub default: Option<Vec<u8>>,
    pub endianness: Option<Endianness>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StringConstraints {
    pub val: Option<String>,
    pub default: Option<String>,
    pub endianness: Option<Endianness>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct EnumConstraints<T>
where
    T: Eq + std::hash::Hash,
{
    pub val: Option<u32>,
    #[serde(default)]
    pub kv: HashMap<T, u32>,
    pub default: Option<T>,
    pub endianness: Option<Endianness>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Endianness {
    Little,
    Big,
}

pub fn bytes_to_u16_vec(input_bytes: &[u8], is_big_endian: bool, output_u16_vec: &mut Vec<u16>) {
    for i in 0..(input_bytes.len() / 2) {
        let j = i * 2;
        if j + 1 < input_bytes.len() {
            output_u16_vec[i] = if is_big_endian {
                u16::from_be_bytes([input_bytes[j], input_bytes[j + 1]])
            } else {
                u16::from_le_bytes([input_bytes[j], input_bytes[j + 1]])
            };
        } else {
            output_u16_vec[i] = input_bytes[j] as u16;
        }
    }
}

pub fn u16_vec_to_bytes(input_u16_vec: &[u16], is_big_endian: bool) -> Vec<u8> {
    input_u16_vec.iter().fold(vec![], |mut x, elem| {
        if is_big_endian {
            x.push((elem >> 8) as u8); // high byte
            x.push((elem & 0xff) as u8); // low byte
        } else {
            x.push((elem & 0xff) as u8); // low byte
            x.push((elem >> 8) as u8); // high byte
        }
        x
    })
}
