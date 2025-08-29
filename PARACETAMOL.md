# PARACETAMOL Plan - Updated Progress

## Overview
Building a Neural Computer using ANNs trained to behave like logic gates, eventually creating a simple CPU that can execute instructions. Later, integrate with VoxQuad for music pattern recognition.

## Phase 1: Logic Gates Foundation ✅ COMPLETED
### Achievements:
- ✅ All 7 basic logic gates implemented (AND, OR, NOT, NAND, NOR, XOR, XNOR)
- ✅ Each gate trained with 10,000 epochs
- ✅ Separate training modules in `src/training/`
- ✅ Consolidated computer gates in `src/computer/gates.rs`
- ✅ Interactive testing menu in main.rs

## Phase 2: Arithmetic Components ✅ COMPLETED
### Achievements:
- ✅ **Half Adder**: XOR gate (sum) + AND gate (carry)
- ✅ **Full Adder**: Built from 2 Half Adders + OR gate
- ✅ **4-bit ALU**: 
  - Addition using 4 chained Full Adders
  - Subtraction using two's complement
  - Bitwise AND, OR, XOR operations
  - Carry and Zero flags

## Phase 3: Memory & CPU Architecture 🚧 IN PROGRESS

### 3.1 Memory Components (Next Steps)
```rust
// 4-bit Memory Cell using neural network to store state
struct MemoryCell {
    data: u8,  // 4-bit value
    write_enable: LogicGate,  // Control writing
}

// 16 x 4-bit Memory Bank
struct Memory {
    cells: [MemoryCell; 16],
}
```

### 3.2 Register Components
```rust
struct Register4Bit {
    value: u8,
    load_enable: LogicGate,
}

struct CPURegisters {
    accumulator: Register4Bit,  // A register
    program_counter: Register4Bit,  // PC
    instruction_register: Register4Bit,  // IR
}
```

### 3.3 Instruction Set Architecture (4-bit)
```rust
enum Instruction {
    NOP,        // 0x0: No operation
    LOAD(u8),   // 0x1: Load memory[addr] into A
    STORE(u8),  // 0x2: Store A into memory[addr]
    ADD(u8),    // 0x3: Add memory[addr] to A
    SUB(u8),    // 0x4: Subtract memory[addr] from A
    AND(u8),    // 0x5: AND memory[addr] with A
    OR(u8),     // 0x6: OR memory[addr] with A
    XOR(u8),    // 0x7: XOR memory[addr] with A
    JUMP(u8),   // 0x8: Set PC to addr
    JZ(u8),     // 0x9: Jump if zero flag set
    HALT,       // 0xF: Stop execution
}
```

### 3.4 CPU Implementation
```rust
struct SimpleCPU {
    registers: CPURegisters,
    memory: Memory,
    alu: ALU,
    halted: bool,
}

impl SimpleCPU {
    fn fetch(&mut self) -> u8 {
        // Read instruction from memory[PC]
    }
    
    fn decode(&self, instruction: u8) -> Instruction {
        // Decode 4-bit instruction
    }
    
    fn execute(&mut self, instruction: Instruction) {
        // Execute using ALU and update registers/memory
    }
    
    fn cycle(&mut self) {
        let instruction = self.fetch();
        let decoded = self.decode(instruction);
        self.execute(decoded);
        self.registers.program_counter.increment();
    }
}
```

## Phase 4: Programming the Neural Computer

### 4.1 Simple Assembler
Create an assembler to convert human-readable programs to machine code:
```
; Program to add two numbers
LOAD 0x0    ; Load memory[0] into A
ADD 0x1     ; Add memory[1] to A  
STORE 0x2   ; Store result in memory[2]
HALT        ; Stop
```

### 4.2 Test Programs
1. **Addition**: Add two numbers
2. **Counter**: Increment a value in loop
3. **Logic Test**: Test AND/OR/XOR operations
4. **Conditional**: Jump based on zero flag

## Phase 5: Extended Architecture (Future)

### 5.1 8-bit Expansion
- Expand ALU to 8-bit operations
- Larger memory space (256 bytes)
- More complex instructions (MUL, DIV, SHIFT)
- Stack operations (PUSH, POP)

### 5.2 Advanced Features
- Interrupts
- I/O operations
- Multiple registers
- Indirect addressing

## Phase 6: VoxQuad Music Integration (After Computer is Complete)

### 6.1 Music Pattern Recognition
```rust
// Convert VoxQuad chord progressions to neural training data
struct ChordProgression {
    chords: Vec<u8>,  // Numeric chord encoding
    key: u8,
    genre: u8,
}

struct MusicPredictor {
    network: NeuralNetwork,
}
```

### 6.2 Chord Encoding System
Map VoxQuad's chord names to numeric values:
- C Major = 0x00
- C# Major = 0x01
- D Major = 0x02
- etc...

### 6.3 Training Datasets
Convert common progressions to training data:
- I-V-vi-IV → [0, 7, 9, 5]
- ii-V-I → [2, 7, 0]
- Blues: I-I-I-I-IV-IV-I-I-V-IV-I-V

### 6.4 REST API Integration
```rust
// Axum server endpoints
POST /api/computer/execute  // Run CPU program
POST /api/music/predict     // Get chord suggestions
```

## Current Implementation Status

### Completed ✅
- All 7 logic gates with neural networks
- Half Adder (XOR + AND)
- Full Adder (2 Half Adders + OR)
- 4-bit ALU (Add, Sub, And, Or, Xor)
- Interactive testing menu

### In Progress 🚧
- Memory components
- Register implementation
- CPU architecture

### Todo 📝
- Instruction decoder
- Fetch-decode-execute cycle
- Simple assembler
- Test programs
- 8-bit expansion
- VoxQuad integration

## Next Immediate Steps
1. Create `memory.rs` with 16 x 4-bit memory cells
2. Create `registers.rs` with A, PC, IR registers
3. Create `cpu.rs` with fetch-decode-execute cycle
4. Create `assembler.rs` to convert assembly to machine code
5. Test with simple programs (add two numbers, counter)
6. Once working, expand to 8-bit architecture
7. Finally, integrate VoxQuad music pattern recognition

## Key Insights from Implementation
- Neural networks can successfully learn logic gate behavior
- Building components hierarchically (gates → adders → ALU) works well
- Each component needs proper training (10,000 epochs)
- Two's complement for subtraction works with neural ALU
- Proper architecture (Half Adder → Full Adder → ALU) is essential