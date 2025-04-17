use std::collections::HashMap;

use modbus_register_schema::*;

use tokio_modbus::prelude::*;

use tracing;

/// Write a holding register. Used by both the write single register
/// and write multiple registers requests.
pub fn register_write(
    registers: &mut HashMap<u16, RegisterDescription>,
    addr: u16,
    values: &[u16],
) -> Result<(), ExceptionCode> {
    if let Some(desc) = registers.get_mut(&addr) {
        match &mut desc.value {
            RegisterValueType::U8(constraints) => {
                constraints.val = Some(values[0] as u8);
                tracing::info!(
                    "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                    desc.name,
                    addr,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    values[0],
                    values
                );
            }
            RegisterValueType::U16(constraints) => {
                constraints.val = Some(values[0]);
                tracing::info!(
                    "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                    desc.name,
                    addr,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    values[0],
                    values
                );
            }
            RegisterValueType::U32(constraints) => {
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                let bytes = convert_u16_vec_to_bytes(values, is_big_endian);
                let v = if is_big_endian {
                    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                } else {
                    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                };
                constraints.val = Some(v);
                tracing::info!(
                    "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                    desc.name,
                    addr,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    v,
                    values
                );
            }
            RegisterValueType::U64(constraints) => {
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                let bytes = convert_u16_vec_to_bytes(values, is_big_endian);
                let v = if is_big_endian {
                    u64::from_be_bytes([
                        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
                        bytes[7],
                    ])
                } else {
                    u64::from_le_bytes([
                        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
                        bytes[7],
                    ])
                };
                constraints.val = Some(v);
                tracing::info!(
                    "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                    desc.name,
                    addr,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    v,
                    values
                );
            }
            RegisterValueType::Bytes(constraints) => {
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                let bytes: Vec<u8> = convert_u16_vec_to_bytes(values, is_big_endian);
                tracing::info!(
                    "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {:?} (raw: {:?})",
                    desc.name,
                    addr,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    bytes,
                    values
                );
                constraints.val = Some(bytes);
            }
            RegisterValueType::String(constraints) => {
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                let bytes = convert_u16_vec_to_bytes(values, is_big_endian);
                let text = String::from_utf8(bytes).unwrap();
                tracing::info!(
                    "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                    desc.name,
                    addr,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    text,
                    values
                );
                constraints.val = Some(text);
            }
            RegisterValueType::Enum(constraints) => {
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                let bytes = convert_u16_vec_to_bytes(values, is_big_endian);
                let v = if is_big_endian {
                    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                } else {
                    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                };
                constraints.val = Some(v);
                let name =
                    constraints
                        .kv
                        .iter()
                        .find_map(|(name, index)| if &v == index { Some(name) } else { None });
                tracing::info!(
                    "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} = {:?} (raw: {:?})",
                    desc.name,
                    addr,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    v,
                    name,
                    values
                );
            }
        }
    } else {
        tracing::error!("SERVER: ExceptionCode::IllegalDataAddress({addr})");
        return Err(ExceptionCode::IllegalDataAddress);
    }

    Ok(())
}
