# TD02_Bitcoin_wallet_practice

TD2 de Blockchain Programming : un wallet Bitcoin en ligne de commande, en Rust, avec BIP 39 et BIP 32.

## Lancer le programme

Il faut Rust et cargo.

```
cargo run --bin cli
```

Commandes du CLI :

```
cli -h          aide
cli -g          générer une seed
cli -i          importer une seed
cli -r          supprimer la seed enregistrée
cli -bip32      menu BIP 32 (il faut avoir généré ou importé une seed avant)
cli -q / exit   quitter
```

Dans le menu BIP 32 on tape le numéro de l'option (1 à 6).

## Rapport

Les fonctions sont dans `src/lib.rs` et la boucle du CLI dans `src/bin/cli.rs`.

Librairies : `getrandom` pour l'aléatoire, `sha2`, `hmac` et `pbkdf2` pour les hash, `unicode-normalization` pour la normalisation NFKD, et `secp256k1` seulement pour passer d'une clé privée à la clé publique et pour l'addition des clés privées modulo n. Je n'ai pas utilisé de librairie Bitcoin ou BIP 32, la dérivation est faite à la main.

### 1. CLI BIP 39

Nombre aléatoire : `cli -g` tire 16 octets (128 bits) avec `getrandom`, qui utilise le générateur aléatoire du système d'exploitation. C'est un générateur fait pour la crypto, on ne peut pas prévoir le résultat.

Binaire, octets, hex et lots de 11 bits : le programme affiche l'entier, les 16 octets, le binaire sur 128 bits et l'hex. Ensuite il ajoute le checksum, qui correspond aux 4 premiers bits du SHA-256 des 16 octets (128 / 32 = 4). Ça fait 132 bits, donc 12 lots de 11 bits qui sont affichés un par un.

Mnémonique : chaque lot de 11 bits est un nombre entre 0 et 2047, c'est l'index du mot dans la liste BIP 39 anglaise (`mots.txt`, 2048 mots). Les 12 mots forment la seed.

Import : `cli -i` puis on colle les mots séparés par des espaces. Le programme vérifie :
- qu'il y a 12, 15, 18, 21 ou 24 mots
- que chaque mot est dans la liste
- le checksum : chaque mot est remis en 11 bits, on sépare l'entropie et le checksum (1 bit de checksum pour 32 bits d'entropie, donc 4 bits pour 12 mots et 8 bits pour 24 mots), on recalcule le SHA-256 de l'entropie et on compare.

Exemple avec le premier vecteur de test de BIP 39 (11 fois `abandon` puis `about`) :

```
Entropie : 00000000000000000000000000000000
Checksum de la phrase : 0011
Checksum attendu : 0011
Seed valide
```

Avec 12 fois `abandon`, le checksum de la phrase vaut 0000 au lieu de 0011 donc la seed est refusée.

### 2. BIP 32

Seed : la phrase (normalisée en NFKD) passe dans PBKDF2-HMAC-SHA512 avec 2048 itérations et le sel "mnemonic" + passphrase (passphrase vide ici). On obtient une seed de 64 octets.

1 - Master private key et chain code : HMAC-SHA512 de la seed avec la clé "Bitcoin seed". Les 32 premiers octets donnent la master private key et les 32 derniers le chain code. Je vérifie que la clé est valide (entre 1 et n - 1).

2 - Master public key : clé privée * G sur la courbe secp256k1, en format compressé sur 33 octets (02 ou 03 selon la parité de y, puis x).

3 - Clé enfant : dérivation normale (pas hardened) à l'index 0 :
- HMAC-SHA512 avec le chain code du parent comme clé, sur la clé publique du parent (33 octets) suivie de l'index (4 octets en big endian)
- les 32 premiers octets sont ajoutés à la clé privée du parent modulo n, ça donne la clé privée enfant
- les 32 derniers octets sont le chain code enfant

4 - Clé enfant à l'index N : pareil avec l'index N. N va de 0 à 2^31 - 1, au-dessus ce sont les index hardened que je n'ai pas faits.

5 - Clé enfant à l'index N au niveau M : je refais la dérivation M fois avec l'index N, donc le chemin est m/N/N/.../N avec M fois N. Le programme affiche le chemin. Par exemple N = 5 et M = 3 donne m/5/5/5, et avec M = 1 on retrouve la clé de l'option 4. M va de 1 à 255 parce que la profondeur est codée sur 1 octet dans BIP 32.

### 3. Vérification sur https://iancoleman.io/bip39/

J'ai collé la phrase dans le site, sans passphrase, onglet BIP32 avec "Custom derivation path".

Avec le vecteur de test (11 fois `abandon` puis `about`) le CLI donne :

```
Master private key : 1837c1be8e2995ec11cda2b066151be2cfb48adf9e47b151d46adab3a21cdf67
Chain Code : 7923408dadd3c7b56eed15567707ae5e5dca089de972e07f3b860450e2a3b70e
Master public key : 03d902f35f560e0470c63313c7369168d9d7df2d49bf295fd9fb7cb109ccee0494
Child public key : 0376bf533d4b15510fa9f4124b6e48616f07debcf2ef0cfb185cdc4a576450b475
Child public key d'index 5 : 02a82f6cfe7a2eb95ac2bfee0c8c739c185406761b6b7f4652359f2fddc1595861
Chemin : m/5/5/5
Child public key d'index 5 au niveau 3 : 0278f83f1ee1b5f8b9fcc422592ce0bfbf9a47152bed4818b0e2a4df2a17772f54
```

Sur le site :
- la "BIP32 Root Key" contient la même master private key et le même chain code (je l'ai décodée en base58 pour comparer)
- la xpub du chemin `m` contient la même master public key
- options 3 et 4 : chemin `m`, lignes m/0 et m/5, mêmes clés publiques
- option 5 : chemin `m/5/5`, ligne m/5/5/5, même clé publique

Avec une phrase générée par `cli -g` (`also deal paper lion stuff north equip enroll submit guide vanish theory`), le site accepte la phrase et retrouve la même entropie (`0747067fc11d792c930a59d80cf3c570`), la même master private key, le même chain code et la même clé publique m/0.
