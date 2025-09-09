use std::collections::HashMap;

/// Assembly instruction with optional operand
#[derive(Debug, Clone)]
struct AsmInstruction {
    opcode: String,
    operand: Option<u8>,
}

/// Simple assembler for 4-bit CPU
pub struct Assembler {
    instructions: Vec<AsmInstruction>,
    labels: HashMap<String, usize>,
    machine_code: Vec<u8>,
}

impl Assembler {
    /// Create a new assembler instance
    pub fn new() -> Self {
        Assembler {
            instructions: Vec::new(),
            labels: HashMap::new(),
            machine_code: Vec::new(),
        }
    }

    /// Parse assembly source code
    pub fn parse(&mut self, source: &str) -> Result<(), String> {
        let lines: Vec<&str> = source.lines().collect();
        let mut current_address = 0;

        // First pass: collect labels and instructions
        for (line_num, line) in lines.iter().enumerate() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            // Check for label definition
            if line.ends_with(':') {
                let label = line.trim_end_matches(':');
                self.labels.insert(label.to_string(), current_address);
                continue;
            }

            // Parse instruction
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let opcode = parts[0].to_uppercase();
            
            // Remove comments from operand if present
            let operand = if parts.len() > 1 {
                let op_str = parts[1].split(';').next().unwrap_or("").trim();
                if op_str.is_empty() {
                    None
                } else {
                    Some(self.parse_operand(op_str, line_num)?)
                }
            } else {
                None
            };

            let instruction = AsmInstruction {
                opcode: opcode.clone(),
                operand,
            };

            // Calculate next address based on instruction size
            current_address += self.instruction_size(&opcode);
            self.instructions.push(instruction);
        }

