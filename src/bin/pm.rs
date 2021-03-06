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
        let packages = identify_packages(&args);
        install::install(flags, packages);
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

fn identify_packages(args: &Vec<String>) -> Vec<String> {
    let mut return_vector: Vec<String> = Vec::new();
    let mut command = false;

    for arg in &args[1..] {
        if &arg[0..1] != "-" {
            if command {
                return_vector.push(arg.clone());
            } else {
                command = true;
            }
        }
    }

    return_vector
}