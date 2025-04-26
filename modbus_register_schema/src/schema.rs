use config_file_derives::ConfigFile;
use config_file_types;

use serde::{Deserialize, Serialize};

use super::description::RegisterDescription;

#[derive(Clone, Debug, Default, Deserialize, Serialize, ConfigFile)]
#[config_file_ext("toml")]
pub struct RegisterSchema {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coils: Vec<RegisterDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub discrete_inputs: Vec<RegisterDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub input_registers: Vec<RegisterDescription>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holding_registers: Vec<RegisterDescription>,

    #[serde(skip)]
    pub path: String,
}
