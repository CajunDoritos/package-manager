use std::env;

enum Commands {
    Invalid,
    Help,
    Install,
    Remove
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = identify_command(&args);
    if let Commands::Install = command {
        println!("install");
    }
    println!("{:?}", args);
}

fn identify_command(args: &Vec<String>) -> Commands {
    for arg in &args[1..] {
        if &arg[0..1] != "-" {
            match arg.to_lowercase().as_str() {
                "help" => return Commands::Help,
                "h" => return Commands::Help,
                "install" => return Commands::Install,
                "i" => return Commands::Install,
                "remove" => return Commands::Remove,
                "r" => return Commands::Remove,
                _ => Commands::Invalid
            };
        }
    }
    Commands::Invalid
}