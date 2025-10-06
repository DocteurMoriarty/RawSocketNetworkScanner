pub mod cli;
pub mod structs;
pub mod parsing;
pub mod errors;

use cli::args::{parse_args, Args};

fn main() {
    let args: Args = parse_args();

    println!(
        "SRC IP: {}",
        args.src_ip().as_deref().unwrap_or("<missing>")
    );
    println!(
        "DST IP: {}",
        args.dst_ip().as_deref().unwrap_or("<missing>")
    );

    if let Some(mac) = args.src_mac() {
        println!("SRC MAC: {:02X?}", mac);
    } else {
        println!("SRC MAC: <missing>");
    }

    if let Some(ip_bit) = args.ip_bitfield() {
        println!("IP bitfield: 0x{:02X}", ip_bit);
    }

    for i in 0..*args.count() {
        println!(
            "Packet {}: {} -> {}:{}, IP bitfield=0x{:02X}, SRC MAC={:02X?}",
            i + 1,
            args.src_ip().as_deref().unwrap_or("<missing>"),
            args.dst_ip().as_deref().unwrap_or("<missing>"),
            args.dest_port().unwrap_or(0),
            args.ip_bitfield().unwrap_or(0),
            args.src_mac().unwrap_or([0; 6])
        );
    }
}
