use modbus_register_schema::{
    types::{u16_flags::*, u32_flags::*, u64_flags::*},
    *,
};

use tokio_modbus::prelude::*;

use tracing;

pub async fn write_register(
    ctx: &mut tokio_modbus::client::Context,
    desc: &RegisterDescription,
    params: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    match &desc.value {
        RegisterValueType::Coils(_constraints) => {
            let values = params[3..]
                .iter()
                .map(|s| s.parse::<u8>().unwrap() != 0)
                .collect::<Vec<bool>>();
            tracing::info!(
                "write(name: {}, addr: {}, count: {}) -> {:?}",
                desc.name,
                desc.address,
                desc.count,
                values
            );
            let _ = ctx.write_multiple_coils(desc.address, &values).await?;
        }
        RegisterValueType::Discrete(_constraints) => {}
        RegisterValueType::U8(constraints) => {
            let v = params[3].parse::<u16>()?;
            if v > u8::MAX.into() || !validate(v as u8, &constraints) {
                return Err("v > u8::MAX.into() || validate(v as u8, &constraints)".into());
            }
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                [v]
            );
            let _ = ctx.write_single_register(desc.address, v).await?;
        }
        RegisterValueType::U16(constraints) => {
            let v = params[3].parse::<u16>()?;
            if !validate(v, &constraints) {
                return Err("validate(v, &constraints)".into());
            }
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                [v]
            );
            let _ = ctx.write_single_register(desc.address, v).await?;
        }
        RegisterValueType::U32(constraints) => {
            let v = params[3].parse::<u32>()?;
            if !validate(v, &constraints) {
                return Err("validate(v, &constraints)".into());
            }
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = if is_big_endian {
                &v.to_be_bytes()
            } else {
                &v.to_le_bytes()
            };
            let mut w = vec![
                0u16;
                if bytes.len() % 2 != 0 {
                    (bytes.len() + 1) / 2
                } else {
                    bytes.len() / 2
                }
            ];
            serialize_registers(bytes, is_big_endian, &mut w);
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                w
            );
            let _ = ctx.write_multiple_registers(desc.address, &w).await?;
        }
        RegisterValueType::U64(constraints) => {
            let v = params[3].parse::<u64>()?;
            if !validate(v, &constraints) {
                return Err("validate(v, &constraints)".into());
            }
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = if is_big_endian {
                &v.to_be_bytes()
            } else {
                &v.to_le_bytes()
            };
            let mut w = vec![
                0u16;
                if bytes.len() % 2 != 0 {
                    (bytes.len() + 1) / 2
                } else {
                    bytes.len() / 2
                }
            ];
            serialize_registers(bytes, is_big_endian, &mut w);
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                w
            );
            let _ = ctx.write_multiple_registers(desc.address, &w).await?;
        }
        RegisterValueType::U16Flags(constraints) => {
            let v = params[3].parse::<u16>()?;
            let vf = U16ValueFlags::from_u16(v, constraints.flag_names.len() as u8);
            let cf = NumericConstraints {
                val: Some(v),
                default: constraints.default,
                lt: constraints.lt,
                lte: constraints.lte,
                gt: constraints.gt,
                gte: constraints.gte,
                endianness: constraints.endianness.clone(),
            };
            if !validate(vf.value, &cf) {
                return Err("validate(v, &constraints)".into());
            }
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                [v]
            );
            let _ = ctx.write_single_register(desc.address, v).await?;
        }
        RegisterValueType::U32Flags(constraints) => {
            let v = params[3].parse::<u32>()?;
            let vf = U32ValueFlags::from_u32(v, constraints.flag_names.len() as u8);
            let cf = NumericConstraints {
                val: Some(v),
                default: constraints.default,
                lt: constraints.lt,
                lte: constraints.lte,
                gt: constraints.gt,
                gte: constraints.gte,
                endianness: constraints.endianness.clone(),
            };
            if !validate(vf.value, &cf) {
                return Err("validate(v, &constraints)".into());
            }
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = if is_big_endian {
                &v.to_be_bytes()
            } else {
                &v.to_le_bytes()
            };
            let mut w = vec![
                0u16;
                if bytes.len() % 2 != 0 {
                    (bytes.len() + 1) / 2
                } else {
                    bytes.len() / 2
                }
            ];
            serialize_registers(bytes, is_big_endian, &mut w);
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                w
            );
            let _ = ctx.write_multiple_registers(desc.address, &w).await?;
        }
        RegisterValueType::U64Flags(constraints) => {
            let v = params[3].parse::<u64>()?;
            let vf = U64ValueFlags::from_u64(v, constraints.flag_names.len() as u8);
            let cf = NumericConstraints {
                val: Some(v),
                default: constraints.default,
                lt: constraints.lt,
                lte: constraints.lte,
                gt: constraints.gt,
                gte: constraints.gte,
                endianness: constraints.endianness.clone(),
            };
            if !validate(vf.value, &cf) {
                return Err("validate(v, &constraints)".into());
            }
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = if is_big_endian {
                &v.to_be_bytes()
            } else {
                &v.to_le_bytes()
            };
            let mut w = vec![
                0u16;
                if bytes.len() % 2 != 0 {
                    (bytes.len() + 1) / 2
                } else {
                    bytes.len() / 2
                }
            ];
            serialize_registers(bytes, is_big_endian, &mut w);
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                w
            );
            let _ = ctx.write_multiple_registers(desc.address, &w).await?;
        }
        RegisterValueType::Bytes(constraints) => {
            let values = params[3..]
                .iter()
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            if values.len() > (desc.count * 2) as usize {
                return Err(format!("len: {} > max_size: {}", values.len(), desc.count * 2).into());
            }
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let mut w = vec![
                0u16;
                if values.len() % 2 != 0 {
                    (values.len() + 1) / 2
                } else {
                    values.len() / 2
                }
            ];
            serialize_registers(values.as_slice(), is_big_endian, &mut w);
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {:?} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                values,
                w
            );
            let _ = ctx.write_multiple_registers(desc.address, &w).await?;
        }
        RegisterValueType::String(constraints) => {
            let values = params[3..].join("");
            if values.as_bytes().len() > (desc.count * 2) as usize {
                return Err(format!(
                    "len: {} > max_size: {}",
                    values.as_bytes().len(),
                    desc.count * 2
                )
                .into());
            }
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let mut w = vec![
                0u16;
                if values.len() % 2 != 0 {
                    (values.len() + 1) / 2
                } else {
                    values.len() / 2
                }
            ];
            serialize_registers(values.as_bytes(), is_big_endian, &mut w);
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {:?} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                values,
                w
            );
            let _ = ctx.write_multiple_registers(desc.address, &w).await?;
        }
        RegisterValueType::Enum(constraints) => {
            let mut v = 0;
            println!("{:?}", constraints.kv);
            if let Some(i) = constraints.kv.get(params[3]) {
                v = i.clone();
            } else {
                _ = v;
                return Err(format!("{} not in {:?}", params[3], constraints.kv).into());
            }
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = if is_big_endian {
                &v.to_be_bytes()
            } else {
                &v.to_le_bytes()
            };
            let mut w = vec![
                0u16;
                if bytes.len() % 2 != 0 {
                    (bytes.len() + 1) / 2
                } else {
                    bytes.len() / 2
                }
            ];
            serialize_registers(bytes, is_big_endian, &mut w);
            tracing::info!(
                "write(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} = {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                params[3],
                w
            );
            let _ = ctx.write_multiple_registers(desc.address, &w).await?;
        }
    }

    Ok(())
}

fn validate<T: std::fmt::Display + std::fmt::Debug + PartialOrd>(
    value: T,
    constraints: &NumericConstraints<T>,
) -> bool {
    if let Some(lt) = &constraints.lt {
        if value >= *lt {
            tracing::warn!(
                "invalid value, v: {}, constraints: {:?}",
                value,
                constraints
            );
            return false;
        }
    }
    if let Some(lte) = &constraints.lte {
        if value > *lte {
            tracing::warn!(
                "invalid value, v: {}, constraints: {:?}",
                value,
                constraints
            );

            return false;
        }
    }
    if let Some(gt) = &constraints.gt {
        if value <= *gt {
            tracing::warn!(
                "invalid value, v: {}, constraints: {:?}",
                value,
                constraints
            );
            return false;
        }
    }
    if let Some(gte) = &constraints.gte {
        if value < *gte {
            tracing::warn!(
                "invalid value, v: {}, constraints: {:?}",
                value,
                constraints
            );

            return false;
        }
    }
    return true;
}
