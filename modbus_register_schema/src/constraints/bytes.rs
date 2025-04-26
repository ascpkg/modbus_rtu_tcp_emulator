use serde::{Deserialize, Serialize};

use super::endian::Endianness;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BytesConstraints {
    pub val: Option<Vec<u8>>,
    pub default: Option<Vec<u8>>,
    pub endianness: Option<Endianness>,
}
