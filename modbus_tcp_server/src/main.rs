use std::net::SocketAddr;

use clap::Parser;

use modbus_register_schema::*;

use tokio::net::TcpListener;

use tokio_modbus::server::tcp::{accept_tcp_connection, Server};

use time::{macros::format_description, UtcOffset};

use tracing;
use tracing_subscriber::{self, fmt::time::OffsetTime};

pub mod cli;
pub mod read;
pub mod service;
pub mod write;

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

    let args = cli::Args::parse();
    tracing::info!("{:?}", args);
    let socket_addr: SocketAddr = args.addr.parse().unwrap();
    let schema = RegisterSchema::load(&args.schema, false).unwrap();

    let listener = TcpListener::bind(socket_addr).await?;
    let server = Server::new(listener);
    let new_service =
        |_socket_addr| Ok(Some(service::ModbusTcpEmulatorService::new(schema.clone())));
    let on_connected = |stream, socket_addr| async move {
        accept_tcp_connection(stream, socket_addr, new_service)
    };
    let on_process_error = |err| {
        tracing::error!("{err}");
    };
    server.serve(&on_connected, on_process_error).await?;
    Ok(())
}
