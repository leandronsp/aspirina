use crate::layer::Layer;
use crate::matrix::Matrix;
use crate::neural_network::NeuralNetwork;

/// Represents all available logic gates
#[derive(Debug, Clone)]
pub enum GateType {
    AND,
    NAND,
    OR,
    NOR,
    XOR,
    XNOR,
    NOT,
}

/// A neural logic gate that can perform any of the 7 basic boolean operations
#[derive(Debug)]
pub struct LogicGate {
    gate_type: GateType,
    network: NeuralNetwork,
}

impl LogicGate {
    /// Creates a new logic gate of the specified type
    pub fn new(gate_type: GateType) -> Self {
        let network = match gate_type {
            GateType::AND => create_and_network(),
            GateType::NAND => create_nand_network(),
            GateType::OR => create_or_network(),
            GateType::NOR => create_nor_network(),
            GateType::XOR => create_xor_network(),
            GateType::XNOR => create_xnor_network(),
            GateType::NOT => create_not_network(),
        };

        LogicGate { gate_type, network }
    }

    /// Train the gate with appropriate training data
    pub fn train(&self, epochs: usize) {
        match self.gate_type {
            GateType::NOT => {
                let input = Matrix::new(vec![vec![0.0], vec![1.0]]);
                let targets = Matrix::new(vec![vec![1.0, 0.0]]);

                for _ in 0..epochs {
                    self.network.train(input.clone(), targets.clone());
                }
            }
            _ => {
                let input = Matrix::new(vec![
                    vec![0.0, 0.0],
                    vec![0.0, 1.0],
                    vec![1.0, 0.0],
                    vec![1.0, 1.0],
                ]);

                let targets = match self.gate_type {
                    GateType::AND => Matrix::new(vec![vec![0.0, 0.0, 0.0, 1.0]]),
                    GateType::NAND => Matrix::new(vec![vec![1.0, 1.0, 1.0, 0.0]]),
                    GateType::OR => Matrix::new(vec![vec![0.0, 1.0, 1.0, 1.0]]),
                    GateType::NOR => Matrix::new(vec![vec![1.0, 0.0, 0.0, 0.0]]),
                    GateType::XOR => Matrix::new(vec![vec![0.0, 1.0, 1.0, 0.0]]),
                    GateType::XNOR => Matrix::new(vec![vec![1.0, 0.0, 0.0, 1.0]]),
                    GateType::NOT => unreachable!(), // Handled above
                };

                for _ in 0..epochs {
                    self.network.train(input.clone(), targets.clone());
                }
            }
        }
    }

    /// Compute the gate output for given inputs
    pub fn compute(&self, inputs: Vec<f64>) -> f64 {
        let input_matrix = Matrix::new(vec![inputs]);
        let result = self.network.predict(input_matrix);
        result.data[0][0]
    }

    /// Get the gate type
    pub fn gate_type(&self) -> &GateType {
        &self.gate_type
    }
}

// Network creation functions for each gate type
fn create_and_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![0.8, 0.8],   // First neuron positive bias
            vec![0.6, 0.6],   // Second neuron also positive
            vec![-0.3, -0.3], // Third neuron with negative bias
        ])),
        Layer::new(Matrix::new(vec![vec![1.2, 0.8, -0.5]])), // Output layer biased towards conjunction
    ];
    NeuralNetwork::new(layers)
}

fn create_nand_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![-0.8, -0.8], // Negative weights for NAND logic
            vec![0.5, 0.5],   // Positive bias neuron
            vec![-0.3, 0.3],  // Mixed weights for complexity
        ])),
        Layer::new(Matrix::new(vec![vec![-1.2, 1.5, 0.8]])),
    ];
    NeuralNetwork::new(layers)
}

fn create_or_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![1.0, 1.0],   // Strong positive weights for OR
            vec![0.5, 0.5],   // Additional positive weights
            vec![-0.2, -0.2], // Small negative bias
        ])),
        Layer::new(Matrix::new(vec![vec![1.5, 1.0, -0.3]])), // Output favors disjunction
    ];
    NeuralNetwork::new(layers)
}

fn create_nor_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![-0.8, -0.8], // Negative weights to invert OR logic
            vec![-0.5, -0.5], // Additional negative weights
            vec![0.3, 0.3],   // Helper positive weights
        ])),
        Layer::new(Matrix::new(vec![vec![1.0, 1.2, -0.4]])),
    ];
    NeuralNetwork::new(layers)
}

