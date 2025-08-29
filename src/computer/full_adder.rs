use crate::computer::gates::{GateType, LogicGate};
use crate::computer::half_adder::HalfAdder;

/// A full adder built from two half adders and an OR gate
/// Full Adder = HalfAdder1(A, B) + HalfAdder2(Sum1, Cin) + OR(Carry1, Carry2)
#[derive(Debug)]
pub struct FullAdder {
    half_adder1: HalfAdder, // A + B
    half_adder2: HalfAdder, // (A XOR B) + Cin
    or_gate: LogicGate,     // Carry1 OR Carry2
}

/// Result of full adder computation
#[derive(Debug, PartialEq)]
pub struct FullAdderResult {
    pub sum: bool,
    pub carry: bool,
}

impl FullAdder {
    /// Create a new full adder using two half adders
    pub fn new() -> Self {
        let half_adder1 = HalfAdder::new();
        let half_adder2 = HalfAdder::new();
        let or_gate = LogicGate::new(GateType::OR);

        // Train the OR gate
        or_gate.train(10_000);

        FullAdder {
            half_adder1,
            half_adder2,
            or_gate,
        }
    }

    /// Compute full adder output for three binary inputs (A, B, Carry_in)
    pub fn compute(&self, a: bool, b: bool, carry_in: bool) -> FullAdderResult {
        // First half adder: A + B
        let result1 = self.half_adder1.compute(a, b);

        // Second half adder: (A XOR B) + Cin
        let result2 = self.half_adder2.compute(result1.sum, carry_in);

        // Final carry: Carry1 OR Carry2
        let carry1_f = if result1.carry { 1.0 } else { 0.0 };
        let carry2_f = if result2.carry { 1.0 } else { 0.0 };
        let final_carry_output = self.or_gate.compute(vec![carry1_f, carry2_f]);

        FullAdderResult {
            sum: result2.sum,
            carry: final_carry_output > 0.5,
        }
    }

    /// Test the full adder with all possible inputs
    pub fn test(&self) {
        println!("=== Full Adder Test ===");
        println!("A | B | Cin | Sum | Carry");
        println!("--|---|-----|-----|------");

        let test_cases = [
            (false, false, false),
            (false, false, true),
            (false, true, false),
            (false, true, true),
            (true, false, false),
            (true, false, true),
            (true, true, false),
            (true, true, true),
        ];

        for (a, b, cin) in test_cases.iter() {
            let result = self.compute(*a, *b, *cin);

            // Expected results for full adder
            let expected_sum = *a ^ *b ^ *cin;
            let expected_carry = (*a & *b) | (*cin & (*a ^ *b));

            let sum_correct = result.sum == expected_sum;
            let carry_correct = result.carry == expected_carry;

            println!(
                "{} | {} |  {} |  {} |   {}  {}",
                if *a { 1 } else { 0 },
                if *b { 1 } else { 0 },
                if *cin { 1 } else { 0 },
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

impl Default for FullAdder {
    fn default() -> Self {
        Self::new()
    }
}
