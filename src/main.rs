use cli::args::{parse_args, Args};
use packet_builder::{PacketBuilder, L4Protocol};

fn main() {
    let args: Args = parse_args();

    // Exemple d'utilisation de la fonction d'assemblage de paquets
    match build_and_display_packet(&args) {
        Ok(packet_bytes) => {
            println!("Paquet assemblé avec succès !");
            println!("Taille du paquet: {} bytes", packet_bytes.len());
            println!("Premiers 32 bytes: {:02X?}", &packet_bytes[..std::cmp::min(32, packet_bytes.len())]);
        }
        Err(e) => {
            eprintln!("Erreur lors de l'assemblage du paquet: {}", e);
            std::process::exit(1);
        }
    }
}

fn build_and_display_packet(args: &Args) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Construire le PacketBuilder à partir des arguments CLI
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

    // Construire le paquet complet
    let network_packet = packet_builder.build_packet()?;

    // Afficher les informations du paquet
    println!("=== INFORMATIONS DU PAQUET ===");
    println!("Ethernet - SRC MAC: {:02X?}", network_packet.ethernet.src_mac);
    println!("Ethernet - DST MAC: {:02X?}", network_packet.ethernet.dst_mac);
    println!("Ethernet - EtherType: 0x{:04X}", network_packet.ethernet.ethertype);
    
    println!("IPv4 - SRC IP: {:?}", network_packet.ipv4.src_addr);
    println!("IPv4 - DST IP: {:?}", network_packet.ipv4.dst_addr);
    println!("IPv4 - Protocol: {}", network_packet.ipv4.protocol);
    println!("IPv4 - Total Length: {}", network_packet.ipv4.total_length);
    println!("IPv4 - Header Checksum: 0x{:04X}", network_packet.ipv4.header_checksum);

    match &network_packet.l4_data {
        packet_builder::L4Data::Tcp(tcp) => {
            println!("TCP - SRC Port: {}", tcp.src_port);
            println!("TCP - DST Port: {}", tcp.dst_port);
            println!("TCP - Flags: 0x{:04X}", tcp.flags);
            println!("TCP - Checksum: 0x{:04X}", tcp.checksum);
        }
        packet_builder::L4Data::Udp(udp) => {
            println!("UDP - SRC Port: {}", udp.src_port);
            println!("UDP - DST Port: {}", udp.dst_port);
            println!("UDP - Length: {}", udp.length);
            println!("UDP - Checksum: 0x{:04X}", udp.checksum);
        }
    }

    println!("Taille totale du paquet: {} bytes", network_packet.get_packet_size());

    // Assembler le paquet final en bytes
    let packet_bytes = network_packet.assemble_packet()?;

    Ok(packet_bytes)
}
