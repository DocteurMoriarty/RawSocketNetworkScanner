use projet_rsns_morissetlarresacha::{
    structs::packet_builder::PacketBuilder,
    formats::format_factory::FormatFactory,
    structs::formats::FormatType,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let packet_builder = PacketBuilder::from_cli_args(
        Some("192.168.1.1"),
        Some("192.168.1.2"),
        None,
        None,
        Some(12345),
        Some(80),
        Some("tcp"),
        None,
        Some(b"Hello, Network!".to_vec()),
    )?;

    let packet = packet_builder.build_packet()?;
    let factory = FormatFactory::new();

    println!("=== Exemple d'utilisation des formats ===\n");

    let pcap_data = factory.write_packet(&packet, FormatType::Pcap)?;
    println!("Données PCAP générées: {} octets", pcap_data.len());
    println!("Premiers 32 octets PCAP: {:02X?}\n", &pcap_data[..pcap_data.len().min(32)]);

    let json_data = factory.write_packet(&packet, FormatType::Json)?;
    let json_str = String::from_utf8(json_data)?;
    println!("Données JSON générées:");
    println!("{}", json_str);

    Ok(())
}
