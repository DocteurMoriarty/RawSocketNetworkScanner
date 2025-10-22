#Les builders ont été dévellopé avec une architecture modulaire.

## Structure des modules

### 1. `tcp_builder.rs`
- **Responsabilité** : Construction des en-têtes TCP
- **Struct public** : `TcpBuilder`
- **Méthodes principales** :
  - `new(src_ip, dst_ip)` : Création d'un builder TCP
  - `build_tcp_header(src_port, dst_port, payload)` : Construction de l'en-tête TCP avec calcul de checksum

### 2. `udp_builder.rs`
- **Responsabilité** : Construction des en-têtes UDP
- **Struct public** : `UdpBuilder`
- **Méthodes principales** :
  - `new(src_ip, dst_ip)` : Création d'un builder UDP
  - `build_udp_header(src_port, dst_port, payload)` : Construction de l'en-tête UDP avec calcul de checksum

### 3. `ipv4_builder.rs`
- **Responsabilité** : Construction des en-têtes IPv4
- **Struct public** : `Ipv4Builder`
- **Méthodes principales** :
  - `new(src_ip, dst_ip, ip_bitfield)` : Création d'un builder IPv4
  - `build_ipv4_header(l4_data)` : Construction de l'en-tête IPv4 avec calcul de checksum

### 4. `ethernet_builder.rs`
- **Responsabilité** : Construction des en-têtes Ethernet
- **Struct public** : `EthernetBuilder`
- **Méthodes principales** :
  - `new()` : Création d'un builder Ethernet
  - `build_ethernet_header(src_mac, dst_mac)` : Construction de l'en-tête Ethernet

### 5. `packet_assembler.rs`
- **Responsabilité** : Assemblage final des paquets
- **Struct public** : `PacketAssembler`
- **Méthodes principales** :
  - `new()` : Création d'un assembleur
  - `assemble_packet(packet)` : Assemblage complet du paquet
  - `get_packet_size(packet)` : Calcul de la taille du paquet

### 6. `packet_factory.rs`
- **Responsabilité** : Orchestration de la construction de paquets
- **Struct public** : `PacketFactory`
- **Méthodes principales** :
  - `new(src_ip, dst_ip, ip_bitfield)` : Création d'une factory
  - `build_packet(builder)` : Construction complète d'un paquet réseau
  - `from_cli_args(...)` : Création depuis les arguments CLI

## Avantages de cette architecture

1. **Séparation des responsabilités** : Chaque module a une responsabilité claire
2. **Réutilisabilité** : Les builders peuvent être utilisés indépendamment
3. **Testabilité** : Chaque composant peut être testé isolément
4. **Maintenabilité** : Code plus facile à maintenir et à étendre
5. **Modularité** : Possibilité d'ajouter facilement de nouveaux protocoles

## Utilisation

```rust
use crate::packets::builder::packet_factory::PacketFactory;

let factory = PacketFactory::new(src_ip, dst_ip, ip_bitfield);
let packet = factory.build_packet(&packet_builder)?;
```
