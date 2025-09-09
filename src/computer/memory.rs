/// 4-bit Memory Cell that stores a single 4-bit value
#[derive(Debug, Clone, Copy)]
pub struct MemoryCell {
    data: u8, // 4-bit value (0-15)
}

impl MemoryCell {
    /// Create a new memory cell with initial value 0
    pub fn new() -> Self {
        MemoryCell { data: 0 }
    }

    /// Create a memory cell with specific initial value
    pub fn with_value(value: u8) -> Self {
        MemoryCell {
            data: value & 0x0F, // Ensure 4-bit
        }
    }

    /// Read the stored value
    pub fn read(&self) -> u8 {
        self.data
    }

    /// Write a new value (4-bit masked)
    pub fn write(&mut self, value: u8) {
        self.data = value & 0x0F;
    }
}

impl Default for MemoryCell {
    fn default() -> Self {
        Self::new()
    }
}

/// 16 x 4-bit Memory Bank for the neural computer
#[derive(Debug)]
pub struct Memory {
    cells: [MemoryCell; 16],
}

impl Memory {
    /// Create new memory with all cells initialized to 0
    pub fn new() -> Self {
        Memory {
            cells: [MemoryCell::new(); 16],
        }
    }

    /// Read from memory at given address (4-bit address: 0-15)
    pub fn read(&self, address: u8) -> u8 {
        let addr = (address & 0x0F) as usize; // Ensure 4-bit address
        self.cells[addr].read()
    }

    /// Write to memory at given address
    pub fn write(&mut self, address: u8, value: u8) {
        let addr = (address & 0x0F) as usize; // Ensure 4-bit address
        self.cells[addr].write(value);
    }

    /// Load program data into memory starting at address 0
    pub fn load_program(&mut self, program: &[u8]) {
        for (i, &instruction) in program.iter().enumerate() {
            if i >= 16 {
                break; // Memory is only 16 cells
            }
            self.write(i as u8, instruction);
        }
    }

    /// Get a snapshot of all memory contents for debugging
    pub fn dump(&self) -> [u8; 16] {
        let mut dump = [0u8; 16];
        for i in 0..16 {
            dump[i] = self.cells[i].read();
        }
        dump
    }

    /// Clear all memory (set to 0)
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.write(0);
        }
    }

    /// Initialize memory with test data
    pub fn init_test_data(&mut self) {
        // Example test program: add two numbers
        // Using 4-bit values only (0x0 - 0xF)
        self.write(0x0, 0x1); // Instruction: LOAD
        self.write(0x1, 0x5); // Address: 0x5
        self.write(0x2, 0x3); // Instruction: ADD  
        self.write(0x3, 0x6); // Address: 0x6
        self.write(0x4, 0x2); // Instruction: STORE
        self.write(0x5, 0x7); // Address: 0x7
        self.write(0x6, 0xF); // Instruction: HALT
        self.write(0x7, 0x0); // Unused
        self.write(0x8, 0x3); // Data: 3
        self.write(0x9, 0x5); // Data: 5
        self.write(0xA, 0x0); // Result will be stored here
    }

    /// Test the memory component
    pub fn test(&self) {
        println!("=== Memory Test ===");
        println!("Address | Value");
        println!("--------|------");
        
        for i in 0..16 {
            let value = self.read(i);
            println!("  0x{:X}   |  0x{:X}", i, value);
        }
        println!();
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to test memory
pub fn test() {
    let mut memory = Memory::new();
    
    // Test basic read/write
    println!("=== Memory Component Test ===");
    println!("Testing basic read/write operations...");
    
    memory.write(0x5, 0xA);
    memory.write(0xF, 0x3);
    
    assert_eq!(memory.read(0x5), 0xA);
    assert_eq!(memory.read(0xF), 0x3);
    assert_eq!(memory.read(0x0), 0x0); // Unwritten should be 0
    
    println!("✓ Basic read/write working");
    
    // Test 4-bit masking
    memory.write(0x1, 0xFF); // Should be masked to 0xF
    assert_eq!(memory.read(0x1), 0xF);
    println!("✓ 4-bit masking working");
    
    // Test program loading  
    let program = [0x1, 0x3, 0x2, 0xF];
    memory.load_program(&program);
    
    for (i, &expected) in program.iter().enumerate() {
        assert_eq!(memory.read(i as u8), expected);
    }
    println!("✓ Program loading working");
    
    // Load test data and display
    memory.clear();
    memory.init_test_data();
    memory.test();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_cell_basic() {
        let mut cell = MemoryCell::new();
        assert_eq!(cell.read(), 0);
        
        cell.write(0x5);
        assert_eq!(cell.read(), 0x5);
        
        cell.write(0xFF); // Should be masked to 0xF
        assert_eq!(cell.read(), 0xF);
    }

    #[test]
    fn test_memory_addressing() {
        let mut memory = Memory::new();
        
        // Test all 16 addresses
        for i in 0..16 {
            memory.write(i, i);
        }
        
        for i in 0..16 {
            assert_eq!(memory.read(i), i);
        }
    }

    #[test]
    fn test_memory_masking() {
        let mut memory = Memory::new();
        
        // Test address masking (should wrap around)
        memory.write(0x1F, 0x7); // Address 0x1F should wrap to 0xF
        assert_eq!(memory.read(0xF), 0x7);
        
        // Test value masking
        memory.write(0x5, 0xAB); // Should be masked to 0xB
        assert_eq!(memory.read(0x5), 0xB);
    }

    #[test]
    fn test_program_loading() {
        let mut memory = Memory::new();
        let program = [0x1, 0x2, 0x3, 0x4, 0x5];
        
        memory.load_program(&program);
        
        for (i, &expected) in program.iter().enumerate() {
            assert_eq!(memory.read(i as u8), expected);
        }
    }
}