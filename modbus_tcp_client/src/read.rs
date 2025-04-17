use modbus_register_schema::*;

use tokio_modbus::prelude::*;

use tracing;

pub async fn read_register(
    ctx: &mut tokio_modbus::client::Context,
    desc: &RegisterDescription,
    is_input_register: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match &desc.value {
        RegisterValueType::U8(constraints) => {
            let resp = if is_input_register {
                ctx.read_input_registers(desc.address, 1).await??
            } else {
                ctx.read_holding_registers(desc.address, 1).await??
            };
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = convert_u16_vec_to_bytes(&resp, is_big_endian);
            tracing::info!(
                "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                bytes[0],
                resp
            );
        }
        RegisterValueType::U16(constraints) => {
            let resp = if is_input_register {
                ctx.read_input_registers(desc.address, 1).await??
            } else {
                ctx.read_holding_registers(desc.address, 1).await??
            };
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = convert_u16_vec_to_bytes(&resp, is_big_endian);
            let v = if is_big_endian {
                u16::from_be_bytes([bytes[0], bytes[1]])
            } else {
                u16::from_le_bytes([bytes[0], bytes[1]])
            };
            tracing::info!(
                "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                resp
            );
        }
        RegisterValueType::U32(constraints) => {
            let resp = if is_input_register {
                ctx.read_input_registers(desc.address, 2).await??
            } else {
                ctx.read_holding_registers(desc.address, 2).await??
            };
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = convert_u16_vec_to_bytes(&resp, is_big_endian);
            let v = if is_big_endian {
                u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
            } else {
                u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
            };
            tracing::info!(
                "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                resp
            );
        }
        RegisterValueType::U64(constraints) => {
            let resp = if is_input_register {
                ctx.read_input_registers(desc.address, 4).await??
            } else {
                ctx.read_holding_registers(desc.address, 4).await??
            };
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = convert_u16_vec_to_bytes(&resp, is_big_endian);
            let v = if is_big_endian {
                u64::from_be_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ])
            } else {
                u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ])
            };
            tracing::info!(
                "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                v,
                resp
            );
        }
        RegisterValueType::Bytes(constraints) => {
            let resp = if is_input_register {
                ctx.read_input_registers(desc.address, desc.count * 2)
                    .await??
            } else {
                ctx.read_holding_registers(desc.address, desc.count * 2)
                    .await??
            };
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = convert_u16_vec_to_bytes(&resp, is_big_endian);
            tracing::info!(
                "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {:?} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                bytes,
                resp
            );
        }
        RegisterValueType::String(constraints) => {
            let resp = if is_input_register {
                ctx.read_input_registers(desc.address, desc.count * 2)
                    .await??
            } else {
                ctx.read_holding_registers(desc.address, desc.count * 2)
                    .await??
            };
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = convert_u16_vec_to_bytes(&resp, is_big_endian);
            tracing::info!(
                "read(name: {}, addr: {}, count: {}, endianness: {:?}) -> {} (raw: {:?})",
                desc.name,
                desc.address,
                desc.count,
                constraints.endianness.as_ref().unwrap_or(&Endianness::Big),
                String::from_utf8(bytes).unwrap(),
                resp
            );
        }

        RegisterValueType::Enum(constraints) => {
            let resp = if is_input_register {
                ctx.read_input_registers(desc.address, 2).await??
            } else {
                ctx.read_holding_registers(desc.address, 2).await??
            };
            let is_big_endian = constraints.endianness == Some(Endianness::Big);
            let bytes = convert_u16_vec_to_bytes(&resp, is_big_endian);
            let v = if is_big_endian {
                u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
            } else {
                u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
            };
            let name = constraints
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
                resp
            );
        }
    }

    Ok(())
}
