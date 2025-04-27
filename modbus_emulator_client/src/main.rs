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
            let salve = Slave(args.slave);
            rtu::attach_slave(serial_stream, salve)
        } else {
            // connect tcp
            let socket_addr: SocketAddr = args.addr.parse().unwrap();
            tcp::connect(socket_addr).await?
        };

    // show help manual
    let help_text = format!(
        r#"
------------------------------------------------------------
e | exit                         : Exit the program
h | help                         : Show this help message
q | query <type> <index>         : Query register schema
r | read  <type> <index>         : Read register data
w | write <type> <index> <value> : Write data to register
                         <value> : the value to write
                          <type> : c | coils
                                   d | discrete
                                   i | input
                                   h | holding
                         <index> : a | all
                     coils index : 0 - {} (read + write)
            discrete input index : 0 - {} (read only)
            input register index : 0 - {} (read only)
          holding register index : 0 - {} (read + write)
------------------------------------------------------------"#,
        schema.coils.len(),
        schema.discrete_inputs.len(),
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

                let type_ = params[1];
                let index = params[2];
                if index != "a" && index != "all" {
                    let index = index.parse::<usize>()?;
                    let desc = if type_ == "c" || type_ == "coils" {
                        if index >= schema.coils.len() {
                            tracing::warn!("coils index out of range");
                            continue;
                        }
                        &schema.coils[index]
                    } else if type_ == "d" || type_ == "discrete" {
                        if index >= schema.discrete_inputs.len() {
                            tracing::warn!("discrete_inputs index out of range");
                            continue;
                        }
                        &schema.discrete_inputs[index]
                    } else if type_ == "i" || type_ == "input" {
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
                    let registers = if type_ == "c" || type_ == "coils" {
                        &schema.coils
                    } else if type_ == "d" || type_ == "discrete" {
                        &schema.discrete_inputs
                    } else if type_ == "i" || type_ == "input" {
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

                let type_ = params[1];
                let index = params[2];
                if index != "a" && index != "all" {
                    let index = index.parse::<usize>()?;
                    let desc = if type_ == "c" || type_ == "coils" {
                        if index >= schema.coils.len() {
                            tracing::warn!("coils index out of range");
                            continue;
                        }
                        &schema.coils[index]
                    } else if type_ == "d" || type_ == "discrete" {
                        if index >= schema.discrete_inputs.len() {
                            tracing::warn!("discrete_inputs index out of range");
                            continue;
                        }
                        &schema.discrete_inputs[index]
                    } else if type_ == "i" || type_ == "input" {
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

                    read::read_register(&mut ctx, desc, type_ == "i" || type_ == "input").await?
                } else {
                    let registers = if type_ == "c" || type_ == "c" {
                        &schema.coils
                    } else if type_ == "d" || type_ == "discrete" {
                        &schema.discrete_inputs
                    } else if type_ == "i" || type_ == "input" {
                        &schema.input_registers
                    } else {
                        &schema.holding_registers
                    };
                    for desc in registers {
                        read::read_register(&mut ctx, desc, type_ == "i" || type_ == "input")
                            .await?
                    }
                }
            } else if action == "w" || action == "write" {
                if params.len() < 3 {
                    tracing::warn!("args missing, write <type> <index> <value>");
                    continue;
                }

                let type_ = params[1];
                let index = params[2];
                let index = index.parse::<usize>()?;
                if type_ == "c" || type_ == "coils" {
                    let desc = &schema.coils[index];
                    write::write_register(&mut ctx, desc, params).await?
                } else if type_ == "h" || type_ == "holding" {
                    let desc = &schema.holding_registers[index];
                    write::write_register(&mut ctx, desc, params).await?
                }
            }
        }
    }

    ctx.disconnect().await?;

    Ok(())
}
