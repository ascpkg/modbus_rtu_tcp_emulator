use std::future;

use modbus_register_schema::*;

use tokio_modbus::prelude::*;

use super::data::ModbusServiceData;

pub struct ModbusEmulatorRtuService {
    pub data: ModbusServiceData,
}

impl ModbusEmulatorRtuService {
    pub fn new(schema: RegisterSchema) -> Self {
        Self {
            data: ModbusServiceData::new(schema),
        }
    }
}

impl tokio_modbus::server::Service for ModbusEmulatorRtuService {
    type Request = SlaveRequest<'static>;
    type Response = Response;
    type Exception = ExceptionCode;
    type Future = future::Ready<Result<Self::Response, Self::Exception>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        self.data.dispatch(req.request)
    }
}
