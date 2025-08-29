mod training {
    pub mod xor_gate;
    pub mod and_gate;
}

use std::io::{self, Write};

fn main() {
    println!("Aspirina Neural Network Library");
    println!("==============================");
    println!();
    println!("Available training scenarios:");
    println!("1. XOR Gate");
    println!("2. AND Gate");
    println!("0. Exit");
    println!();
    print!("Select a scenario (0-2): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    match input.trim() {
        "1" => training::xor_gate::run(),
        "2" => training::and_gate::run(),
        "0" => println!("Goodbye!"),
        _ => println!("Invalid selection. Please run again and choose 0-2."),
    }
}
