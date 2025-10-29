# Assignment: Raw-Socket Network Scanner (Rust)

## Résumé du projet

Un scanner réseau simple en Rust qui construit et (optionnellement) envoie des paquets via des sockets bruts (raw sockets), en construisant manuellement les en-têtes L2/L3/L4.

## Instructions de build

### Build standard (requis pour le projet)

Le projet doit être compilable avec `cargo build --release` sur un environnement Linux standard :

```bash
cargo build --release
cargo run --release -- <options>
```

### Mode no_std (travail supplémentaire)

Le projet supporte également une compilation en mode `no_std` pour environnements embarqués :

```bash
cargo build --lib --no-default-features --features alloc
```

## Interface en ligne de commande (CLI)

Le programme doit être invoquable avec `cargo run -- <options>` ou `cargo run --release -- <options>`. Chaque option doit être acceptée comme un flag séparé, même si elles peuvent être combinées lors des tests. Le programme doit terminer avec le code de sortie 0 en cas de succès et un code non-zéro en cas d'erreur fatale.

### Flags supportés (testés individuellement)

- `--src_ip=<IPv4>` : Adresse IPv4 source à placer dans l'en-tête IP (ex: `192.168.25.2`)
- `--dst_ip=<IPv4>` : Adresse IPv4 destination à placer dans l'en-tête IP (ex: `192.168.1.25`)
- `--dest_port=<port>` : Numéro de port destination (couche 4) (ex: `8080`)
- `--src_mac=<aa:bb:cc:dd:ee:ff>` : Adresse MAC source à utiliser à la couche Ethernet (ex: `aa:bb:cc:dd:ee:ff`)
- `--dst_mac=<aa:bb:cc:dd:ee:ff>` : Adresse MAC destination à utiliser à la couche Ethernet (ex: `11:22:33:44:55:66`)
- `--l4_protocol=<udp|tcp>` : Choisir le protocole de couche 4 pour la sonde (ex: `udp` ou `tcp`)
- `--timeout_ms=<milliseconds>` : Délai d'attente entre les tentatives de sondes/retry (ex: `2000`)
- `--debug_file=<path>` : Écrire la sortie de debug dans le fichier nommé (ex: `./debug.pcap`)
- `--debug_format=<json|pcap>` : Format du fichier de debug : soit `json` soit `pcap` (ex: `json`)
- `--ip_bitfield=<hex>` : Valeur brute 8 bits à OU logique dans le champ flags/bitfield de l'en-tête IPv4 (pour modéliser le "evil bit") ; accepte des valeurs comme `0x00` ou `0x04` (ex: `0x04`)
- `--dry_run` : Ne pas envoyer de paquets sur le réseau ; à la place, écrire les paquets dans le fichier de debug. **Recommandé pour les tests** et requis si le programme nécessite des privilèges root mais que le grader s'exécute sans accès privilégié.

**Note** : Les noms des flags ci-dessus sont stricts et ne doivent pas être modifiés — le runner automatisé attend ces noms exacts. D'autres flags optionnels peuvent être ajoutés (par exemple `--help`, `--verbosity`), mais ils doivent être documentés séparément.

## Exemples d'invocations (chaque commande testée individuellement)

Ces commandes correspondent exactement à celles que l'instructeur appellera individuellement :

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

## Exigences fonctionnelles

### Construction de paquets

Le programme doit construire les en-têtes Ethernet (L2), IPv4 (L3) et UDP/TCP (L4) selon les options sélectionnées. Les checksums doivent être définis correctement, sauf si vous documentez explicitement l'utilisation de l'offload du noyau. Si l'offload est utilisé, un `--dry_run` doit toujours afficher les valeurs de checksum prévues dans le fichier de debug.

### Champ bitfield IP

L'option `--ip_bitfield` doit être appliquée dans l'octet flags/fragmentation IPv4 que le programme écrit. Il est acceptable d'implémenter cela comme "définir l'octet supérieur (8 bits) Flags+FragmentOffset de l'en-tête IPv4 à la valeur fournie" pour ce devoir.

### Adresses MAC

Lorsque `--src_mac`/`--dst_mac` sont fournis, le programme doit utiliser ces valeurs pour les en-têtes Ethernet (ou les inclure dans le fichier de debug si `--dry_run` est utilisé).

### Timeout

L'option `--timeout_ms` contrôle le délai entre les paquets et/ou le comportement de timeout/retry.

