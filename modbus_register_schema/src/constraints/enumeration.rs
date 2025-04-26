use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::endian::Endianness;

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
