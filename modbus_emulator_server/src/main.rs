use std::net::SocketAddr;

use clap::Parser;

use modbus_register_schema::*;

use tokio::net::TcpListener;

use tokio_modbus::server::{rtu, tcp};

use time::{macros::format_description, UtcOffset};

use tracing;
use tracing_subscriber::{self, fmt::time::OffsetTime};

pub mod cli;
pub mod op;
pub mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init stdout tracing log
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_line_number(true)
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"),
        ))
        .init();

    // parse command line args
    let args = cli::Args::parse();
    tracing::info!("{:?}", args);
    let schema = RegisterSchema::load(&args.schema, false).unwrap();

    if (args.addr.starts_with("COM") || args.addr.starts_with("/dev/")) && args.baud_rate > 0 {
        // run rtu server
        let serial_builder = tokio_serial::new(args.addr, args.baud_rate);
        let serial_server = tokio_serial::SerialStream::open(&serial_builder).unwrap();

        let rtu_master = rtu::Server::new(serial_server);
        let service = service::rtu::ModbusEmulatorRtuService::new(schema.clone());
        rtu_master.serve_forever(service).await?;
    } else {
        // run tcp server
        let socket_addr: SocketAddr = args.addr.parse().unwrap();
        let tcp_listener = TcpListener::bind(socket_addr).await?;
        let tcp_server = tcp::Server::new(tcp_listener);
        let service = |_socket_addr| {
            Ok(Some(service::tcp::ModbusEmulatorTcpService::new(
                schema.clone(),
            )))
        };
        let on_connected = |stream, socket_addr| async move {
            tcp::accept_tcp_connection(stream, socket_addr, service)
        };
        let on_process_error = |err| {
            tracing::error!("{err}");
        };
        tcp_server.serve(&on_connected, on_process_error).await?;
    }

    Ok(())
}
