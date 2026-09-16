fn main() {
    let mut world : String = String::new();
    while world.trim() != "exit" { 
        let _test = std::io::stdin().read_line(&mut world);
        if world.trim() == "cli -t" {
            println!("Ceci est un test\n\n");
        } else if world.trim() == "cli -h"{
            println!("cli -[parameter]\n       -h       help\n       -t       test\n 'cli quit' or 'exit'     for exit\n\n");
        } else if world.trim() == "exit" || world.trim() == "cli quit"{
            break
        } else {
            println!("error... \n type 'cli -h' for help\n\n");
        }
        world.clear()
    }
    println!("MERCI AU REVOIR");
}
