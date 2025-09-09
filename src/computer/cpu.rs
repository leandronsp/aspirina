use super::alu::{ALU, ALUOperation};
use super::memory::Memory;
use super::registers::CPURegisters;

/// 4-bit CPU Instructions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    NOP,         // 0x0: No operation
    LOAD(u8),    // 0x1: Load memory[addr] into A
    STORE(u8),   // 0x2: Store A into memory[addr]
    ADD(u8),     // 0x3: Add memory[addr] to A
    SUB(u8),     // 0x4: Subtract memory[addr] from A
    AND(u8),     // 0x5: AND memory[addr] with A
    OR(u8),      // 0x6: OR memory[addr] with A
    XOR(u8),     // 0x7: XOR memory[addr] with A
    JUMP(u8),    // 0x8: Set PC to addr
    JZ(u8),      // 0x9: Jump if zero flag set
    LDI(u8),     // 0xA: Load immediate value into A
    INC,         // 0xB: Increment A
    DEC,         // 0xC: Decrement A
    CMP(u8),     // 0xD: Compare A with memory[addr] (sets flags)
    JNZ(u8),     // 0xE: Jump if zero flag NOT set
    HALT,        // 0xF: Stop execution
}

/// Simple 4-bit CPU implementation
pub struct SimpleCPU {
    pub registers: CPURegisters,
    pub memory: Memory,
    pub alu: ALU,
    pub halted: bool,
    pub cycle_count: usize,
}

impl SimpleCPU {
    /// Create a new CPU instance
    pub fn new() -> Self {
        SimpleCPU {
            registers: CPURegisters::new(),
            memory: Memory::new(),
            alu: ALU::new(),
            halted: false,
            cycle_count: 0,
        }
    }

    /// Reset CPU to initial state
    pub fn reset(&mut self) {
        self.registers.reset();
        self.memory.clear();
        self.halted = false;
        self.cycle_count = 0;
    }

    /// Load a program into memory
    pub fn load_program(&mut self, program: &[u8]) {
        self.memory.load_program(program);
    }

    /// Fetch instruction from memory at PC
    fn fetch(&mut self) -> u8 {
        let pc = self.registers.program_counter.read();
        let instruction = self.memory.read(pc);
        self.registers.instruction_register.write(instruction);
        instruction
    }

    /// Decode instruction byte into Instruction enum
    /// Since memory is 4-bit, we need a two-byte instruction format:
    /// First byte: opcode, Second byte: operand (if needed)
    fn decode(&self, opcode: u8) -> Instruction {
        // For 4-bit memory, opcodes are direct values
        match opcode {
            0x0 => Instruction::NOP,
            0x1 => {
                // LOAD needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::LOAD(operand)
            },
            0x2 => {
                // STORE needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::STORE(operand)
            },
            0x3 => {
                // ADD needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::ADD(operand)
            },
            0x4 => {
                // SUB needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::SUB(operand)
            },
            0x5 => {
                // AND needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::AND(operand)
            },
            0x6 => {
                // OR needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::OR(operand)
            },
            0x7 => {
                // XOR needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::XOR(operand)
            },
            0x8 => {
                // JUMP needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::JUMP(operand)
            },
            0x9 => {
                // JZ needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::JZ(operand)
            },
            0xA => {
                // LDI needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::LDI(operand)
            },
            0xB => Instruction::INC,
            0xC => Instruction::DEC,
            0xD => {
                // CMP needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::CMP(operand)
            },
            0xE => {
                // JNZ needs operand from next memory location
                let pc = self.registers.program_counter.read();
                let operand = self.memory.read((pc + 1) & 0x0F);
                Instruction::JNZ(operand)
            },
            0xF => Instruction::HALT,
            _ => Instruction::NOP,
        }
    }

