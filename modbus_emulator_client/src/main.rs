use std::{io::Write, net::SocketAddr};

use clap::Parser;

use modbus_register_schema::*;

use tokio_modbus::prelude::*;

use tokio_serial::SerialStream;

use time::{macros::format_description, UtcOffset};

use tracing;
use tracing_subscriber::{self, fmt::time::OffsetTime};

pub mod cli;
pub mod read;
pub mod write;

#[tokio::main(flavor = "current_thread")]
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

    let mut ctx =
        if (args.addr.starts_with("COM") || args.addr.starts_with("/dev/")) && args.baud_rate > 0 {
            // connect serial
            let serial_builder = tokio_serial::new(args.addr, args.baud_rate);
            let serial_stream = SerialStream::open(&serial_builder).unwrap();
            let serial_slave = Slave(0x17);
            rtu::attach_slave(serial_stream, serial_slave)
        } else {
            // connect tcp
            let socket_addr: SocketAddr = args.addr.parse().unwrap();
            tcp::connect(socket_addr).await?
        };

    // show help manual
    let help_text = format!(
        r#"
|---------------------------------------------------------------------------------------------------------|
| e | exit                  : Exit the program                                                            |
| h | help                  : Show this help message                                                      |
| q | query <type> <index>  : Query input/holding register schema                                         |
|                             <type> can be 'i' | 'input' or 'h' | 'holding'                              |
|                             <index> can be 'a' | 'all' or a specific index (input: 0-{}, holding: 0-{}) |
|                                                                                                         |
| r | read <type> <index>   : Read input/holding register                                                 |
|                             <type> can be 'i' | 'input' or 'h' | 'holding'                              |
|                             <index> can be 'a' | 'all' or a specific index (0-{})                       |
|                                                                                                         |
| w | write <index> <value> : Write to holding register                                                   |
|                             <index> can be a specific index (0-{})                                      |
|                             <value> is the value to write                                               |
|---------------------------------------------------------------------------------------------------------|"#,
        schema.input_registers.len(),
        schema.holding_registers.len(),
        schema.input_registers.len(),
        schema.holding_registers.len()
    );
    tracing::info!("{}", help_text);

    // user interaction
    let mut input = String::new();
    loop {
        input.clear();
        print!("\x1b[1m\x1b[32m> \x1b[0m");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input).unwrap();
        let params = input.split_whitespace().collect::<Vec<&str>>();
        if params.len() > 0 {
            let action = params[0];
            if action == "e" || action == "exit" {
                break;
            }
            if action == "h" || action == "help" {
                tracing::info!("{}", help_text);
            } else if action == "q" || action == "query" {
                if params.len() < 3 {
                    tracing::warn!("args missing, query <type> <index>");
                    continue;
                }
                let is_input_register = action == "i" || action == "input";
                if params[2] != "a" || params[2] != "all" {
                    let index = params[2].parse::<usize>()?;
                    let desc = if is_input_register {
                        if index >= schema.input_registers.len() {
                            tracing::warn!("input_registers index out of range");
                            continue;
                        }
                        &schema.input_registers[index]
                    } else {
                        if index >= schema.holding_registers.len() {
                            tracing::warn!("holding_registers index out of range");
                            continue;
                        }
                        &schema.holding_registers[index]
                    };

                    tracing::info!("{:?}", desc);
                } else {
                    let registers = if is_input_register {
                        &schema.input_registers
                    } else {
                        &schema.holding_registers
                    };
                    for desc in registers {
                        tracing::info!("{:?}", desc);
                    }
                }
            } else if action == "r" || action == "read" {
                if params.len() < 3 {
                    tracing::warn!("args missing, read <type> <index>");
                    continue;
                }
                let is_input_register = action == "i" || action == "input";
                if params[2] != "a" || params[2] != "all" {
                    let index = params[2].parse::<usize>()?;
                    let desc = if is_input_register {
                        if index >= schema.input_registers.len() {
                            tracing::warn!("input_registers index out of range");
                            continue;
                        }
                        &schema.input_registers[index]
                    } else {
                        if index >= schema.holding_registers.len() {
                            tracing::warn!("holding_registers index out of range");
                            continue;
                        }
                        &schema.holding_registers[index]
                    };

                    read::read_register(&mut ctx, desc, is_input_register).await?
                } else {
                    let registers = if is_input_register {
                        &schema.input_registers
                    } else {
                        &schema.holding_registers
                    };
                    for register in registers {
                        read::read_register(&mut ctx, register, is_input_register).await?
                    }
                }
            } else if action == "w" || action == "write" {
                if params.len() < 2 {
                    tracing::warn!("args missing, write <index>");
                    continue;
                }

                let index = params[1].parse::<usize>()?;
                let desc = &schema.holding_registers[index];
                write::write_register(&mut ctx, desc, params).await?
            }
        }
    }

    ctx.disconnect().await?;

    Ok(())
}
