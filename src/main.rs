#![cfg(feature = "std")]
use std::error::Error;
use std::process;

use projet_rsns_morissetlarresacha::cli::args::parse_args;
use projet_rsns_morissetlarresacha::cli::args::Args;
use projet_rsns_morissetlarresacha::structs::packet_builder::PacketBuilder;
use projet_rsns_morissetlarresacha::structs::l4_protocol::L4Data;
use projet_rsns_morissetlarresacha::formats::format_factory::{FormatFactory, FormatType};
use projet_rsns_morissetlarresacha::sender::raw_socket::{RawSocketSender, get_interface_index};

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
        args.src_ip().as_deref(),
        args.dst_ip().as_deref(),
        args.src_mac().clone(),
        args.dst_mac().clone(),
        Some(12345), // Port source par défaut
        args.dest_port().clone(),
        args.l4_protocol().clone().as_deref(),
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

    if args.dry_run().as_ref().copied().unwrap_or(false) {
        if let Some(path) = args.debug_file().as_ref() {
            let factory = FormatFactory::new();
            let bytes = match args.debug_format().as_deref() {
                Some("pcap") => factory.write_packet(&network_packet, FormatType::Pcap)?,
                _ => factory.write_packet(&network_packet, FormatType::Json)?,
            };
            std::fs::write(path, &bytes)?;
            println!("Fichier debug écrit: {} ({} octets)", path, bytes.len());
        }
        return Ok(packet_bytes);
    }

    let iface = "eth0";
    let if_index = get_interface_index(iface)?;
    let sender = RawSocketSender::new()?;
    sender.set_write_timeout(args.timeout_ms().as_ref().copied())?;
    let sent = sender.send(if_index, network_packet.ethernet.dst_mac, &packet_bytes)?;
    println!("Paquet envoyé: {} octets via {}", sent, iface);

    if let Some(path) = args.debug_file().as_ref() {
        let factory = FormatFactory::new();
        let bytes = match args.debug_format().as_deref() {
            Some("pcap") => factory.write_packet(&network_packet, FormatType::Pcap)?,
            _ => factory.write_packet(&network_packet, FormatType::Json)?,
        };
        std::fs::write(path, &bytes)?;
        println!("Fichier debug écrit: {} ({} octets)", path, bytes.len());
    }

    Ok(packet_bytes)
}
