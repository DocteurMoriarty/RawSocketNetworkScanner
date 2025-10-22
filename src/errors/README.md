# Gestion des erreurs personnaliser 

Les erreurs pouvant etre presente dans le projet sont toutes prisent en charges de maniere personnaliser grace a la methode ParseError.
## Structure du fichier

### 1. `errors.rs`
- **Responsabilité** : Gestion des erreurs personnaliser
- **Enum** : 
  - `ParseError` : Structure des differentes erreurs possible dans le projet
- **Méthodes** :
  - `PcapWriter::new()` : Création d'un writer PCAP
  - `write_global_header()` : Écriture de l'en-tête global PCAP
  - `write_packet(packet)` : Écriture d'un paquet
  - `PcapReader::new(data)` : Création d'un reader PCAP
  - `read_global_header()` : Lecture de l'en-tête global
  - `read_next_packet()` : Lecture du prochain paquet