### Modes de sortie

- **Format PCAP** : Si `--debug_format=pcap`, le programme doit sauvegarder les paquets qu'il a construits dans un fichier pcap à `--debug_file`. Le fichier pcap doit être lisible par Wireshark/tshark.
- **Format JSON** : Si `--debug_format=json`, le programme doit écrire un document JSON décrivant chaque paquet (champs d'en-tête L2/L3/L4 et checksums calculés). Le JSON doit être compact et analysable par machine.

### Mode silencieux (quiet mode)

Lorsqu'il est invoqué avec l'un des flags ci-dessus, le programme ne doit **pas** imprimer la progression du scan sur stdout/stderr. Les erreurs doivent toujours être imprimées sur stderr et retourner un code de sortie non-zéro.

## Privilèges requis

- **Envoi de paquets** : Nécessite les privilèges root (`sudo`) pour créer des sockets bruts (raw sockets). Sur Linux, utilisez `sudo cargo run --release -- <options>`.
- **Mode `--dry_run`** : Aucun privilège requis. **Recommandé pour les tests** et lors de l'exécution sans accès privilégié. Le but final est d'exécuter le binaire sans sudo lorsque possible.

## Déclaration éthique

**IMPORTANT** : Ce scanner réseau doit être utilisé **UNIQUEMENT** dans des environnements autorisés. L'utilisation de cet outil sur des réseaux sans autorisation explicite est illégale et contraire à l'éthique. Les développeurs déclarent qu'ils utiliseront cet outil uniquement pour des tests légitimes et autorisés dans des environnements contrôlés et permis.

## Architecture du projet

Le projet utilise une architecture modulaire avec les composants suivants :

- **`packets/builder`** : Construction modulaire des paquets (Ethernet, IPv4, TCP, UDP) avec calcul correct des checksums
- **`formats`** : Sérialisation JSON et PCAP avec support `no_std` via `serde_json_core`
- **`sender`** : Envoi via raw sockets (Linux `AF_PACKET`)
- **`parsing`** : Validation et parsing des arguments CLI et données réseau (MAC, IPv4, hex)
- **`structs`** : Structures de données pour représentations des en-têtes réseau (L2/L3/L4)
- **`utils`** : Utilitaires (checksum, conversion bytes, formatage MAC/IP)
- **`errors`** : Gestion d'erreurs centralisée avec types personnalisés
- **`no_std`** : Support des environnements embarqués avec allocation personnalisée

### Documentation Rust

Le projet inclut une documentation Rust complète avec exemples et code markdown comme montré en classe. Consultez la documentation avec :

```bash
cargo doc --open
```

## Travail supplémentaire (Further work)

### Mode no_std

Le projet a été partiellement refactorisé pour fonctionner en mode `no_std` avec :
- Utilisation de `alloc` pour l'allocation dynamique
- Types personnalisés `VecNoStd`, `StringNoStd` pour la compatibilité
- Support des formats JSON via `serde_json_core`
- Références : Articles de Phil Opp OS (https://os.phil-opp.com/) pour la compréhension du `no_std`

### Système de types

Le projet utilise le système de types de Rust pour construire une bibliothèque sûre qui détecte les erreurs qu'un développeur non averti pourrait faire au moment de la compilation, démontrant les avantages de Rust.

## Build et soumission

- Le projet doit être compilable avec `cargo build --release` sur un environnement Linux standard.
- Le dépôt doit inclure le code source (repo Git ou git bundle). Ne pas inclure de binaire mais garantir des builds reproductibles (un nix flake serait apprécié mais n'est pas attendu), sinon rendre le build plus robuste en fournissant des utilitaires utiles comme `rust-toolchain.toml` et en durcissant des versions spécifiques dans `Cargo.toml`.

## Structure des modules documentés

Chaque module majeur contient un `README.md` détaillant sa structure et ses responsabilités :
- `src/utils/README.md` - Utilitaires modulaires
- `src/structs/README.md` - Structures de données réseau
- `src/formats/README.md` - Formats de sérialisation
- `src/sender/README.md` - Envoi de paquets
- `src/errors/README.md` - Gestion d'erreurs
- `src/parsing/README.md` - Parsing d'entrées

## Remarques

Si quelque chose dans ce README n'est pas clair, ajustez le README mais ne changez pas les noms des flags — le grader attend les noms exacts pour les options. Pour toute question, référez-vous à l'instructeur.
