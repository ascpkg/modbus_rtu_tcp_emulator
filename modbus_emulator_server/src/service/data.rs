use std::collections::HashMap;
use std::future;
use std::sync::{Arc, Mutex};

use modbus_register_schema::*;

use tokio_modbus::prelude::{ExceptionCode, Response, *};

use tracing;

use crate::op::{
    read::{register_read_bool, register_read_u16},
    write::{register_write_bool, register_write_u16},
};

pub struct ModbusServiceData {
    coils: Arc<Mutex<HashMap<u16, RegisterDescription>>>,
    discrete_inputs: Arc<Mutex<HashMap<u16, RegisterDescription>>>,
    input_registers: Arc<Mutex<HashMap<u16, RegisterDescription>>>,
    holding_registers: Arc<Mutex<HashMap<u16, RegisterDescription>>>,
}

impl ModbusServiceData {
    pub fn new(schema: RegisterSchema) -> Self {
        let mut coils = HashMap::new();
        for desc in schema.coils {
            coils.insert(desc.address, desc);
        }

        let mut discrete_inputs = HashMap::new();
        for desc in schema.discrete_inputs {
            discrete_inputs.insert(desc.address, desc);
        }

        let mut input_registers = HashMap::new();
        for desc in schema.input_registers {
            input_registers.insert(desc.address, desc);
        }

        let mut holding_registers = HashMap::new();
        for desc in schema.holding_registers {
            holding_registers.insert(desc.address, desc);
        }

        Self {
            coils: Arc::new(Mutex::new(coils)),
            discrete_inputs: Arc::new(Mutex::new(discrete_inputs)),
            input_registers: Arc::new(Mutex::new(input_registers)),
            holding_registers: Arc::new(Mutex::new(holding_registers)),
        }
    }

    pub fn dispatch(
        &self,
        request: Request<'static>,
    ) -> future::Ready<Result<Response, ExceptionCode>> {
        let result = match request {
            // read/write coils
            Request::ReadCoils(addr, quantity) => {
                register_read_bool(&self.coils.lock().unwrap(), addr, quantity)
                    .map(Response::ReadCoils)
            }
            Request::WriteSingleCoil(addr, value) => register_write_bool(
                &mut self.coils.lock().unwrap(),
                addr,
                std::slice::from_ref(&value),
            )
            .map(|_| Response::WriteSingleCoil(addr, value)),
            Request::WriteMultipleCoils(addr, values) => {
                register_write_bool(&mut self.coils.lock().unwrap(), addr, &values)
                    .map(|_| Response::WriteMultipleRegisters(addr, values.len() as u16))
            }
            // read discrete inputs
            Request::ReadDiscreteInputs(addr, cnt) => {
                register_read_bool(&self.discrete_inputs.lock().unwrap(), addr, cnt)
                    .map(Response::ReadDiscreteInputs)
            }
            // read input registers
            Request::ReadInputRegisters(addr, cnt) => {
                register_read_u16(&self.input_registers.lock().unwrap(), addr, cnt)
                    .map(Response::ReadInputRegisters)
            }
            // read/write holding registers
            Request::ReadHoldingRegisters(addr, cnt) => {
                register_read_u16(&self.holding_registers.lock().unwrap(), addr, cnt)
                    .map(Response::ReadHoldingRegisters)
            }
            Request::WriteSingleRegister(addr, value) => register_write_u16(
                &mut self.holding_registers.lock().unwrap(),
                addr,
                std::slice::from_ref(&value),
            )
            .map(|_| Response::WriteSingleRegister(addr, value)),
            Request::WriteMultipleRegisters(addr, values) => {
                register_write_u16(&mut self.holding_registers.lock().unwrap(), addr, &values)
                    .map(|_| Response::WriteMultipleRegisters(addr, values.len() as u16))
            }
            _ => {
                tracing::error!("SERVER: Exception::IllegalFunction - Unimplemented function code in request: {:?}", request);
                Err(ExceptionCode::IllegalFunction)
            }
        };
        future::ready(result)
    }
}
