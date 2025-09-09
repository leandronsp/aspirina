use super::full_adder::FullAdder;
use super::gates::{GateType, LogicGate};

/// 4-bit ALU operations
#[derive(Debug, Clone, PartialEq)]
pub enum ALUOperation {
    Add,      // A + B
    Subtract, // A - B (using two's complement)
    And,      // A & B
    Or,       // A | B
    Xor,      // A ^ B
}

/// 4-bit ALU result
#[derive(Debug, PartialEq)]
pub struct ALUResult {
    pub result: u8,  // 4-bit result (0-15)
    pub carry: bool, // Carry/overflow flag
    pub zero: bool,  // Zero flag
}

/// 4-bit Arithmetic Logic Unit built from full adders and logic gates
#[derive(Debug)]
pub struct ALU {
    // Four full adders for arithmetic operations
    adder0: FullAdder, // LSB
    adder1: FullAdder,
    adder2: FullAdder,
    adder3: FullAdder, // MSB
    // Logic gates for bitwise operations
    and_gates: [LogicGate; 4],
    or_gates: [LogicGate; 4],
    xor_gates: [LogicGate; 4],
}

impl ALU {
    /// Create a new 4-bit ALU
    pub fn new() -> Self {
        let adder0 = FullAdder::new();
        let adder1 = FullAdder::new();
        let adder2 = FullAdder::new();
        let adder3 = FullAdder::new();

        // Create logic gates for each bit position
        let and_gates = [
            LogicGate::new(GateType::AND),
            LogicGate::new(GateType::AND),
            LogicGate::new(GateType::AND),
            LogicGate::new(GateType::AND),
        ];

        let or_gates = [
            LogicGate::new(GateType::OR),
            LogicGate::new(GateType::OR),
            LogicGate::new(GateType::OR),
            LogicGate::new(GateType::OR),
        ];

        let xor_gates = [
            LogicGate::new(GateType::XOR),
            LogicGate::new(GateType::XOR),
            LogicGate::new(GateType::XOR),
            LogicGate::new(GateType::XOR),
        ];

        // Train all logic gates
        for i in 0..4 {
            and_gates[i].train(10_000);
            or_gates[i].train(10_000);
            xor_gates[i].train(10_000);
        }

        ALU {
            adder0,
            adder1,
            adder2,
            adder3,
            and_gates,
            or_gates,
            xor_gates,
        }
    }

    /// Perform ALU operation on two 4-bit numbers
    pub fn compute(&self, a: u8, b: u8, operation: ALUOperation) -> ALUResult {
        // Ensure inputs are 4-bit
        // Mask out extra bits, e.g. 0x0F for 4-bit inputs
        // Example: 0010 0101 -> 0000 0101
        let a = a & 0x0F;
        let b = b & 0x0F;

        match operation {
            ALUOperation::Add => self.add(a, b),
            ALUOperation::Subtract => self.subtract(a, b),
            ALUOperation::And => self.bitwise_and(a, b),
            ALUOperation::Or => self.bitwise_or(a, b),
            ALUOperation::Xor => self.bitwise_xor(a, b),
        }
    }

    /// 4-bit addition using chain of full adders
    fn add(&self, a: u8, b: u8) -> ALUResult {
        let a_bits = self.to_bits(a);
        let b_bits = self.to_bits(b);

        // Chain full adders
        let result0 = self.adder0.compute(a_bits[0], b_bits[0], false);
        let result1 = self.adder1.compute(a_bits[1], b_bits[1], result0.carry);
        let result2 = self.adder2.compute(a_bits[2], b_bits[2], result1.carry);
        let result3 = self.adder3.compute(a_bits[3], b_bits[3], result2.carry);

        let result_bits = [result0.sum, result1.sum, result2.sum, result3.sum];
        let result_value = self.bits_to_u8(result_bits);

        ALUResult {
            result: result_value,
            carry: result3.carry,
            zero: result_value == 0,
        }
    }

    /// 4-bit subtraction using two's complement (A - B = A + (~B + 1))
    fn subtract(&self, a: u8, b: u8) -> ALUResult {
        // Two's complement: invert bits and add 1
        let b_complement = (!b) & 0x0F;
        let b_plus_one = self.add(b_complement, 1);

        // A + (~B + 1)
        self.add(a, b_plus_one.result)
    }

