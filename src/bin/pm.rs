use std::env;

use libpm::{
    types,
    install
};

enum Commands {
    Invalid,
    Help,
    Install,
    Remove
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = identify_command(&args);
    let flags = identify_flags(&args);

    if let Commands::Install = command {
        install::install(flags);
    }

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
                _ => break
            }
        }
    }
    Commands::Invalid
}

fn identify_flags(args: &Vec<String>) -> Vec<types::Flags> {
    let mut return_vector: Vec<types::Flags> = Vec::new();

    for arg in &args[1..] {
        if &arg[0..1] == "-" {
            match arg[1..].to_lowercase().as_str() {
                "y" => return_vector.push(types::Flags::Yes),
                _ => return_vector.push(types::Flags::Invalid)
            }
        }
    }
    return_vector
}