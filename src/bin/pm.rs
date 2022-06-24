use std::env;

enum Commands {
    Invalid,
    Help,
    Install,
    Remove
}

enum Flags {
    Invalid,
    Yes
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = identify_command(&args);
    let flags = identify_flags(&args);
    
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

fn identify_flags(args: &Vec<String>) -> Vec<Flags> {
    let mut return_vector: Vec<Flags> = Vec::new();

    for arg in &args[1..] {
        if &arg[0..1] == "-" {
            match arg[1..].to_lowercase().as_str() {
                "y" => return_vector.push(Flags::Yes),
                _ => return_vector.push(Flags::Invalid)
            }
        }
    }
    return_vector
}