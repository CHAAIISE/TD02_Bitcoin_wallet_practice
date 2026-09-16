use td2::{generate_seed, master_private_key, public_key, child_key};
use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut seed : Vec<String> = Vec::<String>::new();
    let mut b : bool = false;
    let mut word : String = String::new();
    let q : bool = true;
    let words: Vec<&str> = include_str!("../../mots.txt").lines().collect();
    println!("\n cli -h for commandes \n");
    while q { 
        let _test = std::io::stdin().read_line(&mut word);
        if word.trim() == "cli -g" {
            if b {
                println!("seed déjà enregistrée");
                println!("{}", seed.join(" "));
                println!("reset avant création d'une nouvelle");
            } else {
                println!("Generation d'une seed...\n\n");
                match generate_seed() {
                    Ok(phrase) => {
                        seed = phrase.clone();
                        b = true;
                        println!("Phrase générée : {}", phrase.join(" "));
                    }
                    Err(error) => {
                        eprintln!("Erreur de génération : {error}");
                    }
                }
            } 
        } else if word.trim() == "cli -h"{
            println!("cli -[parameter]\n
                       -h       help\n
                       -g       generate seed\n 
                       -i       import seed\n
                       -r       reset seed\n
                       -q / 'exit'     for exit\n
                       -bip32\n\n");

        } else if word.trim() == "cli -i"{
            if b {
                println!("seed déjà enregistrée");
                println!("{}", seed.join(" "));
                println!("reset avant d'en rentrée une nouvelle");
            } else {
                println!("Entrez vos mots séparé d'un espace : \n");
                let mut phrase : String = String::new();
                stdin().read_line(&mut phrase).expect("Erreur...");
                let mots : Vec<String> = phrase.split_whitespace().map(String::from).collect();
                println!("\nVoici la seed phrase : {}\n", mots.join(" "));
                if mots.len() != 12 {
                    println!("Seed invalide\n");
                } else if !(mots.iter().all(|mot| words.contains(&mot.as_str()))) {
                    println!("Seed invalide\n");
                } else {
                    seed = mots.clone();
                    b = true;
                    println!("Seed valide\n");
                }
            }
        } else if word.trim() == "cli -r" {
            if !b {
                println!("seed pas enregistrée");
                println!("veuillez en creer ou en importer une");
            } else {
                seed.clear();
                b = false;
                println!("seed reset\n");
            }
        } else if word.trim() == "exit" || word.trim() == "cli -q"{
            break
        } else if word.trim() == "cli -bip32" {
            if !b {
                println!("seed pas enregistrée");
                println!("veuillez en creer ou en importer une avant d'utiliser la partie bip32");
            } else {
                println!("                1 - Extract the master private key and the chain code\n
                2 - Extract the master public key\n
                3 - Generate a child key\n
                4 - Generate a child key at index1 N : d \n
                5 - Generate a child key at index1 N at derivation level M\n
                6 - Quit bip32\n
                Entrer votre choix : \n");
                while q {
                    let mut word2 : String = String::new();
                    let _test2 = std::io::stdin().read_line(&mut word2);
                    if word2.trim() == "6" {
                        println!("cli -h for commands\n");
                        break;
                    } else if word2.trim() == "1" {
                        let (private_key, chain_code) = master_private_key(&seed, "")?;               
                        print!("Master private key : ");
                        for byte in &private_key {
                            print!("{byte:02x}");
                        }
                        println!();
                        print!("Chain Code : ");
                        for byte in &chain_code {
                            print!("{byte:02x}");
                        }
                        println!();
                    } else if word2.trim() == "2" {
                        let (private_key, _chain_code) = master_private_key(&seed, "")?;               
                        let public_key = public_key(private_key)?;
                        print!("Master public key : ");
                        for byte in &public_key {
                            print!("{byte:02x}");
                        }
                        println!();
                    } else if word2.trim() == "3" {
                        let (private_key, chain_code) = master_private_key(&seed, "")?;
                        let (child_private_key, child_chain_code) = child_key(private_key, chain_code, 0u32.to_be_bytes())?;
                        let child_public_key = public_key(child_private_key)?;
                        print!("Child private key : ");
                        for byte in &child_private_key {
                            print!("{byte:02x}");
                        }
                        println!();
                        print!("Child public key : ");
                        for byte in &child_public_key {
                            print!("{byte:02x}");
                        }
                        println!();
                        print!("Child chain code : ");
                        for byte in &child_chain_code {
                            print!("{byte:02x}");
                        }
                        println!();
                    } else if word2.trim() == "4" {
                        println!("Entrée l'index souhaité : ");
                        let mut saisie = String::new();
                        stdin().read_line(&mut saisie)?;
                        let n: u32 = saisie.trim().parse()?;
                        if n >= (1u32 << 31) {
                            println!("L’index doit être compris entre 0 et 2 147 483 647.");
                            continue;
                        }
                        let index: [u8; 4] = n.to_be_bytes();
                        let (private_key, chain_code) = master_private_key(&seed, "")?;
                        let (child_private_key, child_chain_code) = child_key(private_key, chain_code, index)?;
                        let child_public_key = public_key(child_private_key)?;
                        print!("Child private key d'index {} : ", n);
                        for byte in &child_private_key {
                            print!("{byte:02x}");
                        }
                        println!();
                        print!("Child public key d'index {} : ", n);
                        for byte in &child_public_key {
                            print!("{byte:02x}");
                        }
                        println!();
                        print!("Child chain code d'index {} : ", n);
                        for byte in &child_chain_code {
                            print!("{byte:02x}");
                        }
                        println!();
                    }  else if word2.trim() == "5" {
                        println!("Entrer l'index souhaité : ");
                        let mut saisie = String::new();
                        stdin().read_line(&mut saisie)?;
                        let n: u32 = saisie.trim().parse()?;
                        if n >= (1u32 << 31) {
                            println!("L’index doit être compris entre 0 et 2 147 483 647.");
                            continue;
                        }
                        println!("Entrer la dérivation souhaité : ");
                        let mut saisie2 = String::new();
                        stdin().read_line(&mut saisie2)?;
                        let m: u32 = saisie2.trim().parse()?;
                        if m >= (1u32 << 31) {
                            println!("La dérivation doit être compris entre 0 et 2 147 483 647.");
                            continue;
                        }
                        let index1: [u8; 4] = n.to_be_bytes();
                        let index2: [u8; 4] = m.to_be_bytes();
                        let (private_key, chain_code) = master_private_key(&seed, "")?;
                        let (child_private_key, child_chain_code) = child_key(private_key, chain_code, index1)?;
                        let (child_private_key2, child_chain_code2) = child_key(child_private_key, child_chain_code, index2)?;
                        let child_public_key2 = public_key(child_private_key2)?;
                        print!("Child private key d'index {} et de dérivation {} : ", n, m);
                        for byte in &child_private_key2 {
                            print!("{byte:02x}");
                        }
                        println!();
                        print!("Child public key d'index {} et de dérivation {} : ", n, m);
                        for byte in &child_public_key2 {
                            print!("{byte:02x}");
                        }
                        println!();
                        print!("Child chain code d'index {} et de dérivation {} : ", n, m);
                        for byte in &child_chain_code2 {
                            print!("{byte:02x}");
                        }
                        println!();
                    } else {
                        println!("erreur\n");
                    }
                    word2.clear();
                }
            }
        } else {
            println!("error... \n type 'cli -h' for help\n\n");
        }
        word.clear();
    }
    println!("MERCI AU REVOIR");
    Ok(())
}
