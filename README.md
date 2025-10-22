# RawSocketNetworkScanner
Projet Rust conçu pour fournir un parseur d'arguments en ligne de commande (CLI) robuste, avec gestion d’erreurs personnalisée et tests unitaires complets. Il permet de manipuler et valider facilement des adresses IPv4, des adresses MAC, des champs hexadécimaux, ainsi que d’autres paramètres réseau de manière sécurisée et maintenable.

## Exécutions attendues (exemples)

Ces commandes sont supportées individuellement. Les noms des options sont strictement ceux-ci:

```
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

Notes:
- `--dry_run` génère uniquement le paquet (et éventuellement un fichier debug), sans envoi réseau.
- En l’absence de `--dry_run`, l’envoi se fait via raw socket.
- L’interface utilisée par défaut est interne au programme (aucun flag d’interface requis).
