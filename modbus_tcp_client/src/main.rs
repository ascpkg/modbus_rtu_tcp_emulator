use std::net::SocketAddr;

use clap::Parser;

use modbus_register_schema::*;

use tokio_modbus::prelude::*;

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

    let args = cli::Args::parse();
    tracing::info!("{:?}", args);
    let socket_addr: SocketAddr = args.addr.parse().unwrap();
    let schema = RegisterSchema::load(&args.schema, false).unwrap();

    let mut ctx = tcp::connect(socket_addr).await?;

    let help_text = format!(
        r#"Commands
quit          : Exit the program
help          : Show this help message
query         : Query input/holding register schema
         Usage: query <type> <index>
                <type> can be 'input' or 'holding'
                <index> can be 'all' or a specific index (input: 0-{}, holding: 0-{})

read          : Read input/holding register
         Usage: read <type> <index>
                <type> can be 'input' or 'holding'
                <index> can be 'all' or a specific index (0-{})

write         : Write to holding register
         Usage: write <index> <value>
                <index> can be a specific index (0-{})
                <value> is the value to write"#,
        schema.input_registers.len(),
        schema.holding_registers.len(),
        schema.input_registers.len(),
        schema.holding_registers.len()
    );
    tracing::info!("{}", help_text);

    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let params = input.split_whitespace().collect::<Vec<&str>>();
        if params.len() > 0 {
            let action = params[0];
            if action == "quit" {
                break;
            }
            if action == "help" {
                tracing::info!("{}", help_text);
            } else if action == "query" {
                if params.len() < 3 {
                    tracing::warn!("args missing, query <type> <index>");
                    continue;
                }
                let is_input_register = params[1] == "input";
                if params[2] != "all" {
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
            } else if action == "read" {
                if params.len() < 3 {
                    tracing::warn!("args missing, read <type> <index>");
                    continue;
                }
                let is_input_register = action == "ri";
                if params[2] != "all" {
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
            } else if action == "write" {
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
