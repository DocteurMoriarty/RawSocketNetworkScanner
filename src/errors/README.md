
### `errors.rs`

**Responsabilité** : Gestion des erreurs de parsing réseau
**Type personnalisé** :
  - `Result<T>` : Alias pour `core::result::Result<T, ParseError>`
**Enum** :
  - `ParseError` : Définit les différentes erreurs possibles :
    - Adresse MAC ou IPv4 invalide
    - Trop/pas assez d’octets
    - Valeur hexadécimale incorrecte
    - Longueur invalide
    - Valeur trop grande pour la taille spécifiée
    - Champ requis manquant
    - Format incorrect
    - Erreurs JSON ou Serde

- **Implémentations** :
  - `fmt::Display` : Affichage lisible des erreurs
  - `From<serde_json_core::Error>` : Conversion automatique des erreurs Serde
  - `std::error::Error` (si la feature `std` est activée)