        Ok(())
    }

    /// Parse an operand (number or label reference)
    fn parse_operand(&self, operand: &str, line_num: usize) -> Result<u8, String> {
        // Check if it's a hex number (0x prefix)
        if operand.starts_with("0x") || operand.starts_with("0X") {
            let hex_str = &operand[2..];
            u8::from_str_radix(hex_str, 16)
                .map_err(|_| format!("Invalid hex number at line {}: '{}' (hex part: '{}')", line_num + 1, operand, hex_str))
                .and_then(|v| {
                    if v > 15 {
                        Err(format!("Value {} exceeds 4-bit range (0-15) at line {}", v, line_num + 1))
                    } else {
                        Ok(v)
                    }
                })
        }
        // Check if it's a decimal number
        else if operand.chars().all(|c| c.is_ascii_digit()) {
            operand.parse::<u8>()
                .map_err(|_| format!("Invalid number at line {}: '{}'", line_num + 1, operand))
                .and_then(|v| {
                    if v > 15 {
                        Err(format!("Value {} exceeds 4-bit range (0-15) at line {}", v, line_num + 1))
                    } else {
                        Ok(v)
                    }
                })
        }
        // Otherwise treat as label (will be resolved in second pass)
        else {
            Err(format!("Invalid operand at line {}: '{}' (not a number or valid label)", line_num + 1, operand))
        }
    }

    /// Get instruction size in bytes
    fn instruction_size(&self, opcode: &str) -> usize {
        match opcode {
            "NOP" | "INC" | "DEC" | "HALT" | "HLT" => 1,
            _ => 2, // All other instructions have operands
        }
    }

    /// Assemble parsed instructions into machine code
    pub fn assemble(&mut self) -> Result<Vec<u8>, String> {
        self.machine_code.clear();

        for instruction in &self.instructions {
            let opcode_byte = self.encode_opcode(&instruction.opcode)?;
            self.machine_code.push(opcode_byte);

            // Add operand if instruction has one
            if self.instruction_size(&instruction.opcode) == 2 {
                let operand = instruction.operand
                    .ok_or_else(|| format!("Instruction {} requires operand", instruction.opcode))?;
                self.machine_code.push(operand);
            }
        }

        Ok(self.machine_code.clone())
    }

    /// Encode instruction mnemonic to opcode byte
    fn encode_opcode(&self, mnemonic: &str) -> Result<u8, String> {
        match mnemonic {
            "NOP" => Ok(0x0),
            "LOAD" | "LD" => Ok(0x1),
            "STORE" | "ST" => Ok(0x2),
            "ADD" => Ok(0x3),
            "SUB" => Ok(0x4),
            "AND" => Ok(0x5),
            "OR" => Ok(0x6),
            "XOR" => Ok(0x7),
            "JUMP" | "JMP" => Ok(0x8),
            "JZ" => Ok(0x9),
            "LDI" => Ok(0xA),
            "INC" => Ok(0xB),
            "DEC" => Ok(0xC),
            "CMP" => Ok(0xD),
            "JNZ" => Ok(0xE),
            "HALT" | "HLT" => Ok(0xF),
            _ => Err(format!("Unknown instruction: {}", mnemonic)),
        }
    }

    /// Disassemble machine code back to assembly
    pub fn disassemble(machine_code: &[u8]) -> String {
        let mut result = String::new();
        let mut i = 0;

        while i < machine_code.len() {
            let opcode = machine_code[i];
            let address = i;

            let (mnemonic, has_operand) = match opcode {
                0x0 => ("NOP", false),
                0x1 => ("LOAD", true),
                0x2 => ("STORE", true),
                0x3 => ("ADD", true),
                0x4 => ("SUB", true),
                0x5 => ("AND", true),
                0x6 => ("OR", true),
                0x7 => ("XOR", true),
                0x8 => ("JUMP", true),
                0x9 => ("JZ", true),
                0xA => ("LDI", true),
                0xB => ("INC", false),
                0xC => ("DEC", false),
                0xD => ("CMP", true),
                0xE => ("JNZ", true),
                0xF => ("HALT", false),
                _ => ("???", false),
            };

            if has_operand && i + 1 < machine_code.len() {
                let operand = machine_code[i + 1];
                result.push_str(&format!("{:02X}: {} 0x{:X}\n", address, mnemonic, operand));
                i += 2;
            } else {
                result.push_str(&format!("{:02X}: {}\n", address, mnemonic));
                i += 1;
            }
        }

        result
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to assemble a program
pub fn assemble(source: &str) -> Result<Vec<u8>, String> {
    let mut assembler = Assembler::new();
    assembler.parse(source)?;
    assembler.assemble()
}

/// Test the assembler
pub fn test() {
    println!("=== Assembler Test ===");
    
    // Test program 1: Simple addition (self-contained)
    let program1 = r#"
        ; Self-contained addition program (5 + 3)
        LDI 5      ; Load 5 into accumulator
        STORE 0xE  ; Store 5 at address E
        LDI 3      ; Load 3 into accumulator
        STORE 0xD  ; Store 3 at address D
        LOAD 0xE   ; Load back the 5
        ADD 0xD    ; Add the 3 from address D
        STORE 0xF  ; Store result (8) at F
        HALT       ; Stop execution
    "#;

    match assemble(program1) {
        Ok(machine_code) => {
            println!("Program 1 (Self-contained 5+3) assembled successfully:");
            for (i, byte) in machine_code.iter().enumerate() {
                println!("  [{:02X}]: 0x{:02X}", i, byte);
            }
            
            // Test disassembly
            println!("\nDisassembled:");
            print!("{}", Assembler::disassemble(&machine_code));
        }
        Err(e) => println!("Assembly error: {}", e),
    }

    // Test program 2: Loop with counter
    let program2 = r#"
        ; Count down from 3 to 0
        LDI 3      ; Counter = 3
        DEC        ; Decrement counter
        JNZ 2      ; Jump back to DEC if not zero
        HALT       ; Stop when counter reaches 0
    "#;

    println!("\n--- Program 2: Countdown Loop ---");
    match assemble(program2) {
        Ok(machine_code) => {
            println!("Program 2 assembled successfully:");
            for (i, byte) in machine_code.iter().enumerate() {
                println!("  [{:02X}]: 0x{:02X}", i, byte);
            }
        }
        Err(e) => println!("Assembly error: {}", e),
    }

    // Test program 3: Logical operations
    let program3 = r#"
        ; Test AND and OR operations
        LDI 12     ; Load 12 (1100 in binary)
        STORE 0xC  ; Store 12 at C
        LDI 5      ; Load 5 (0101 in binary)
        STORE 0xD  ; Store 5 at D
        LOAD 0xC   ; Load 12
        AND 0xD    ; AND with 5 = 4 (0100)
        STORE 0xB  ; Store result at B
        LOAD 0xC   ; Load 12 again
        OR 0xD     ; OR with 5 = 13 (1101)
        STORE 0xA  ; Store result at A
        HALT
    "#;

    println!("\n--- Program 3: Logical Operations ---");
    match assemble(program3) {
        Ok(machine_code) => {
            println!("Program 3 assembled successfully:");
            for (i, byte) in machine_code.iter().enumerate() {
                println!("  [{:02X}]: 0x{:02X}", i, byte);
            }
        }
        Err(e) => println!("Assembly error: {}", e),
    }
}

