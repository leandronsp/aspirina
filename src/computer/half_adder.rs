use crate::computer::gates::{GateType, LogicGate};

/// A half adder that computes sum and carry for two bits
/// Sum = A XOR B, Carry = A AND B
#[derive(Debug)]
pub struct HalfAdder {
    xor_gate: LogicGate,
    and_gate: LogicGate,
}

/// Result of half adder computation
#[derive(Debug, PartialEq)]
pub struct HalfAdderResult {
    pub sum: bool,
    pub carry: bool,
}

impl HalfAdder {
    /// Create a new half adder with trained gates
    pub fn new() -> Self {
        let xor_gate = LogicGate::new(GateType::XOR);
        let and_gate = LogicGate::new(GateType::AND);

        // Train both gates
        xor_gate.train(10_000);
        and_gate.train(10_000);

        HalfAdder { xor_gate, and_gate }
    }

    /// Compute half adder output for two binary inputs
    pub fn compute(&self, a: bool, b: bool) -> HalfAdderResult {
        let a_f = if a { 1.0 } else { 0.0 };
        let b_f = if b { 1.0 } else { 0.0 };

        let sum_output = self.xor_gate.compute(vec![a_f, b_f]);
        let carry_output = self.and_gate.compute(vec![a_f, b_f]);

        HalfAdderResult {
            sum: sum_output > 0.5,
            carry: carry_output > 0.5,
        }
    }

    /// Test the half adder with all possible inputs
    pub fn test(&self) {
        println!("=== Half Adder Test ===");
        println!("A | B | Sum | Carry");
        println!("--|---|-----|------");

        let test_cases = [(false, false), (false, true), (true, false), (true, true)];

        for (a, b) in test_cases.iter() {
            let result = self.compute(*a, *b);
            let expected_sum = *a ^ *b; // XOR
            let expected_carry = *a & *b; // AND

            let sum_correct = result.sum == expected_sum;
            let carry_correct = result.carry == expected_carry;

            println!(
                "{} | {} |  {}  |   {}   {}",
                if *a { 1 } else { 0 },
                if *b { 1 } else { 0 },
                if result.sum { 1 } else { 0 },
                if result.carry { 1 } else { 0 },
                if sum_correct && carry_correct {
                    "✓"
                } else {
                    "✗"
                }
            );
        }
        println!();
    }
}

impl Default for HalfAdder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_half_adder_truth_table() {
        let adder = HalfAdder::new();

        // Test all combinations
        assert_eq!(
            adder.compute(false, false),
            HalfAdderResult {
                sum: false,
                carry: false
            }
        );
        assert_eq!(
            adder.compute(false, true),
            HalfAdderResult {
                sum: true,
                carry: false
            }
        );
        assert_eq!(
            adder.compute(true, false),
            HalfAdderResult {
                sum: true,
                carry: false
            }
        );
        assert_eq!(
            adder.compute(true, true),
            HalfAdderResult {
                sum: false,
                carry: true
            }
        );
    }
}
