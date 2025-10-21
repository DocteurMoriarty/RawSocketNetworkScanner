use std::error::Error;
use std::process;

use projet_rsns_morissetlarresacha::cli::args::parse_args;
use projet_rsns_morissetlarresacha::cli::args::Args;
use projet_rsns_morissetlarresacha::structs::packet_builder::PacketBuilder;
use projet_rsns_morissetlarresacha::structs::l4_protocol::L4Data;

fn main() {
    let args = parse_args();

    match build_and_display_packet(&args) {
        Ok(packet_bytes) => {
            println!("Taille du paquet : {} octets", packet_bytes.len());
            println!(
                "Premiers 32 octets : {:02X?}",
                &packet_bytes[..packet_bytes.len().min(32)]
            );
        }
        Err(e) => {
            process::exit(1);
        }
    }
}

fn build_and_display_packet(args: &Args) -> Result<Vec<u8>, Box<dyn Error>> {
    let packet_builder = PacketBuilder::from_cli_args(
        args.src_ip().clone(),
        args.dst_ip().clone(),
        args.src_mac().clone(),
        args.dst_mac().clone(),
        Some(12345), // Port source par défaut
        args.dest_port().clone(),
        args.l4_protocol().clone(),
        args.ip_bitfield().clone(),
        Some(b"Hello, Network!".to_vec()), // Payload de test
    )?;

    let network_packet = packet_builder.build_packet()?;

    println!("\n=== INFORMATIONS DU PAQUET ===");
    println!("Ethernet");
    println!("  ↪ SRC MAC     : {:02X?}", network_packet.ethernet.src_mac);
    println!("  ↪ DST MAC     : {:02X?}", network_packet.ethernet.dst_mac);
    println!("  ↪ EtherType   : 0x{:04X}", network_packet.ethernet.ethertype);

    println!("IPv4");
    println!("  ↪ SRC IP      : {:?}", network_packet.ipv4.src_addr);
    println!("  ↪ DST IP      : {:?}", network_packet.ipv4.dst_addr);
    println!("  ↪ Protocol    : {}", network_packet.ipv4.protocol);
    println!("  ↪ Total Length: {}", network_packet.ipv4.total_length);
    println!("  ↪ Checksum    : 0x{:04X}", network_packet.ipv4.header_checksum);

    match &network_packet.l4_data {
        L4Data::Tcp(tcp) => {
            println!("TCP");
            println!("  ↪ SRC Port    : {}", tcp.src_port);
            println!("  ↪ DST Port    : {}", tcp.dst_port);
            println!("  ↪ Flags       : 0x{:04X}", tcp.flags);
            println!("  ↪ Checksum    : 0x{:04X}", tcp.checksum);
        }
        L4Data::Udp(udp) => {
            println!("UDP");
            println!("  ↪ SRC Port    : {}", udp.src_port);
            println!("  ↪ DST Port    : {}", udp.dst_port);
            println!("  ↪ Length      : {}", udp.length);
            println!("  ↪ Checksum    : 0x{:04X}", udp.checksum);
        }
    }

    println!("Taille totale du paquet : {} octets", network_packet.get_packet_size());

    let packet_bytes = network_packet.assemble_packet()?;
    Ok(packet_bytes)
}
