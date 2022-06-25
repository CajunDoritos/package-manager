use crate::types;

pub fn install(flags: Vec<types::Flags>, packages: Vec<String>) {
    print!("Using flags: ");
    for flag in flags {
        if let types::Flags::Yes = flag {
            print!("-y");
        }
        if let types::Flags::Invalid = flag {
            print!("invalid flag");
        }
        print!(" ");
    }
    println!();
    println!("Packages:");
    for package in packages {
        println!("{}", package);
    }
}
