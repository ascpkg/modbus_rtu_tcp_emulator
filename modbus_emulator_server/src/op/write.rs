use std::collections::HashMap;

use modbus_register_schema::{
    types::{u16_flags::U16ValueFlags, u32_flags::U32ValueFlags, u64_flags::U64ValueFlags},
    *,
};

use tokio_modbus::prelude::*;

use tracing;

/// Write a holding register. Used by both the write single register
/// and write multiple registers requests.

pub fn register_write_bool(
    registers: &mut HashMap<u16, RegisterDescription>,
    addr: u16,
    values: &[bool],
) -> Result<(), ExceptionCode> {
    if let Some(desc) = registers.get_mut(&addr) {
        match &mut desc.value {
            RegisterValueType::Coils(constraints) => {
                constraints.set_bits(0, values);
                tracing::info!(
                    "write(name: {}, addr: {}, count: {}) -> {:?} (raw: {:?})",
                    desc.name,
                    addr,
                    desc.count,
                    values,
                    constraints.val
                );
            }
            RegisterValueType::Discrete(_constraints) => {}
            RegisterValueType::U8(_constraints) => {}
            RegisterValueType::U16(_constraints) => {}
            RegisterValueType::U32(_constraints) => {}
            RegisterValueType::U64(_constraints) => {}
            RegisterValueType::U16Flags(_constraints) => {}
            RegisterValueType::U32Flags(_constraints) => {}
            RegisterValueType::U64Flags(_constraints) => {}
            RegisterValueType::Bytes(_constraints) => {}
            RegisterValueType::String(_constraints) => {}
            RegisterValueType::Enum(_constraints) => {}
        }
    }

    Ok(())
}

pub fn register_write_u16(
    registers: &mut HashMap<u16, RegisterDescription>,
    addr: u16,
    values: &[u16],
) -> Result<(), ExceptionCode> {
    if let Some(desc) = registers.get_mut(&addr) {
        match &mut desc.value {
            RegisterValueType::Coils(_constraints) => {}
            RegisterValueType::Discrete(_constraints) => {}
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
                let bytes = deserialize_registers(values, is_big_endian);
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
                let bytes = deserialize_registers(values, is_big_endian);
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
            RegisterValueType::U16Flags(constraints) => {
                let vf = U16ValueFlags::from_u16(values[0], constraints.flag_names.len() as u8);
                constraints.val = Some(vf);
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
            RegisterValueType::U32Flags(constraints) => {
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                let bytes = deserialize_registers(values, is_big_endian);
                let v = if is_big_endian {
                    u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                } else {
                    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                };
                let vf = U32ValueFlags::from_u32(v, constraints.flag_names.len() as u8);
                constraints.val = Some(vf);
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
            RegisterValueType::U64Flags(constraints) => {
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                let bytes = deserialize_registers(values, is_big_endian);
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
                let vf = U64ValueFlags::from_u64(v, constraints.flag_names.len() as u8);
                constraints.val = Some(vf);
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
                let bytes: Vec<u8> = deserialize_registers(values, is_big_endian);
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
                let bytes = deserialize_registers(values, is_big_endian);
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
                let bytes = deserialize_registers(values, is_big_endian);
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
