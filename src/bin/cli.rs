use TD2::generate_seed;
use std::io::stdin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut word : String = String::new();
    let words: Vec<&str> = include_str!("../../mots.txt").lines().collect();
    while word.trim() != "exit" { 
        let _test = std::io::stdin().read_line(&mut word);
        if word.trim() == "cli -g" {
            println!("Generation d'une seed...\n\n");
            if let Err(error) = generate_seed() {
                eprintln!("Erreur de génération : {error}\n");
            }
        } else if word.trim() == "cli -h"{
            println!("cli -[parameter]\n
                       -h       help\n
                       -g       generate seed\n 
                       -i       import seed\n
                       -q / 'exit'     for exit\n\n");

        } else if word.trim() == "cli -i"{
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
                println!("Seed valide\n");
            }
        } else if word.trim() == "exit" || word.trim() == "cli quit"{
            break
        } else {
            println!("error... \n type 'cli -h' for help\n\n");
        }
        word.clear()
    }
    println!("MERCI AU REVOIR");
    Ok(())
}
