#![cfg(feature = "std")]
use std::error::Error;
use std::process;

use projet_rsns_morissetlarresacha::cli::Args;
use projet_rsns_morissetlarresacha::structs::packet_builder::PacketBuilder;
use projet_rsns_morissetlarresacha::formats::format_factory::FormatFactory;
use projet_rsns_morissetlarresacha::structs::formats::FormatType;
use projet_rsns_morissetlarresacha::sender::raw_socket::{RawSocketSender, get_interface_index};

fn main() {
    let args = Args::parse_args();

    match build_and_send_packet(&args) {
        Ok(_) => {
            // Mode silencieux - pas de sortie sur stdout
        }
        Err(e) => {
            eprintln!("Erreur: {}", e);
            process::exit(1);
        }
    }
}

fn build_and_send_packet(args: &Args) -> Result<(), Box<dyn Error>> {
    let packet_builder = PacketBuilder::from_cli_args(
        args.src_ip.as_deref(),
        args.dst_ip.as_deref(),
        args.src_mac.clone(),
        args.dst_mac.clone(),
        Some(12345), // Port source par défaut
        args.dest_port.clone(),
        args.l4_protocol.as_deref(),
        args.ip_bitfield.clone(),
        Some(b"Hello, Network!".to_vec()), // Payload de test
    )?;

    let network_packet = packet_builder.build_packet()?;
    let packet_bytes = network_packet.assemble_packet()?;

    // Écrire le fichier de debug si demandé
    if let Some(path) = args.debug_file.as_ref() {
        let factory = FormatFactory::new();
        let bytes = match args.debug_format.as_deref() {
            Some("pcap") => factory.write_packet(&network_packet, FormatType::Pcap)?,
            _ => factory.write_packet(&network_packet, FormatType::Json)?,
        };
        std::fs::write(path, &bytes)?;
    }

    // Envoyer le paquet seulement si pas en mode dry_run ET si on a les arguments nécessaires
    if !args.dry_run && args.src_ip.is_some() && args.dst_ip.is_some() {
        // Essayer de détecter l'interface réseau automatiquement
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

fn detect_interface() -> Result<String, Box<dyn Error>> {
    // Essayer de détecter l'interface réseau principale
    let interfaces = ["eth0", "enp0s3", "wlan0", "lo"];
    
    for iface in &interfaces {
        if get_interface_index(iface).is_ok() {
            return Ok(iface.to_string());
        }
    }
    
    Err("Aucune interface réseau disponible".into())
}