    /// Execute the given instruction
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NOP => {
                // Do nothing
            }
            Instruction::LOAD(addr) => {
                let value = self.memory.read(addr);
                self.registers.accumulator.write(value);
                self.registers.update_flags(value, false);
            }
            Instruction::STORE(addr) => {
                let value = self.registers.accumulator.read();
                self.memory.write(addr, value);
            }
            Instruction::ADD(addr) => {
                let a = self.registers.accumulator.read();
                let b = self.memory.read(addr);
                let result = self.alu.compute(a, b, ALUOperation::Add);
                self.registers.accumulator.write(result.result);
                self.registers.update_flags(result.result, result.carry);
            }
            Instruction::SUB(addr) => {
                let a = self.registers.accumulator.read();
                let b = self.memory.read(addr);
                let result = self.alu.compute(a, b, ALUOperation::Subtract);
                self.registers.accumulator.write(result.result);
                self.registers.update_flags(result.result, result.carry);
            }
            Instruction::AND(addr) => {
                let a = self.registers.accumulator.read();
                let b = self.memory.read(addr);
                let result = self.alu.compute(a, b, ALUOperation::And);
                self.registers.accumulator.write(result.result);
                self.registers.update_flags(result.result, false);
            }
            Instruction::OR(addr) => {
                let a = self.registers.accumulator.read();
                let b = self.memory.read(addr);
                let result = self.alu.compute(a, b, ALUOperation::Or);
                self.registers.accumulator.write(result.result);
                self.registers.update_flags(result.result, false);
            }
            Instruction::XOR(addr) => {
                let a = self.registers.accumulator.read();
                let b = self.memory.read(addr);
                let result = self.alu.compute(a, b, ALUOperation::Xor);
                self.registers.accumulator.write(result.result);
                self.registers.update_flags(result.result, false);
            }
            Instruction::JUMP(addr) => {
                self.registers.program_counter.write(addr);
                return; // Don't increment PC after jump
            }
            Instruction::JZ(addr) => {
                if self.registers.zero_flag {
                    self.registers.program_counter.write(addr);
                    return; // Don't increment PC after jump
                } else {
                    self.registers.program_counter.increment(); // Skip operand if not jumping
                }
            }
            Instruction::JNZ(addr) => {
                if !self.registers.zero_flag {
                    self.registers.program_counter.write(addr);
                    return; // Don't increment PC after jump
                } else {
                    self.registers.program_counter.increment(); // Skip operand if not jumping
                }
            }
            Instruction::LDI(value) => {
                self.registers.accumulator.write(value);
                self.registers.update_flags(value, false);
            }
            Instruction::INC => {
                let value = self.registers.accumulator.read();
                let result = self.alu.compute(value, 1, ALUOperation::Add);
                self.registers.accumulator.write(result.result);
                self.registers.update_flags(result.result, result.carry);
            }
            Instruction::DEC => {
                let value = self.registers.accumulator.read();
                let result = self.alu.compute(value, 1, ALUOperation::Subtract);
                self.registers.accumulator.write(result.result);
                self.registers.update_flags(result.result, result.carry);
            }
            Instruction::CMP(addr) => {
                let a = self.registers.accumulator.read();
                let b = self.memory.read(addr);
                let result = self.alu.compute(a, b, ALUOperation::Subtract);
                // Only update flags, don't store result
                self.registers.update_flags(result.result, result.carry);
            }
            Instruction::HALT => {
                self.halted = true;
                return; // Don't increment PC after halt
            }
        }
        
        // Increment PC appropriately
        // Two-byte instructions need extra increment to skip operand
        // (except JZ/JNZ which handle their own PC)
        let needs_extra_increment = matches!(instruction,
            Instruction::LOAD(_) | Instruction::STORE(_) | Instruction::ADD(_) |
            Instruction::SUB(_) | Instruction::AND(_) | Instruction::OR(_) |
            Instruction::XOR(_) | Instruction::LDI(_) | Instruction::CMP(_)
        );
        
        self.registers.program_counter.increment();
        if needs_extra_increment {
            self.registers.program_counter.increment(); // Skip operand byte
        }
    }

    /// Run one fetch-decode-execute cycle
    pub fn cycle(&mut self) {
        if self.halted {
            return;
        }
        
        let instruction_byte = self.fetch();
        let instruction = self.decode(instruction_byte);
        self.execute(instruction);
        self.cycle_count += 1;
    }

    /// Run until HALT or max cycles reached
    pub fn run(&mut self, max_cycles: usize) {
        while !self.halted && self.cycle_count < max_cycles {
            self.cycle();
        }
    }

    /// Display CPU state for debugging
    pub fn display_state(&self) {
        println!("=== CPU State (Cycle {}) ===", self.cycle_count);
        self.registers.display();
        println!("Halted: {}", self.halted);
        println!("\nMemory:");
        self.memory.test();
    }

    /// Test the CPU with a simple program
    pub fn test_simple_add(&mut self) {
        println!("=== CPU Test: Simple Addition ===");
        
        // Program: Load 5, Add 3, Store result at address 0xF
        // Using two-byte format for 4-bit memory
        let program = [
            0xA, 0x5,  // LDI 5 - Load immediate 5 into A
            0x3, 0xE,  // ADD E - Add memory[E] to A (we'll put 3 there)
            0x2, 0xF,  // STORE F - Store result at address F
            0xF,       // HALT
        ];
        
        // Put data value 3 at address 0xE
        self.memory.write(0xE, 3);
        
        // Load and run program
        self.load_program(&program);
        
        // Debug: Show loaded program
        println!("Loaded program in memory:");
        for i in 0..8 {
            println!("  Addr 0x{:X}: 0x{:X}", i, self.memory.read(i));
        }
        println!("  Data at 0xE: {}", self.memory.read(0xE));
        
        // Run with debug
        for i in 0..10 {
            if self.halted {
                println!("CPU halted at cycle {}", i);
                break;
            }
            let pc = self.registers.program_counter.read();
            let instr = self.memory.read(pc);
            println!("Cycle {}: PC=0x{:X}, Instruction=0x{:X}, A={}", 
                     i, pc, instr, self.registers.accumulator.read());
            self.cycle();
        }
        
        // Check result
        let result = self.memory.read(0xF);
        println!("Program result: {} (expected: 8)", result);
        println!("Final accumulator: {}", self.registers.accumulator.read());
        assert_eq!(result, 8);
        println!("✓ Simple addition test passed");
    }

    /// Test conditional jump
    pub fn test_conditional(&mut self) {
        println!("\n=== CPU Test: Conditional Jump ===");
        self.reset();
        
        // Program: Count down from 3 to 0
        let program = [
            0xA, 0x3,  // LDI 3 - Load 3 into A
            0x2, 0xE,  // STORE E - Store A at address E (counter)
            0xC,       // DEC - Decrement A (at address 4)
            0x9, 0x8,  // JZ 8 - Jump to address 8 if zero
            0x8, 0x4,  // JUMP 4 - Jump back to DEC
            0xF,       // HALT (at address 8)
        ];
        
        self.load_program(&program);
        self.run(30);
        
        let final_value = self.registers.accumulator.read();
        println!("Final accumulator: {} (expected: 0)", final_value);
        assert_eq!(final_value, 0);
        println!("✓ Conditional jump test passed");
    }

    /// Test logical operations
    pub fn test_logical(&mut self) {
        println!("\n=== CPU Test: Logical Operations ===");
        self.reset();
        
        // Program: Test AND, OR
        // Note: Results stored at 0xC and 0xB
        let program = [
            0xA, 0xC,  // LDI C (12) - Load 12 into A
            0x5, 0xD,  // AND D - AND with memory[D] (we'll put 5 there)
            0x2, 0xC,  // STORE C - Store result at C
            0xA, 0xA,  // LDI A (10) - Load 10 into A  
            0x6, 0xD,  // OR D - OR with memory[D] (5)
            0x2, 0xB,  // STORE B - Store result at B
            0xF,       // HALT
        ];
        
        self.memory.write(0xD, 5);
        self.load_program(&program);
        self.run(20);
        
        let and_result = self.memory.read(0xC);
        let or_result = self.memory.read(0xB);
        
        println!("12 AND 5 = {} (expected: 4)", and_result);
        println!("10 OR 5 = {} (expected: 15)", or_result);
        
        assert_eq!(and_result, 4);  // 1100 & 0101 = 0100
        assert_eq!(or_result, 15);  // 1010 | 0101 = 1111
        
        println!("✓ Logical operations test passed");
    }
}

