# Architecture modulaire des formats

## Vue d'ensemble

Le module `formats` fournit une architecture modulaire pour la sérialisation et désérialisation des paquets réseau dans différents formats.

## Structure des modules

### 1. `pcap.rs`
- **Responsabilité** : Gestion du format PCAP (Packet Capture)
- **Structs publics** : 
  - `PcapWriter` : Écriture de fichiers PCAP
  - `PcapReader` : Lecture de fichiers PCAP
- **Méthodes principales** :
  - `PcapWriter::new()` : Création d'un writer PCAP
  - `write_global_header()` : Écriture de l'en-tête global PCAP
  - `write_packet(packet)` : Écriture d'un paquet
  - `PcapReader::new(data)` : Création d'un reader PCAP
  - `read_global_header()` : Lecture de l'en-tête global
  - `read_next_packet()` : Lecture du prochain paquet

### 2. `json.rs`
- **Responsabilité** : Sérialisation/désérialisation JSON
- **Structs publics** :
  - `JsonSerializer` : Sérialisation vers JSON
  - `JsonDeserializer` : Désérialisation depuis JSON
  - `JsonPacket` : Structure de données JSON
- **Méthodes principales** :
  - `JsonSerializer::new()` : Création d'un sérialiseur
  - `serialize_packet(packet)` : Sérialisation d'un paquet
  - `serialize_packets(packets)` : Sérialisation de plusieurs paquets
  - `JsonDeserializer::new()` : Création d'un désérialiseur
  - `deserialize_packet(json)` : Désérialisation d'un paquet

### 3. `format_factory.rs`
- **Responsabilité** : Factory pattern pour les formats
- **Struct public** : `FormatFactory`
- **Enum** : `FormatType` (Pcap, Json)
- **Traits** : `FormatWriter`, `FormatReader`
- **Méthodes principales** :
  - `create_writer(format_type)` : Création d'un writer
  - `create_reader(format_type, data)` : Création d'un reader
  - `write_packet(packet, format_type)` : Écriture directe
  - `write_packets(packets, format_type)` : Écriture multiple

## Avantages de cette architecture

1. **Extensibilité** : Facile d'ajouter de nouveaux formats
2. **Uniformité** : Interface commune via les traits
3. **Flexibilité** : Support de formats binaires et textuels
4. **Réutilisabilité** : Composants modulaires
5. **Type Safety** : Sécurité des types avec les enums