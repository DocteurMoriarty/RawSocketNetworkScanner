# Functions de parsing

Les modules parsing est utilisé pour rendre un formats commun au programme et check le format d'entrer des parametres fournis par l'utilisateurs.

## Parsing

### `my_parsing.rs`
- **Responsabilité** : Gestion des format d'entré, retour d'une erreur en cas de mauvais format d'entré
- **Méthodes principales** :
  - `parse_hex(str)` : Parse les chaines de charactères string en hex
  - `parse_ipv4(str)` : Parse les chaines de charactères string en un tableau de 4 entier non signer de 8 bits, représentent l'ip
  - `parse_mac(str)` : Parse les chaines de charactères string en un tableau de 6 entier non signer de 8 bits, représentent l'addresse mac