impl Default for SimpleCPU {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to test CPU
pub fn test() {
    let mut cpu = SimpleCPU::new();
    
    cpu.test_simple_add();
    cpu.test_conditional();
    cpu.test_logical();
    
    println!("\n=== Final CPU State ===");
    cpu.display_state();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_fetch() {
        let mut cpu = SimpleCPU::new();
        cpu.memory.write(0, 0xA5); // LDI 5
        
        let instruction = cpu.fetch();
        assert_eq!(instruction, 0xA5);
        assert_eq!(cpu.registers.instruction_register.read(), 0x5); // IR gets lower nibble
    }

    #[test]
    fn test_cpu_decode() {
        let cpu = SimpleCPU::new();
        
        assert_eq!(cpu.decode(0x00), Instruction::NOP);
        assert_eq!(cpu.decode(0x15), Instruction::LOAD(5));
        assert_eq!(cpu.decode(0x2A), Instruction::STORE(0xA));
        assert_eq!(cpu.decode(0xF0), Instruction::HALT);
    }

    #[test]
    fn test_cpu_execute_ldi() {
        let mut cpu = SimpleCPU::new();
        cpu.execute(Instruction::LDI(7));
        assert_eq!(cpu.registers.accumulator.read(), 7);
    }

    #[test]
    fn test_cpu_program() {
        let mut cpu = SimpleCPU::new();
        
        // Simple program: Load 5, increment, store
        let program = [
            0xA5, // LDI 5
            0xB0, // INC
            0x2F, // STORE F
            0xF0, // HALT
        ];
        
        cpu.load_program(&program);
        cpu.run(10);
        
        assert_eq!(cpu.memory.read(0xF), 6);
        assert!(cpu.halted);
    }
}