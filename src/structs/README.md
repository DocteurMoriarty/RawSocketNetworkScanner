# Structs

Le module `structs` regroupe toutes les structures de données partagées du projet: en-têtes réseaux (Ethernet, IPv4, TCP, UDP), types d’assemblage de paquets, formats de sérialisation et métadonnées.

## Structure des modules

### `ethernet.rs`
- **Responsabilité**: Définition de l’en-tête Ethernet (L2)
- **Structures**:
  - `EthernetHeader` — `src_mac`, `dst_mac`, `ethertype`

### `ip.rs`
- **Responsabilité**: Définition de l’en-tête IPv4 (L3)
- **Structures**:
  - `Ipv4Header` — champs standard (version, ihl, dscp, total_length, identification, flags, fragment_offset, ttl, protocol, header_checksum, `src_addr`, `dst_addr`, `options`)

### `ipv4.rs`
- **Responsabilité**: Représentation compacte d’une adresse IPv4
- **Structures**:
  - `Ipv4Addr` — `octets: [u8; 4]`

### `tcp.rs`
- **Responsabilité**: Définition de l’en-tête TCP (L4)
- **Structures**:
  - `TcpHeader` — ports, numéros de séquence/ack, `data_offset`, `flags`, `window`, `checksum`, `urgent_pointer`, `options`, `payload`

### `udp.rs`
- **Responsabilité**: Définition de l’en-tête UDP (L4)
- **Structures**:
  - `UdpHeader` — `src_port`, `dst_port`, `length`, `checksum`, `payload`

### `l4_protocol.rs`
- **Responsabilité**: Types de haut niveau pour la couche 4
- **Types**:
  - `L4Data` — enum encapsulant `Tcp(TcpHeader)` ou `Udp(UdpHeader)`
  - `L4Protocol` — enum du protocole (`Tcp` | `Udp`)

### `network_packet.rs`
- **Responsabilité**: Représentation d’un paquet réseau complet (L2+L3+L4)
- **Structures**:
  - `NetworkPacket` — `ethernet: EthernetHeader`, `ipv4: Ipv4Header`, `l4_data: L4Data`

### `packet_builder.rs`
- **Responsabilité**: Paramètres de construction d’un paquet réseau
- **Structures**:
  - `PacketBuilder` — sources/destinations IP/MAC, ports, `L4Protocol`, `ip_bitfield`, `payload`

### `formats.rs`
- **Responsabilité**: Type d’export/support de sérialisation
- **Types**:
  - `FormatType` — `Pcap | Json`

### `pcap.rs`
- **Responsabilité**: Structures de base pour manipuler un buffer PCAP
- **Structures**:
  - `PcapWriter` — `buffer: VecNoStd<u8>`
  - `PcapReader` — `data: VecNoStd<u8>`, `position: usize`

### `json.rs`
- **Responsabilité**: Structures de sérialisation JSON dédiées (no_std-friendly)
- **Structures**:
  - `JsonValue` — variant léger (`U64`, `Bool`, `String`)
  - `JsonPacket` — vue JSON du paquet (Ethernet, IPv4, L4, `metadata`)
  - `JsonEthernet`, `JsonIpv4`, `JsonL4`, `JsonMetadata`
  - `JsonSerializer` (`include_raw_data`) et `JsonDeserializer`

### `mod.rs`
- **Responsabilité**: Ré-export des sous-modules

## Notes
- Les structures privilégient des types simples (`[u8; N]`, `u16`, `u32`) pour un contrôle fin de l’assemblage binaire et la compatibilité `no_std`.
- Les champs `options` et `payload` sont optionnels et utilisent `VecNoStd<u8>` quand pertinent.
