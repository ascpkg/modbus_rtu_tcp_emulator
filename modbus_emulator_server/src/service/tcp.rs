use std::future;

use modbus_register_schema::*;

use tokio_modbus::prelude::*;

use crate::service::data::ModbusServiceData;

pub struct ModbusEmulatorTcpService {
    pub data: ModbusServiceData,
}

impl ModbusEmulatorTcpService {
    pub fn new(schema: RegisterSchema) -> Self {
        Self {
            data: ModbusServiceData::new(schema),
        }
    }
}

impl tokio_modbus::server::Service for ModbusEmulatorTcpService {
    type Request = Request<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.data.dispatch(req)
    }
}