fn create_xor_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![0.5, 0.5],
            vec![-0.3, -0.3],
            vec![0.8, -0.8],
            vec![-0.6, 0.6],
        ])),
        Layer::new(Matrix::new(vec![vec![0.9, -0.7, 1.2, -0.4]])),
    ];
    NeuralNetwork::new(layers)
}

fn create_xnor_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![-0.5, -0.5],
            vec![0.3, 0.3],
            vec![0.8, -0.8],
            vec![-0.6, 0.6],
        ])),
        Layer::new(Matrix::new(vec![vec![-0.9, 0.7, -1.2, 0.4]])), // Inverted from XOR weights
    ];
    NeuralNetwork::new(layers)
}

fn create_not_network() -> NeuralNetwork {
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![-1.5], // Strong negative weight to invert
            vec![0.8],  // Helper neuron
        ])),
        Layer::new(Matrix::new(vec![vec![1.2, -0.6]])),
    ];
    NeuralNetwork::new(layers)
}

/// Convenience function to train and test all gates
pub fn test_all_gates() {
    let gates = [
        GateType::AND,
        GateType::NAND,
        GateType::OR,
        GateType::NOR,
        GateType::XOR,
        GateType::XNOR,
        GateType::NOT,
    ];

    for gate_type in gates.iter() {
        println!("=== Testing {:?} Gate ===", gate_type);
        let gate = LogicGate::new(gate_type.clone());

        // Train the gate
        gate.train(10_000);

        // Test the gate
        match gate_type {
            GateType::NOT => {
                let test_cases = [(vec![0.0], "NOT 0", 1.0), (vec![1.0], "NOT 1", 0.0)];

                for (inputs, description, expected) in test_cases.iter() {
                    let output = gate.compute(inputs.clone());
                    let success = if *expected > 0.5 {
                        output > 0.5
                    } else {
                        output < 0.5
                    };
                    println!(
                        "{}: {:.4} (expected: {:.1}) {}",
                        description,
                        output,
                        expected,
                        if success { "✓" } else { "✗" }
                    );
                }
            }
            _ => {
                let test_cases = get_test_cases(gate_type);

                for (inputs, description, expected) in test_cases.iter() {
                    let output = gate.compute(inputs.clone());
                    let success = if *expected > 0.5 {
                        output > 0.5
                    } else {
                        output < 0.5
                    };
                    println!(
                        "{}: {:.4} (expected: {:.1}) {}",
                        description,
                        output,
                        expected,
                        if success { "✓" } else { "✗" }
                    );
                }
            }
        }
        println!();
    }
}

fn get_test_cases(gate_type: &GateType) -> Vec<(Vec<f64>, &'static str, f64)> {
    match gate_type {
        GateType::AND => vec![
            (vec![0.0, 0.0], "0 AND 0", 0.0),
            (vec![0.0, 1.0], "0 AND 1", 0.0),
            (vec![1.0, 0.0], "1 AND 0", 0.0),
            (vec![1.0, 1.0], "1 AND 1", 1.0),
        ],
        GateType::NAND => vec![
            (vec![0.0, 0.0], "0 NAND 0", 1.0),
            (vec![0.0, 1.0], "0 NAND 1", 1.0),
            (vec![1.0, 0.0], "1 NAND 0", 1.0),
            (vec![1.0, 1.0], "1 NAND 1", 0.0),
        ],
        GateType::OR => vec![
            (vec![0.0, 0.0], "0 OR 0", 0.0),
            (vec![0.0, 1.0], "0 OR 1", 1.0),
            (vec![1.0, 0.0], "1 OR 0", 1.0),
            (vec![1.0, 1.0], "1 OR 1", 1.0),
        ],
        GateType::NOR => vec![
            (vec![0.0, 0.0], "0 NOR 0", 1.0),
            (vec![0.0, 1.0], "0 NOR 1", 0.0),
            (vec![1.0, 0.0], "1 NOR 0", 0.0),
            (vec![1.0, 1.0], "1 NOR 1", 0.0),
        ],
        GateType::XOR => vec![
            (vec![0.0, 0.0], "0 XOR 0", 0.0),
            (vec![0.0, 1.0], "0 XOR 1", 1.0),
            (vec![1.0, 0.0], "1 XOR 0", 1.0),
            (vec![1.0, 1.0], "1 XOR 1", 0.0),
        ],
        GateType::XNOR => vec![
            (vec![0.0, 0.0], "0 XNOR 0", 1.0),
            (vec![0.0, 1.0], "0 XNOR 1", 0.0),
            (vec![1.0, 0.0], "1 XNOR 0", 0.0),
            (vec![1.0, 1.0], "1 XNOR 1", 1.0),
        ],
        GateType::NOT => vec![], // Handled separately
    }
}

