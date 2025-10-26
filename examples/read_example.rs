use projet_rsns_morissetlarresacha::cli::Args;

fn main() {
    let args: Args = Args::parse_args();

    // Show IP Addresses
    let src_ip = args.src_ip.as_deref().unwrap_or("<missing>");
    let dst_ip = args.dst_ip.as_deref().unwrap_or("<missing>");
    println!("SRC IP: {} -> DST IP: {}", src_ip, dst_ip);

    // Show MAC Addresses
    if let Some(src_mac) = args.src_mac {
        println!("SRC MAC: {:02X?}", src_mac);
    } else {
        println!("SRC MAC: <missing>");
    }

    if let Some(dst_mac) = args.dst_mac {
        println!("DST MAC: {:02X?}", dst_mac);
    } else {
        println!("DST MAC: <missing>");
    }

    // Show L4 Protocol
    if let Some(l4) = args.l4_protocol {
        println!("L4 Protocol: {}", l4);
    } else {
        println!("L4 Protocol: <missing>");
    }

    // Show Timeout
    if let Some(timeout) = args.timeout_ms {
        println!("Timeout: {} ms", timeout);
    } else {
        println!("Timeout: <missing>");
    }

    // Show Debug File
    if let Some(debug_file) = args.debug_file {
        println!("Debug file: {}", debug_file);
        println!("Debug format: {}", args.debug_format.as_deref().unwrap_or("json"));
    } else {
        println!("Debug file: <missing>");
    }

    // Show IP Bitfield
    if let Some(ip_bit) = args.ip_bitfield {
        println!("IP Bitfield: 0x{:02X}", ip_bit);
    } else {
        println!("IP Bitfield: <missing>");
    }

    // Show Dry Run
    println!("Dry Run: {}", args.dry_run);

    // Show Count
    for i in 0..args.count {
        println!("Iteration {}: SRC={} DST={} PORT={} BIT={} SMAC={:02X?} DMAC={:02X?}",
            i,
            args.src_ip.as_deref().unwrap_or("<missing>"),
            args.dst_ip.as_deref().unwrap_or("<missing>"),
            args.dest_port.unwrap_or(0),
            args.ip_bitfield.unwrap_or(0),
            args.src_mac.unwrap_or([0; 6]),
            args.dst_mac.unwrap_or([0; 6])
        );
    }
}