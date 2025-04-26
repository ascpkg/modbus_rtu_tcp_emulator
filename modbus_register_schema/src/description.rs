use serde::{Deserialize, Serialize};

use super::value_type::RegisterValueType;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterDescription {
    pub name: String,
    pub address: u16,
    pub count: u16,
    pub value: RegisterValueType,
}
