# Formats

Le module `formats` fournit la sérialisation/désérialisation des paquets réseau en JSON et PCAP, ainsi qu’une fabrique pour produire les implémentations selon un `FormatType`.

## Structure des modules

### `pcap.rs`
- **Responsabilité**: Gestion du format PCAP (Packet Capture)
- **Structures**:
  - `PcapWriter` — writer PCAP basé sur un `VecNoStd<u8>` interne
  - `PcapReader` — reader PCAP depuis un buffer
- **Méthodes principales**:
  - `PcapWriter::new()` — crée un writer
  - `write_global_header()` — écrit l’en-tête global PCAP
  - `write_packet(&NetworkPacket)` — écrit un paquet capturé
  - `get_data()` — vue sur le buffer interne
  - `into_data()` — consomme et renvoie le buffer
  - `PcapReader::new(data)` — crée un reader depuis des octets
  - `read_global_header()` — lit et valide l’en-tête global
  - `read_next_packet()` — lit le prochain paquet (octets)
  - `has_more_packets()` — indique s’il reste des paquets

### `json.rs`
- **Responsabilité**: Sérialisation JSON légère (no_std-friendly via `serde_json_core`)
- **Structures**:
  - `JsonSerializer` — options de sérialisation (`include_raw_data`)
  - `JsonDeserializer` — lecture de structures JSON
  - `JsonPacket`, `JsonEthernet`, `JsonIpv4`, `JsonL4`, `JsonMetadata`, `JsonValue`
- **Méthodes principales**:
  - `JsonSerializer::new()` / `without_raw_data()` — configuration
  - `serialize_packet(&NetworkPacket)` — sérialise un paquet en `StringNoStd`
  - `serialize_packets(&[NetworkPacket])` — sérialise un tableau de paquets
  - `JsonDeserializer::new()` — crée un désérialiseur
  - `deserialize_packet(&str)` — parse un `JsonPacket`
  - `deserialize_packets(&str)` — parse plusieurs `JsonPacket`

### `format_factory.rs`
- **Responsabilité**: Fabrique et traits communs pour writers/readers
- **Types**:
  - `FormatFactory` — point d’entrée pour créer writer/reader
  - `FormatType` — `Pcap | Json`
  - `FormatWriter`, `FormatReader` — contrats communs
- **Méthodes principales**:
  - `FormatFactory::new()` — crée la fabrique
  - `create_writer(FormatType)` — `PcapWriter` ou `JsonSerializer`
  - `create_reader(FormatType, VecNoStd<u8>)` — `PcapReader` ou `JsonDeserializer`
  - `write_packet(&NetworkPacket, FormatType)` — sérialise un paquet (retourne bytes)
  - `write_packets(&[NetworkPacket], FormatType)` — sérialise plusieurs paquets
