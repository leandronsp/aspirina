use aspirina::layer::Layer;
use aspirina::matrix::Matrix;
use aspirina::neural_network::NeuralNetwork;

pub fn run() {
    println!("=== NAND Gate Training ===");
    println!("Training neural network to learn NAND logic gate...");

    let network = create_network();

    let input = Matrix::new(vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ]);

    let targets = Matrix::new(vec![vec![1.0, 1.0, 1.0, 0.0]]);

    // Training loop with progress
    let epochs = 10_000;
    for idx in 0..epochs {
        if idx % 2000 == 0 || idx == epochs - 1 {
            println!("Epoch: {}/{}", idx + 1, epochs);
        }
        network.train(input.clone(), targets.clone());
    }

    println!("\n=== Training Complete ===");
    println!("Testing NAND gate logic:");

    // Test all NAND combinations
    let test_cases = [
        (vec![0.0, 0.0], "0 NAND 0"),
        (vec![0.0, 1.0], "0 NAND 1"),
        (vec![1.0, 0.0], "1 NAND 0"),
        (vec![1.0, 1.0], "1 NAND 1"),
    ];

    for (input_vals, description) in test_cases {
        let result = network.predict(Matrix::new(vec![input_vals.clone()]));
        let output = result.data[0][0];
        let expected = if input_vals[0] > 0.5 && input_vals[1] > 0.5 {
            0.0  // NAND is NOT AND
        } else {
            1.0
        };

        println!(
            "{} = {:.4} (expected: {:.1})",
            description, output, expected
        );

        // Check if learning was successful
        let success = if expected > 0.5 {
            output > 0.5
        } else {
            output < 0.5
        };
        if success {
            println!("  ✓ Correct!");
        } else {
            println!("  ✗ Needs more training");
        }
    }
}

fn create_network() -> NeuralNetwork {
    // NAND gate is inverted AND - high unless both inputs are high
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