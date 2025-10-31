#![cfg(feature = "std")]
use std::error::Error;
use std::process;

use projet_rsns_morissetlarresacha::{
    cli::Args,
    structs::{
        packet_builder::PacketBuilder,
        formats::FormatType,
        socket::RawSocketSender
    },
    formats::format_factory::FormatFactory,
    sender::raw_socket::get_interface_index
};

// Main function
fn main() {
    let args = Args::parse_args();

    match build_and_send_packet(&args) {
        Ok(_) => {

        }
        Err(e) => {
            eprintln!("Erreur: {}", e);
            process::exit(1);
        }
    }
}

// Construit et envoie le paquet reseau selon les arguments fournis
fn build_and_send_packet(args: &Args) -> Result<(), Box<dyn Error>> {
    let packet_builder = PacketBuilder::from_cli_args(
        args.src_ip.as_deref(),
        args.dst_ip.as_deref(),
        args.src_mac.clone(),
        args.dst_mac.clone(),
        Some(12345),
        args.dest_port.clone(),
        args.l4_protocol.as_deref(),
        args.ip_bitfield.clone(),
        Some(b"Hello, Network!".to_vec()),
    )?;

    let network_packet = packet_builder.build_packet()?;
    let packet_bytes = network_packet.assemble_packet()?;

    if let Some(path) = args.debug_file.as_ref() {
        let factory = FormatFactory::new();
        let bytes = match args.debug_format.as_deref() {
            Some("pcap") => factory.write_packet(&network_packet, FormatType::Pcap)?,
            _ => factory.write_packet(&network_packet, FormatType::Json)?,
        };
        std::fs::write(path, &bytes)?;
    }

    if !args.dry_run && args.src_ip.is_some() && args.dst_ip.is_some() {
        if let Ok(iface) = detect_interface() {
            if let Ok(if_index) = get_interface_index(&iface) {
                if let Ok(sender) = RawSocketSender::new() {
                    let _ = sender.set_write_timeout(args.timeout_ms.as_ref().copied());
                    let _ = sender.send(if_index, network_packet.ethernet.dst_mac, &packet_bytes);
                }
            }
        }
    }

    Ok(())
}

// Detecte automatiquement une interface réseau disponible
fn detect_interface() -> Result<String, Box<dyn Error>> {
    let interfaces = ["eth0", "enp0s3", "wlan0", "lo"];
    for iface in &interfaces {
        if get_interface_index(iface).is_ok() {
            return Ok(iface.to_string());
        }
    }

    Err(
        "Aucune interface réseau disponible".into()
    )
}
