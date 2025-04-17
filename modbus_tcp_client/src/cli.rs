use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "modbus_tcp_client")]
pub struct Args {
    #[arg(long, default_value = "127.0.0.1:5052")]
    pub addr: String,

    #[arg(long, default_value = "schema.toml")]
    pub schema: String,
}
