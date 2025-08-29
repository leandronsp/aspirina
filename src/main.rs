mod training {
    pub mod and_gate;
    pub mod nand_gate;
    pub mod nor_gate;
    pub mod not_gate;
    pub mod or_gate;
    pub mod xnor_gate;
    pub mod xor_gate;
}

use aspirina::computer::alu;
use aspirina::computer::full_adder;
use aspirina::computer::gates;
use aspirina::computer::half_adder;
use std::io::{self, Write};

fn main() {
    println!("Aspirina Neural Network Library");
    println!("==============================");
    println!();
    println!("Available scenarios:");
    println!("=== Training Scenarios ===");
    println!("1. XOR Gate");
    println!("2. AND Gate");
    println!("3. OR Gate");
    println!("4. NAND Gate");
    println!("5. NOT Gate");
    println!("6. NOR Gate");
    println!("7. XNOR Gate");
    println!("=== Computer Components ===");
    println!("8. Test All Logic Gates");
    println!("9. Test Half Adder");
    println!("10. Test Full Adder");
    println!("11. Test 4-bit ALU");
    println!("0. Exit");
    println!();
    print!("Select a scenario (0-11): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => training::xor_gate::run(),
        "2" => training::and_gate::run(),
        "3" => training::or_gate::run(),
        "4" => training::nand_gate::run(),
        "5" => training::not_gate::run(),
        "6" => training::nor_gate::run(),
        "7" => training::xnor_gate::run(),
        "8" => {
            println!("=== Neural Computer Logic Gates Test ===");
            gates::test_all_gates();
        }
        "9" => {
            println!("=== Neural Computer Half Adder Test ===");
            half_adder::HalfAdder::new().test();
        }
        "10" => {
            println!("=== Neural Computer Full Adder Test ===");
            full_adder::FullAdder::new().test();
        }
        "11" => {
            println!("=== Neural Computer 4-bit ALU Test ===");
            alu::ALU::new().test();
        }
        "0" => println!("Goodbye!"),
        _ => println!("Invalid selection. Please run again and choose 0-11."),
    }
}
