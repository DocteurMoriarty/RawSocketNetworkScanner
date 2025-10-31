#![cfg(feature = "std")]

use clap::Parser;
use crate::prelude::*;
use crate::parsing::my_parser::{parse_mac, parse_hex};

/// Arguments de la ligne de commande CLI
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'i', help = "format: --src_ip=192.168.25.2", long = "src_ip")]
    pub src_ip: Option<StringNoStd>,

    #[arg(short = 'd', help = "format: --dst_ip=192.168.1.25", long = "dst_ip")]
    pub dst_ip: Option<StringNoStd>,

    #[arg(short = 'p', help = "format: --dest_port=8080", long = "dest_port")]
    pub dest_port: Option<u16>,

    #[arg(short = 's', help = "format: --src_mac=aa:bb:cc:dd:ee:ff", long = "src_mac", value_parser = parse_mac)]
    pub src_mac: Option<[u8; 6]>,

    #[arg(short = 'm', help = "format: --dst_mac=11:22:33:44:55:66", long = "dst_mac", value_parser = parse_mac)]
    pub dst_mac: Option<[u8; 6]>,

    #[arg(short = 'l', help = "format: --l4_protocol=udp", long = "l4_protocol")]
    pub l4_protocol: Option<StringNoStd>,

    #[arg(short = 't', long = "timeout_ms", help = "format: --timeout_ms=2000")]
    pub timeout_ms: Option<u64>,

    #[arg(short = 'f', long = "debug_file", help = "format: --debug_file=./debug.pcap")]
    pub debug_file: Option<StringNoStd>,

    #[arg(short = 'g', long = "debug_format", help = "format: --debug_format=json")]
    pub debug_format: Option<StringNoStd>,

    #[arg(short = 'b', long = "ip_bitfield", default_value = "0x00", value_parser = parse_hex)]
    pub ip_bitfield: Option<u8>,

    #[arg(short = 'r', long = "dry_run", help = "format: --dry_run", action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,

    #[arg(long, default_value = "1", hide = true)]
    pub count: u32,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}