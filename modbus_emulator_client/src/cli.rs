use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "modbus emulator client")]
pub struct Args {
    /// windows serial - COMX, linux serial - /dev/X, tcp - host:port
    #[arg(long, default_value = "127.0.0.1:5052")]
    pub addr: String,

    /// serial port baud rate
    #[arg(long, default_value_t = 0)]
    pub baud_rate: u32,

    /// register schema toml file
    #[arg(long, default_value = "schema.toml")]
    pub schema: String,
}
