# Sender

Le module `sender` fournit l’envoi de paquets via des sockets bruts (raw sockets). Implémentation actuelle Linux (`AF_PACKET`).

## Structure des modules

### `raw_socket.rs`
- **Responsabilité**: Création et gestion d’un socket brut couche 2 et envoi de trames construites (Ethernet + payload).
- **Structures**:
  - `RawSocketSender` — wrapper autour d’un descripteur `libc` pour `AF_PACKET`.
- **Fonctions/méthodes principales**:
  - `RawSocketSender::new()` — ouvre un socket `AF_PACKET` / `SOCK_RAW` (`ETH_P_ALL`).
  - `set_write_timeout(timeout_ms: Option<u64>)` — configure `SO_SNDTIMEO`.
  - `send(if_index: i32, dst_mac: [u8; 6], packet: &[u8])` — envoie une trame via `sendto` et `sockaddr_ll`.
  - `Drop` — ferme le descripteur (`close`).
  - `get_interface_index(name: &str)` — récupère l’index d’interface via `if_nametoindex`.

## Notes d’utilisation
- **Plateforme**: Linux uniquement (utilise `AF_PACKET`). Sous Windows, utiliser WSL2/VM Linux, ou implémenter un backend Npcap/WinPcap séparé.
- **Permissions**: nécessite des privilèges élevés (root) pour créer un socket brut.
- **Interface**: fournir un nom d’interface existant pour obtenir un `if_index` valide (ex.: `eth0`, `enp0s3`, `wlan0`).

## Exemple d’envoi (schématique)
```rust
let sender = RawSocketSender::new()?;
sender.set_write_timeout(Some(2000))?;
let if_index = get_interface_index("eth0")?;
let bytes_sent = sender.send(if_index, dst_mac, &packet_bytes)?;
```

