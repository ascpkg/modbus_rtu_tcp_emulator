use std::{
    collections::HashMap,
    future,
    sync::{Arc, Mutex},
};

use modbus_register_schema::*;

use tokio_modbus::prelude::*;

use tracing;

use crate::read::register_read;
use crate::write::register_write;

pub struct ModbusTcpEmulatorService {
    input_registers: Arc<Mutex<HashMap<u16, RegisterDescription>>>,
    holding_registers: Arc<Mutex<HashMap<u16, RegisterDescription>>>,
}

impl ModbusTcpEmulatorService {
    pub fn new(schema: RegisterSchema) -> Self {
        let mut input_registers = HashMap::new();
        for desc in schema.input_registers {
            input_registers.insert(desc.address, desc);
        }

        let mut holding_registers = HashMap::new();
        for desc in schema.holding_registers {
            holding_registers.insert(desc.address, desc);
        }

        Self {
            input_registers: Arc::new(Mutex::new(input_registers)),
            holding_registers: Arc::new(Mutex::new(holding_registers)),
        }
    }
}

impl tokio_modbus::server::Service for ModbusTcpEmulatorService {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let result = match req {
            Request::ReadInputRegisters(addr, cnt) => {
                register_read(&self.input_registers.lock().unwrap(), addr, cnt)
                    .map(Response::ReadInputRegisters)
            }
            Request::ReadHoldingRegisters(addr, cnt) => {
                register_read(&self.holding_registers.lock().unwrap(), addr, cnt)
                    .map(Response::ReadHoldingRegisters)
            }
            Request::WriteMultipleRegisters(addr, values) => {
                register_write(&mut self.holding_registers.lock().unwrap(), addr, &values)
                    .map(|_| Response::WriteMultipleRegisters(addr, values.len() as u16))
            }
            Request::WriteSingleRegister(addr, value) => register_write(
                &mut self.holding_registers.lock().unwrap(),
                addr,
                std::slice::from_ref(&value),
            )
            .map(|_| Response::WriteSingleRegister(addr, value)),
            _ => {
                tracing::error!("SERVER: Exception::IllegalFunction - Unimplemented function code in request: {:?}", req);
                Err(ExceptionCode::IllegalFunction)
            }
        };
        future::ready(result)
    }
}
