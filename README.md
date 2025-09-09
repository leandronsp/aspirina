# aspirina

```
     ___           _______..______    __  .______       __  .__   __.      ___      
    /   \         /       ||   _  \  |  | |   _  \     |  | |  \ |  |     /   \     
   /  ^  \       |   (----`|  |_)  | |  | |  |_)  |    |  | |   \|  |    /  ^  \    
  /  /_\  \       \   \    |   ___/  |  | |      /     |  | |  . `  |   /  /_\  \   
 /  _____  \  .----)   |   |  |      |  | |  |\  \----.|  | |  |\   |  /  _____  \  
/__/     \__\ |_______/    | _|      |__| | _| `._____||__| |__| \__| /__/     \__\ 
```

A neural network library written in Rust.
Inspired by [leandronsp/morphine](https://github.com/leandronsp/morphine).

## Requirements

* Rust
* Cargo

## Development Commands

### Build the project
```bash
cargo build
```

### Run the CLI menu for training samples
```bash
cargo run
```

### Run all tests
```bash
cargo test
```

### Format code
```bash
cargo fmt
```

### Lint code
```bash
cargo clippy
```

## Training Samples

Run `cargo run` to access an interactive menu with logic gate training scenarios and computer component tests.

### Logic Gates (Training Scenarios 1-7)
Each gate is trained with 10,000 epochs to learn boolean logic operations:

| Gate | Truth Table | Description |
|------|------------|-------------|
| XOR  | 0⊕0=0, 0⊕1=1, 1⊕0=1, 1⊕1=0 | Outputs 1 when inputs differ |
| AND  | 0∧0=0, 0∧1=0, 1∧0=0, 1∧1=1 | Outputs 1 when both inputs are 1 |
| OR   | 0∨0=0, 0∨1=1, 1∨0=1, 1∨1=1 | Outputs 1 when at least one input is 1 |
| NAND | ¬(A∧B) | Inverted AND |
| NOT  | ¬0=1, ¬1=0 | Inverts single input |
| NOR  | ¬(A∨B) | Inverted OR |
| XNOR | ¬(A⊕B) | Outputs 1 when inputs are equal |

### Computer Components (Tests 8-16)
Neural networks trained as logic gates are combined to build complete computer architecture:

**Test 8: All Logic Gates** - Validates that logic gates achieve correct truth tables  
**Test 9: Half Adder** - Adds two bits producing sum (XOR) and carry (AND)  
**Test 10: Full Adder** - Adds three bits using two half adders  
**Test 11: 4-bit ALU** - Performs Add/Subtract/AND/OR/XOR on 4-bit numbers  
**Test 12: Memory** - 16 words of 4-bit memory with read/write operations  
**Test 13: Registers** - Accumulator, Program Counter, Instruction Register with flags  
**Test 14: CPU** - Complete 4-bit CPU with 16-instruction ISA and fetch-decode-execute cycle  
**Test 15: Assembler** - Assembly language to machine code converter with disassembly  
**Test 16: Interpreter** - High-level language that compiles to neural CPU operations

## Neural Computer Architecture

Aspirina implements a hypothetical 4-bit computer built entirely from neural networks trained to behave as logic gates. Starting from basic boolean operations, we construct increasingly complex components following traditional computer architecture principles.

The foundation consists of neural networks trained to mimic the 7 fundamental logic gates. Each network learns through backpropagation to produce the correct boolean outputs for its truth table. These gates combine to form arithmetic circuits: half adders compute single-bit addition with carry, full adders chain together for multi-bit arithmetic, and finally a 4-bit ALU performs complete arithmetic and logic operations.

The architecture follows Von Neumann principles with 16 words of 4-bit memory, registers for accumulator and program counter, and a complete instruction set with 16 operations including LOAD, STORE, ADD, SUB, AND, OR, XOR, and JUMP. The CPU implements a fetch-decode-execute cycle, reading instructions from memory, decoding them, and executing operations using the neural ALU. An assembler converts assembly language to machine code, and an interpreter executes high-level language statements directly on the neural computer.

This demonstrates that neural networks can learn to perform deterministic computation, essentially building a computer from learned logic rather than fixed silicon gates. Each component maintains the same training approach: 10,000 epochs of backpropagation to achieve near-perfect accuracy on boolean operations.

## Project Structure

```
src/
├── main.rs           # Interactive menu for training and testing
├── lib.rs            # Module declarations
├── calc.rs           # Activation functions (sigmoid, tanh)
├── matrix.rs         # Matrix operations with operator overloading
├── layer.rs          # Neural network layer
├── neural_network.rs # Core neural network with backpropagation
├── training/         # Logic gate training scenarios
│   ├── and_gate.rs
│   ├── nand_gate.rs
│   ├── nor_gate.rs
│   ├── not_gate.rs
│   ├── or_gate.rs
│   ├── xnor_gate.rs
│   └── xor_gate.rs
└── computer/         # Neural computer components
    ├── mod.rs
    ├── gates.rs      # All 7 logic gates consolidated
    ├── half_adder.rs # XOR + AND gates
    ├── full_adder.rs # 2 half adders + OR gate
    ├── alu.rs        # 4-bit ALU with arithmetic/logic ops
    ├── memory.rs     # 16 x 4-bit memory cells
    ├── registers.rs  # CPU registers and flags
    ├── cpu.rs        # Complete CPU with instruction set
    ├── assembler.rs  # Assembly to machine code converter
    └── interpreter.rs # High-level language interpreter

tests/
├── calc_test.rs
├── matrix_test.rs
├── layer_test.rs
└── neural_network_test.rs
```

----

[ASCII art generator](http://patorjk.com/software/taag/#p=display&f=Graffiti&t=Type%20Something%20)
