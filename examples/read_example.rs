use projet_rsns_morissetlarresacha::cli::{args, args::Args};

fn main() {
    let args: Args = args::parse_args();

    // Show IP Addresses
    let src_ip = args.src_ip().as_deref().unwrap_or("<missing>");
    let dst_ip = args.dst_ip().as_deref().unwrap_or("<missing>");
    println!("SRC IP: {} -> DST IP: {}", src_ip, dst_ip);

    // Show MAC Addresses
    if let Some(src_mac) = args.src_mac() {
        println!("SRC MAC: {:02X?}", src_mac);
    }

    // Show other options 
    if let Some(dst_mac) = args.dst_mac() {
        println!("DST MAC: {:02X?}", dst_mac);
    }

    if let Some(l4) = args.l4_protocol() {
        println!("Layer4 Protocol: {}", l4);
    }

    if let Some(timeout) = args.timeout_ms() {
        println!("Timeout: {} ms", timeout);
    }

    if let Some(debug_file) = args.debug_file() {
        println!("Debug file: {}", debug_file);
        println!("Debug format: {}", args.debug_format().as_deref().unwrap_or("json"));
    }

    if let Some(ip_bit) = args.ip_bitfield() {
        println!("IP bitfield: 0x{:02X}", ip_bit);
    }

    if let Some(dry_run) = args.dry_run() {
        println!("Dry run: {}", dry_run);
    }

    for i in 0..*args.count() {
        println!(
            "Packet {} -> {}:{} | IP bitfield=0x{:02X} | SRC MAC={:02X?} | DST MAC={:02X?}",
            i + 1,
            src_ip,
            args.dest_port().unwrap_or(0),
            args.ip_bitfield().unwrap_or(0),
            args.src_mac().unwrap_or([0; 6]),
            args.dst_mac().unwrap_or([0; 6])
        );
    }
}
