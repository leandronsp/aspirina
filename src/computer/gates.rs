use aspirina::layer::Layer;
use aspirina::matrix::Matrix;
use aspirina::neural_network::NeuralNetwork;

/// Represents all available logic gates
#[derive(Debug, Clone)]
pub enum GateType {
    AND,
    OR,
    XOR,
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
            GateType::OR => create_or_network(),
            GateType::XOR => create_xor_network(),
        };

        LogicGate { gate_type, network }
    }

    /// Train the gate with appropriate training data
    pub fn train(&self, epochs: usize) {
        let input = Matrix::new(vec![
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ]);

        let targets = match self.gate_type {
            GateType::AND => Matrix::new(vec![vec![0.0, 0.0, 0.0, 1.0]]),
            GateType::OR => Matrix::new(vec![vec![0.0, 1.0, 1.0, 1.0]]),
            GateType::XOR => Matrix::new(vec![vec![0.0, 1.0, 1.0, 0.0]]),
        };

        for _ in 0..epochs {
            self.network.train(input.clone(), targets.clone());
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

/// Convenience function to train and test all gates
pub fn test_all_gates() {
    let gates = [GateType::AND, GateType::OR, GateType::XOR];

    for gate_type in gates.iter() {
        println!("=== Testing {:?} Gate ===", gate_type);
        let gate = LogicGate::new(gate_type.clone());

        // Train the gate
        gate.train(10_000);

        // Test the gate
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
        GateType::OR => vec![
            (vec![0.0, 0.0], "0 OR 0", 0.0),
            (vec![0.0, 1.0], "0 OR 1", 1.0),
            (vec![1.0, 0.0], "1 OR 0", 1.0),
            (vec![1.0, 1.0], "1 OR 1", 1.0),
        ],
        GateType::XOR => vec![
            (vec![0.0, 0.0], "0 XOR 0", 0.0),
            (vec![0.0, 1.0], "0 XOR 1", 1.0),
            (vec![1.0, 0.0], "1 XOR 0", 1.0),
            (vec![1.0, 1.0], "1 XOR 1", 0.0),
        ],
    }
}
