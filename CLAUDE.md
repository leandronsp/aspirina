# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Aspirina is a neural network library in Rust. The core is matrix operations and layers with feedforward/backpropagation. Everything else in this repo are examples that exercise the library.

The `src/computer/` module is a sample project: a 4-bit neural CPU where all arithmetic/logic is performed by trained neural networks. It has been extracted into its own repo as [Synapse](https://github.com/leandronsp/synapse).

## Commands

```bash
cargo build              # Build
cargo run                # Interactive menu (gate training + computer tests)
cargo run --release      # Optimized build
cargo test               # All tests
cargo test matrix_test   # Single test file
cargo test -- --nocapture # Tests with stdout
cargo fmt                # Format
cargo clippy             # Lint
```

## Architecture

### Core Library (`src/lib.rs`)
The heart of Aspirina. Matrix operations and layers are the foundation. Everything else builds on top.
- **`matrix.rs`**. Matrix type with operator overloading (`+`, `-`, `*`), transpose, element-wise multiply (`naive_multiply`), and derivative computation.
- **`calc.rs`**. Sigmoid and tanh activation functions with derivatives.
- **`layer.rs`**. Single network layer: weight matrix + optional cached forward pass result.
- **`neural_network.rs`**. `NeuralNetwork` struct using `Rc<RefCell<Layer>>` for shared mutable layers. Forward/backward propagation, train, predict.

### Examples

#### Training Scenarios (`src/training/`)
Seven gate training scenarios (XOR, AND, OR, NAND, NOT, NOR, XNOR), each with a `run()` entry point. These demonstrate the library on classic logic gate problems.

#### Neural Computer (`src/computer/`)
Sample project: a 4-bit CPU built entirely from trained neural networks. Extracted to [Synapse](https://github.com/leandronsp/synapse).
- `gates.rs` (7 neural logic gates), `half_adder.rs`, `full_adder.rs`, `alu.rs` (4-bit ALU), `memory.rs` (16 x 4-bit cells), `registers.rs`, `cpu.rs` (fetch-decode-execute, 16 instructions), `assembler.rs`, `interpreter.rs`.

#### Binary (`src/main.rs`)
Interactive menu that runs training scenarios and computer component tests.

### Tests
Unit tests live in `tests/` directory: `matrix_test.rs`, `calc_test.rs`, `layer_test.rs`, `neural_network_test.rs`.

## Key Design Decisions

### Core
- **Weight matrix shape**: rows = neurons in current layer, columns = inputs. First layer is `hidden_size x input_size`.

### Computer Example (Synapse)
- **4-bit constraint**: All values masked with `& 0x0F`. Only 16 memory locations (0x0-0xF).
- **Memory layout**: Programs in low memory (0x0-0x8), temporaries in high memory (0xD-0xF). Overlapping causes silent corruption.
- **Two-byte instructions**: Each CPU instruction is opcode + operand. PC increments must account for this.
- **Operator precedence in interpreter**: Uses `rfind()` for left-to-right evaluation so `a - b + c` parses as `(a - b) + c`.
