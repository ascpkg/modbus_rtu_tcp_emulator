use serde::{Deserialize, Serialize};

use super::endian::Endianness;

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
pub struct NumericFlagsConstraints<T, N> {
    pub val: Option<T>,
    pub default: Option<N>,
    pub lt: Option<N>,
    pub lte: Option<N>,
    pub gt: Option<N>,
    pub gte: Option<N>,
    pub endianness: Option<Endianness>,
    pub flag_names: Vec<String>,
}
