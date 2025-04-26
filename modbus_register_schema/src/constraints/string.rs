use serde::{Deserialize, Serialize};

use super::endian::Endianness;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StringConstraints {
    pub val: Option<String>,
    pub default: Option<String>,
    pub endianness: Option<Endianness>,
}
