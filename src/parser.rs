use clap::Parser;
use std::{net::IpAddr, path::PathBuf};

fn default_host() -> &'static str {
    if cfg!(debug_assertions) {
        "127.0.0.1"
    } else {
        "0.0.0.0"
    }
}

fn default_port() -> u16 {
    if cfg!(debug_assertions) { 8080 } else { 80 }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Arguments {
    /// IP address to bind the server to
    #[arg(short, long, default_value = default_host())]
    pub(crate) ip_address: IpAddr,
    /// Port number to bind the server to
    #[arg(short, long, default_value_t = default_port())]
    pub(crate) port: u16,
    /// Directory to serve files from
    #[arg(short, long, default_value = "./assets")]
    pub(crate) directory: PathBuf,
}
