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

## Setup & Requirements

### Install Rust
If you don't have Rust installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Verify Installation
```bash
rustc --version
cargo --version
```

## Development Commands

### Build the project
```bash
cargo build
```

### Run the neural network training
```bash
cargo run
```

### Run all tests
```bash
cargo test
```

### Run tests for a specific module
```bash
cargo test calc_test
cargo test matrix_test
cargo test layer_test
cargo test neural_network_test
```

### Format code
```bash
cargo fmt
```

### Lint code
```bash
cargo clippy
```

## What the Neural Network Does

The main program (`cargo run`) demonstrates a simple feedforward neural network learning a **XOR gate** - a classic benchmark problem for neural networks.

### What is a XOR Gate?
XOR (exclusive OR) is a logical operation that outputs:
- **1** when inputs are different (0,1 or 1,0)
- **0** when inputs are the same (0,0 or 1,1)

| Input A | Input B | XOR Output |
|---------|---------|------------|
|    0    |    0    |     0      |
|    0    |    1    |     1      |
|    1    |    0    |     1      |
|    1    |    1    |     0      |

The XOR problem is significant because it cannot be solved by a simple linear classifier - it requires a neural network with at least one hidden layer.

### Architecture
- **Input Layer**: 3 neurons (accepts 3D input vectors)
- **Hidden Layers**: 2 hidden layers with 4 neurons each
- **Output Layer**: 1 neuron (binary classification)
- **Activation**: Sigmoid function throughout

### Training Data
The network learns from 8 training examples with 3 features each:
```
Input:  [0.0, 0.0, 1.0] → Target: 0.0
Input:  [0.0, 0.0, 0.0] → Target: 0.0  
Input:  [0.0, 1.0, 1.0] → Target: 1.0
Input:  [0.0, 1.0, 0.0] → Target: 1.0
Input:  [1.0, 0.0, 1.0] → Target: 1.0
Input:  [1.0, 0.0, 0.0] → Target: 1.0
Input:  [0.6, 0.6, 0.0] → Target: 0.0
Input:  [0.6, 0.6, 1.0] → Target: 0.0
```

### Learning Process
- Trains for 100,000 iterations using backpropagation
- Adjusts weights to minimize prediction error
- Shows training progress with iteration numbers

### Verification
After training, the network makes a prediction on `[1.0, 1.0, 0.0]`. 
This implements a **XOR gate** on the first two inputs - the network learns to output 1.0 when exactly one of the first two inputs is 1.0, and 0.0 when both are the same (both 0 or both 1).

For the test case `[1.0, 1.0, 0.0]`, the expected output should be close to **0.0** (since both inputs are 1, XOR gives 0).

The closer the final prediction is to 0.0, the better the network has learned the XOR pattern!

## Project Structure

```
src/
├── main.rs           # Main training loop and demo
├── lib.rs            # Module declarations
├── calc.rs           # Activation functions (sigmoid, tanh)
├── matrix.rs         # Matrix operations
├── layer.rs          # Neural network layer
└── neural_network.rs # Complete neural network implementation

tests/
├── calc_test.rs           # Tests for activation functions
├── matrix_test.rs         # Tests for matrix operations
├── layer_test.rs          # Tests for layer functionality
└── neural_network_test.rs # Tests for neural network
```

----

[ASCII art generator](http://patorjk.com/software/taag/#p=display&f=Graffiti&t=Type%20Something%20)