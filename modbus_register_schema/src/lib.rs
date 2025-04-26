pub mod constraints;
pub use constraints::{
    BooleanConstraints, BytesConstraints, Endianness, EnumConstraints, NumericConstraints,
    StringConstraints,
};
pub mod description;
pub use description::RegisterDescription;
pub mod schema;
pub use schema::RegisterSchema;
pub mod value_type;
pub use value_type::RegisterValueType;

pub fn serialize_registers(input_bytes: &[u8], is_big_endian: bool, output_u16_vec: &mut Vec<u16>) {
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

pub fn deserialize_registers(input_u16_vec: &[u16], is_big_endian: bool) -> Vec<u8> {
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
