# RawSocketNetworkScanner

Un scanner réseau Rust qui construit et envoie des paquets via raw socket, construisant manuellement les en-têtes L2/L3/L4.

## Instructions de build

### Mode standard (avec std)
```bash
cargo build --release
cargo run -- <options>
```

### Mode no_std (bibliothèque uniquement)
```bash
cargo build --lib --no-default-features --features alloc
```

## Exécutions attendues (exemples)

Ces commandes sont supportées individuellement. Les noms des options sont strictement ceux-ci:

```bash
cargo run -- --src_ip=192.168.25.2
cargo run -- --dst_ip=192.168.1.25
cargo run -- --dest_port=8080
cargo run -- --src_mac=aa:bb:cc:dd:ee:ff
cargo run -- --dst_mac=11:22:33:44:55:66
cargo run -- --l4_protocol=udp
cargo run -- --l4_protocol=tcp
cargo run -- --timeout_ms=2000
cargo run -- --debug_file=./debug.pcap --debug_format=pcap
cargo run -- --debug_file=./debug.json --debug_format=json
cargo run -- --ip_bitfield=0x04 --dry_run
```

## Fonctionnalités

- **Construction de paquets** : Ethernet (L2), IPv4 (L3), UDP/TCP (L4) avec checksums corrects
- **Raw sockets** : Envoi de paquets via raw socket (nécessite privilèges root)
- **Mode dry_run** : Génération de paquets sans envoi réseau (recommandé pour les tests)
- **Formats de debug** : JSON et PCAP (compatible Wireshark)
- **Architecture no_std** : Support des environnements embarqués
- **Mode silencieux** : Aucune sortie sur stdout (erreurs sur stderr uniquement)

## Privilèges requis

- **Envoi de paquets** : Nécessite les privilèges root (`sudo`) pour créer des raw sockets
- **Mode dry_run** : Aucun privilège requis (recommandé pour les tests)

## Déclaration éthique

⚠️ **IMPORTANT** : Ce scanner réseau doit être utilisé UNIQUEMENT dans des environnements autorisés. L'utilisation de cet outil sur des réseaux sans autorisation explicite est illégale et contraire à l'éthique. Les développeurs déclarent qu'ils utiliseront cet outil uniquement pour des tests légitimes et autorisés.

## Architecture

Le projet utilise une architecture modulaire avec :
- **packets/builder** : Construction modulaire des paquets (Ethernet, IPv4, TCP, UDP)
- **formats** : Sérialisation JSON et PCAP
- **sender** : Envoi via raw sockets
- **parsing** : Validation des arguments CLI
- **no_std** : Support des environnements embarqués
