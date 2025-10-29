# Utils

Le module `utils` fournit outils modulaire et redevelloper pour les besoins du projet.

## Structure des modules


### `checksum.rs`
- **Responsabilité** : Calcul du checksum
- **Méthodes principales** :
  - `internet_checksum` : Calcul du checksum

### `convert_bytes.rs`
- **Responsabilité** : Conversion de valeurs numériques en bytes
- **Méthodes principales** :
  - `convert_n_to_bytes` : Convertit un entier (1, 2, 4 ou 8 octets) en vecteur d’octets en big endian, avec vérification de la taille et gestion des erreurs (InvalidLengthBytes, ValueTooLarge).

### `convert_string.rs`
- **Responsabilité** : Conversion d’une Option<String> en Option<&str> sans allocation
- **Méthodes principales** :
  - `my_as_deref` : Retourne une référence slice (&str) au contenu de la chaîne contenue dans une Option<String>.

### `format_mac.rs`
- **Responsabilité** : Conversion de MAC address en chaîne lisible
- **Méthodes principales** :
  - `mac_to_string` : Transforme un tableau de 6 octets en une chaîne formatée MAC en hexadécimal majuscule.

### `push_bytes.rs`
- **Responsabilité** : Copie de données dans un buffer à un offset donné
- **Méthodes principales** :
  - `push_bytes` : Écrit un slice de bytes (data) dans un buffer existant (buf) à partir d’un offset spécifique.
