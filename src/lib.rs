use sha2::{Digest, Sha256};

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

pub fn master_private_key() {
    
}