    /// 4-bit bitwise AND
    fn bitwise_and(&self, a: u8, b: u8) -> ALUResult {
        let a_bits = self.to_bits(a);
        let b_bits = self.to_bits(b);

        let mut result_bits = [false; 4];
        for i in 0..4 {
            let a_f = if a_bits[i] { 1.0 } else { 0.0 };
            let b_f = if b_bits[i] { 1.0 } else { 0.0 };
            let output = self.and_gates[i].compute(vec![a_f, b_f]);
            result_bits[i] = output > 0.5;
        }

        let result_value = self.bits_to_u8(result_bits);
        ALUResult {
            result: result_value,
            carry: false,
            zero: result_value == 0,
        }
    }

    /// 4-bit bitwise OR
    fn bitwise_or(&self, a: u8, b: u8) -> ALUResult {
        let a_bits = self.to_bits(a);
        let b_bits = self.to_bits(b);

        let mut result_bits = [false; 4];
        for i in 0..4 {
            let a_f = if a_bits[i] { 1.0 } else { 0.0 };
            let b_f = if b_bits[i] { 1.0 } else { 0.0 };
            let output = self.or_gates[i].compute(vec![a_f, b_f]);
            result_bits[i] = output > 0.5;
        }

        let result_value = self.bits_to_u8(result_bits);
        ALUResult {
            result: result_value,
            carry: false,
            zero: result_value == 0,
        }
    }

    /// 4-bit bitwise XOR
    fn bitwise_xor(&self, a: u8, b: u8) -> ALUResult {
        let a_bits = self.to_bits(a);
        let b_bits = self.to_bits(b);

        let mut result_bits = [false; 4];
        for i in 0..4 {
            let a_f = if a_bits[i] { 1.0 } else { 0.0 };
            let b_f = if b_bits[i] { 1.0 } else { 0.0 };
            let output = self.xor_gates[i].compute(vec![a_f, b_f]);
            result_bits[i] = output > 0.5;
        }

        let result_value = self.bits_to_u8(result_bits);
        ALUResult {
            result: result_value,
            carry: false,
            zero: result_value == 0,
        }
    }

    /// Convert u8 to array of 4 bits (LSB first)
    fn to_bits(&self, value: u8) -> [bool; 4] {
        [
            (value & 0x01) != 0, // bit 0 (LSB)
            (value & 0x02) != 0, // bit 1
            (value & 0x04) != 0, // bit 2
            (value & 0x08) != 0, // bit 3 (MSB)
        ]
    }

    /// Convert array of 4 bits to u8 (LSB first)
    fn bits_to_u8(&self, bits: [bool; 4]) -> u8 {
        let mut result = 0u8;
        if bits[0] {
            result |= 0x01;
        }
        if bits[1] {
            result |= 0x02;
        }
        if bits[2] {
            result |= 0x04;
        }
        if bits[3] {
            result |= 0x08;
        }
        result
    }

    /// Test the ALU with various operations
    pub fn test(&self) {
        println!("=== 4-bit ALU Test ===");

        let test_cases = [
            (5, 3, ALUOperation::Add, "5 + 3"),
            (7, 2, ALUOperation::Add, "7 + 2"),
            (15, 1, ALUOperation::Add, "15 + 1"),
            (8, 3, ALUOperation::Subtract, "8 - 3"),
            (5, 7, ALUOperation::Subtract, "5 - 7"),
            (12, 5, ALUOperation::And, "12 & 5"),
            (10, 3, ALUOperation::Or, "10 | 3"),
            (15, 5, ALUOperation::Xor, "15 ^ 5"),
        ];

        for (a, b, op, description) in test_cases.iter() {
            let result = self.compute(*a, *b, op.clone());

            let expected = match op {
                ALUOperation::Add => (a + b) & 0x0F,
                ALUOperation::Subtract => (a.wrapping_sub(*b)) & 0x0F,
                ALUOperation::And => a & b,
                ALUOperation::Or => a | b,
                ALUOperation::Xor => a ^ b,
            };

            let correct = result.result == expected;

            println!(
                "{} = {} (expected: {}) {} [C:{} Z:{}]",
                description,
                result.result,
                expected,
                if correct { "✓" } else { "✗" },
                if result.carry { 1 } else { 0 },
                if result.zero { 1 } else { 0 }
            );
        }
        println!();
    }
}

impl Default for ALU {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function to test ALU
pub fn test() {
    ALU::new().test();
}
