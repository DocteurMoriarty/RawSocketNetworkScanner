# Guide : Tester l'envoi de paquets avec Wireshark

## Commandes de test

### 1. Test avec --dry_run (recommandé en premier)

Génère un fichier PCAP que vous pouvez ouvrir directement dans Wireshark :

```bash
cargo run --release -- \
  --src_ip=192.168.100.10 \
  --dst_ip=192.168.100.1 \
  --src_mac=12:34:56:78:9A:BC \
  --dst_mac=DE:AD:BE:EF:CA:FE \
  --dest_port=9999 \
  --l4_protocol=udp \
  --debug_file=./test_send.pcap \
  --debug_format=pcap \
  --dry_run
```

Ouvrez ensuite `test_send.pcap` dans Wireshark pour vérifier que le paquet est bien construit.

### 2. Test avec envoi réel (nécessite sudo sous Linux)

**Important** : Assurez-vous que `dst_mac` correspond à la MAC de votre passerelle ou d'une machine sur le même réseau.

```bash
sudo cargo run --release -- \
  --src_ip=192.168.100.10 \
  --dst_ip=192.168.100.1 \
  --src_mac=12:34:56:78:9A:BC \
  --dst_mac=DE:AD:BE:EF:CA:FE \
  --dest_port=9999 \
  --l4_protocol=udp \
  --debug_file=./test_send.pcap \
  --debug_format=pcap
```

## Configuration Wireshark pour capturer les paquets

### Méthode 1 : Capture en temps réel (recommandé)

1. **Ouvrir Wireshark** et sélectionner votre interface réseau (ex: `eth0`, `wlan0`)
2. **Démarrer la capture** (bouton bleu ▶)
3. **Appliquer un filtre** dans la barre de filtre :
   ```
   ip.src == 192.168.100.10 || ip.dst == 192.168.100.10 || udp.port == 9999
   ```
   
   Ou plus spécifique pour notre test :
   ```
   (eth.src == 12:34:56:78:9a:bc && eth.dst == de:ad:be:ef:ca:fe) || udp.port == 9999
   ```
4. **Exécuter votre commande** dans un autre terminal
5. **Arrêter la capture** après quelques secondes
6. **Rechercher votre paquet** :
   - Les paquets UDP seront visibles
   - Le payload "Hello, Network!" sera visible dans les données

### Méthode 2 : Utiliser le fichier PCAP généré

1. Le programme génère automatiquement `test_send.pcap` si `--debug_file` est spécifié
2. Ouvrez ce fichier dans Wireshark : `File > Open > test_send.pcap`
3. Vous verrez le paquet exactement comme il serait envoyé

## Filtres Wireshark pour identifier votre paquet

### Par adresse MAC source
```
eth.src == 12:34:56:78:9a:bc
```

### Par adresse MAC destination
```
eth.dst == de:ad:be:ef:ca:fe
```

### Par port UDP
```
udp.port == 9999
```

### Par IP source
```
ip.src == 192.168.100.10
```

### Combinaison complète
```
eth.src == 12:34:56:78:9a:bc && udp.port == 9999 && ip.src == 192.168.100.10
```

### Chercher le payload "Hello, Network!"
```
udp contains "Hello"
```

## Valeurs identifiables utilisées dans la commande de test

- **MAC source** : `12:34:56:78:9A:BC` (facile à repérer)
- **MAC dest** : `DE:AD:BE:EF:CA:FE` (classique "DEADBEEF CAFE")
- **IP source** : `192.168.100.10` (sous-réseau facile à isoler)
- **IP dest** : `192.168.100.1` (typiquement une passerelle)
- **Port dest** : `9999` (facile à filtrer dans Wireshark)
- **Protocole** : `UDP` (plus simple que TCP)
- **Payload** : `Hello, Network!` (visible dans les données)

## Vérifier que l'envoi fonctionne

### Si vous voyez le paquet dans Wireshark :
✅ **Succès !** Le paquet a bien été envoyé sur le réseau.

### Si vous ne voyez rien :
1. **Vérifiez les privilèges** : utilisez `sudo`
2. **Vérifiez l'interface** : le programme essaie automatiquement `eth0`, `enp0s3`, `wlan0`, `lo`
3. **Vérifiez la MAC destination** : elle doit correspondre à une machine accessible
4. **Testez d'abord avec `--dry_run`** : vérifiez que le paquet est bien construit dans le fichier PCAP
5. **Vérifiez les filtres Wireshark** : assurez-vous que le filtre n'exclut pas votre paquet

## Exemple : Commande complète avec tous les paramètres

```bash
sudo cargo run --release -- \
  --src_ip=192.168.100.10 \
  --dst_ip=192.168.100.1 \
  --src_mac=12:34:56:78:9A:BC \
  --dst_mac=DE:AD:BE:EF:CA:FE \
  --dest_port=9999 \
  --l4_protocol=udp \
  --timeout_ms=1000 \
  --ip_bitfield=0x00 \
  --debug_file=./test_capture.pcap \
  --debug_format=pcap
```

## Note importante

⚠️ Sur **Windows**, l'envoi réel via raw sockets ne fonctionnera pas (nécessite Linux/WSL2). Utilisez `--dry_run` pour générer le fichier PCAP et l'analyser dans Wireshark.

Pour Windows, utilisez WSL2 ou une VM Linux pour tester l'envoi réel.

