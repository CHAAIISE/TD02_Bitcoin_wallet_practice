use sha2::{Digest, Sha256, Sha512};
use pbkdf2::pbkdf2_hmac;
use unicode_normalization::UnicodeNormalization;
use hmac::{Hmac, KeyInit, Mac};
use secp256k1::SecretKey;


pub fn generate_seed() -> Result<Vec<String>, getrandom::Error> {
    let entropy = get_random_u128()?;
    let bytes = u128_to_bytes(entropy);
    let binary = u128_to_binary(entropy);

    println!("Entier : {}", entropy);
    println!("Octets : {:?}", bytes);
    println!("Binaire : {}", binary);
    println!("Hex : {}", u128_to_hex(entropy));

    let hash = Sha256::digest(bytes);
    let checksum = hash[0] >> 4;
    let binary_and_checksum = format!("{binary}{checksum:04b}");

    println!("Checksum : {checksum:04b}");

    let mut mots : Vec<String> = Vec::new();
    let words: Vec<&str> = include_str!("../mots.txt").lines().collect();

    for (i, group) in split(&binary_and_checksum).enumerate() {
        let text = std::str::from_utf8(group).expect("Erreur");
        println!("Groupe {} : {}", i + 1, text);
        let index = binary_to_u16(text) as usize;
        mots.push(words[index].to_string());
    }

    println!("Voici la seed phrase : {}\n", mots.join(" "));

    Ok(mots)
}

fn binary_to_u16(bin: &str) -> u16 {
    u16::from_str_radix(bin, 2).expect("Binaire invalide")
}

fn split(binary: &str) -> std::slice::Chunks<'_, u8> {
    binary.as_bytes().chunks(11)
}

fn u128_to_hex(num : u128) -> String {
    format!("{:032x}", num)
}

fn u128_to_binary(num: u128) -> String {
    format!("{:0128b}", num)
}

fn u128_to_bytes(num: u128) -> [u8; 16] {
    num.to_be_bytes()
}

fn get_random_u128() -> Result<u128, getrandom::Error> {
    let mut buf = [0u8; 16];
    getrandom::fill(&mut buf)?;
    Ok(u128::from_ne_bytes(buf))
}

pub fn master_private_key(mots: &[String], passphrase: &str) -> Result<([u8; 32], [u8; 32]), secp256k1::Error>{
    let phrase : String = mots.join(" ").nfkd().collect::<String>();
    let salt: String = format!("mnemonic{passphrase}").nfkd().collect();
    let mut seed = [0u8; 64];
    pbkdf2_hmac::<Sha512>(phrase.as_bytes(), salt.as_bytes(), 2048, &mut seed);
    let mut mac = Hmac::<Sha512>::new_from_slice(b"Bitcoin seed").expect("HMAC accepte cette clé");

    mac.update(&seed);
    let result = mac.finalize().into_bytes();

    let private_key: [u8; 32] = result[..32].try_into().expect("32 octets");
    let chain_code: [u8; 32] = result[32..].try_into().expect("32 octets");

    let _ = SecretKey::from_secret_bytes(private_key)?;

    Ok((private_key, chain_code))
}

pub fn public_key(private_key : [u8;32]) -> Result<[u8; 33], secp256k1::Error> {
    let key = secp256k1::SecretKey::from_secret_bytes(private_key)?;
    Ok(key.public_key().serialize())
}

pub fn child_key(master_private_key : [u8;32], chain_code : [u8; 32], index : [u8; 4]) -> Result<([u8; 32], [u8; 32]), secp256k1::Error> {
    let master_public_key = public_key(master_private_key)?;
    let mut mac = Hmac::<Sha512>::new_from_slice(&chain_code).expect("HMAC accepte cette clé");
    mac.update(&master_public_key);
    mac.update(&index);
    let result = mac.finalize().into_bytes();
    let left : [u8; 32] = result[..32].try_into().expect("32 octets"); 
    let child_chain_code : [u8; 32] = result[32..].try_into().expect("32 octets");
    let decalage = secp256k1::Scalar::from_be_bytes(left).map_err(|_| secp256k1::Error::InvalidTweak)?;
    let parent = SecretKey::from_secret_bytes(master_private_key)?;
    let child_private_key =  parent.add_tweak(&decalage)?.to_secret_bytes();
    let _ = SecretKey::from_secret_bytes(child_private_key)?;
    Ok((child_private_key, child_chain_code))
}