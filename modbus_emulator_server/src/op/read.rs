use std::collections::HashMap;

use modbus_register_schema::*;

use tokio_modbus::prelude::*;

use tracing;

pub fn register_read_bool(
    registers: &HashMap<u16, RegisterDescription>,
    addr: u16,
    cnt: u16,
) -> Result<Vec<bool>, ExceptionCode> {
    let mut response: Vec<bool> = Vec::new();
    if let Some(desc) = registers.get(&addr) {
        match &desc.value {
            RegisterValueType::Coils(constraints) => {
                response = constraints.get_bits(0, cnt as usize);
                tracing::info!(
                    "read(name: {}, addr: {}, count: {}) -> {:?} (raw: {:?})",
                    desc.name,
                    desc.address,
                    desc.count,
                    response,
                    constraints.val,
                );
            }
            RegisterValueType::Discrete(constraints) => {
                response = constraints.get_bits(0, cnt as usize);
                tracing::info!(
                    "read(name: {}, addr: {}, count: {}) -> {:?} (raw: {:?})",
                    desc.name,
                    desc.address,
                    desc.count,
                    response,
                    constraints.val,
                );
            }
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
    Ok(response)
}

pub fn register_read_u16(
    registers: &HashMap<u16, RegisterDescription>,
    addr: u16,
    cnt: u16,
) -> Result<Vec<u16>, ExceptionCode> {
    let mut response: Vec<u16> = vec![0; cnt.into()];
    if let Some(desc) = registers.get(&addr) {
        match &desc.value {
            RegisterValueType::Coils(_constraints) => {}
            RegisterValueType::Discrete(_constraints) => {}
            RegisterValueType::U8(constraints) => {
                if let Some(v) = constraints.val.or(constraints.default) {
                    response[0] = v as u16;
                    tracing::info!(
                        "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                        desc.name,
                        desc.address,
                        desc.count,
                        constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                        v,
                        response
                    );
                }
            }
            RegisterValueType::U16(constraints) => {
                if let Some(v) = constraints.val.or(constraints.default) {
                    response[0] = v;
                    tracing::info!(
                        "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                        desc.name,
                        desc.address,
                        desc.count,
                        constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                        v,
                        response
                    );
                }
            }
            RegisterValueType::U32(constraints) => {
                let val = constraints.val.or(constraints.default).unwrap_or(0);
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                let bytes = if is_big_endian {
                    &val.to_be_bytes()
                } else {
                    &val.to_le_bytes()
                };
                serialize_registers(bytes, is_big_endian, &mut response);
                tracing::info!(
                    "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                    desc.name,
                    desc.address,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    val,
                    response
                );
            }
            RegisterValueType::U64(constraints) => {
                let val = constraints.val.or(constraints.default).unwrap_or(0);
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                let bytes = if is_big_endian {
                    &val.to_be_bytes()
                } else {
                    &val.to_le_bytes()
                };
                serialize_registers(bytes, is_big_endian, &mut response);
                tracing::info!(
                    "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                    desc.name,
                    desc.address,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    val,
                    response
                );
            }
            RegisterValueType::U16Flags(constraints) => {
                let vo: Option<u16> = if let Some(vf) = constraints.val.clone() {
                    Some(vf.to_u16())
                } else if let Some(v) = constraints.default {
                    Some(v)
                } else {
                    None
                };

                if let Some(v) = vo {
                    response[0] = v;
                    tracing::info!(
                        "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                        desc.name,
                        desc.address,
                        desc.count,
                        constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                        v,
                        response
                    );
                }
            }
            RegisterValueType::U32Flags(constraints) => {
                let vo: Option<u32> = if let Some(vf) = constraints.val.clone() {
                    Some(vf.to_u32())
                } else if let Some(v) = constraints.default {
                    Some(v)
                } else {
                    None
                };
                if let Some(v) = vo {
                    let is_big_endian = constraints.endianness == Some(Endianness::Big);
                    let bytes = if is_big_endian {
                        &v.to_be_bytes()
                    } else {
                        &v.to_le_bytes()
                    };
                    serialize_registers(bytes, is_big_endian, &mut response);
                    tracing::info!(
                        "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                        desc.name,
                        desc.address,
                        desc.count,
                        constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                        v,
                        response
                    );
                }
            }
            RegisterValueType::U64Flags(constraints) => {
                let vo: Option<u64> = if let Some(vf) = constraints.val.clone() {
                    Some(vf.to_u64())
                } else if let Some(v) = constraints.default {
                    Some(v)
                } else {
                    None
                };
                if let Some(v) = vo {
                    let is_big_endian = constraints.endianness == Some(Endianness::Big);
                    let bytes = if is_big_endian {
                        &v.to_be_bytes()
                    } else {
                        &v.to_le_bytes()
                    };
                    serialize_registers(bytes, is_big_endian, &mut response);
                    tracing::info!(
                        "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                        desc.name,
                        desc.address,
                        desc.count,
                        constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                        v,
                        response
                    );
                }
            }
            RegisterValueType::Bytes(constraints) => {
                let val = constraints
                    .val
                    .clone()
                    .or(constraints.default.clone())
                    .unwrap_or_else(Vec::new);
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                serialize_registers(val.as_slice(), is_big_endian, &mut response);
                tracing::info!(
                    "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {:?} (raw: {:?})",
                    desc.name,
                    desc.address,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    val,
                    response
                );
            }
            RegisterValueType::String(constraints) => {
                let val = constraints
                    .val
                    .clone()
                    .or(constraints.default.clone())
                    .unwrap_or_else(String::new);
                let is_big_endian = constraints.endianness == Some(Endianness::Big);
                serialize_registers(val.as_bytes(), is_big_endian, &mut response);
                tracing::info!(
                    "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {:?} (raw: {:?})",
                    desc.name,
                    desc.address,
                    desc.count,
                    constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                    val,
                    response
                );
            }
            RegisterValueType::Enum(constraints) => {
                let mut set = false;
                let mut v = 0;
                if let Some(i) = constraints.val {
                    set = true;
                    v = i;
                }
                if !set {
                    if let Some(name) = &constraints.default {
                        if let Some(i) = constraints.kv.get(name) {
                            set = true;
                            v = i.clone();
                        }
                    }
                }
                if !set {
                    tracing::warn!("unset, schema: {:?}", desc)
                } else {
                    let is_big_endian = constraints.endianness == Some(Endianness::Big);
                    let bytes = if is_big_endian {
                        &v.to_be_bytes()
                    } else {
                        &v.to_le_bytes()
                    };
                    serialize_registers(bytes, is_big_endian, &mut response);
                    let name =
                        constraints
                            .kv
                            .iter()
                            .find_map(|(name, index)| if &v == index { Some(name) } else { None });
                    tracing::info!(
                        "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} = {:?} (raw: {:?})",
                        desc.name,
                        desc.address,
                        desc.count,
                        constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                        v,
                        name,
                        response
                    );
                }
            }
        }
    } else {
        tracing::error!("SERVER: ExceptionCode::IllegalDataAddress({})", addr);
        return Err(ExceptionCode::IllegalDataAddress);
    }

    Ok(response)
}
