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
    println!("12. Test Memory");
    println!("13. Test Registers");
    println!("14. Test CPU");
    println!("15. Test Assembler");
    println!("16. Test Interpreter");
    println!("0. Exit");
    println!();
    print!("Select a scenario (0-16): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "1" => aspirina_gates::training::xor_gate::run(),
        "2" => aspirina_gates::training::and_gate::run(),
        "3" => aspirina_gates::training::or_gate::run(),
        "4" => aspirina_gates::training::nand_gate::run(),
        "5" => aspirina_gates::training::not_gate::run(),
        "6" => aspirina_gates::training::nor_gate::run(),
        "7" => aspirina_gates::training::xnor_gate::run(),
        "8" => {
            println!("=== Neural Computer Logic Gates Test ===");
            aspirina_gates::computer::gates::test_all_gates();
        }
        "9" => {
            println!("=== Neural Computer Half Adder Test ===");
            aspirina_gates::computer::half_adder::test();
        }
        "10" => {
            println!("=== Neural Computer Full Adder Test ===");
            aspirina_gates::computer::full_adder::test();
        }
        "11" => {
            println!("=== Neural Computer 4-bit ALU Test ===");
            aspirina_gates::computer::alu::test();
        }
        "12" => {
            println!("=== Neural Computer Memory Test ===");
            aspirina_gates::computer::memory::test();
        }
        "13" => {
            println!("=== Neural Computer Registers Test ===");
            aspirina_gates::computer::registers::test();
        }
        "14" => {
            println!("=== Neural Computer CPU Test ===");
            aspirina_gates::computer::cpu::test();
        }
        "15" => {
            println!("=== Neural Computer Assembler Test ===");
            aspirina_gates::computer::assembler::test();
        }
        "16" => {
            println!("=== Neural Computer Interpreter Test ===");
            aspirina_gates::computer::interpreter::test();
        }
        "0" => println!("Goodbye!"),
        _ => println!("Invalid selection. Please run again and choose 0-16."),
    }
}
