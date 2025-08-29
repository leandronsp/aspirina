use aspirina::layer::Layer;
use aspirina::matrix::Matrix;
use aspirina::neural_network::NeuralNetwork;

pub fn run() {
    println!("=== AND Gate Training ===");
    println!("Training neural network to learn AND logic gate...");

    let network = create_network();

    let input = Matrix::new(vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ]);

    let targets = Matrix::new(vec![vec![0.0, 0.0, 0.0, 1.0]]);

    // Training loop with progress
    let epochs = 10_000;
    for idx in 0..epochs {
        if idx % 2000 == 0 || idx == epochs - 1 {
            println!("Epoch: {}/{}", idx + 1, epochs);
        }
        network.train(input.clone(), targets.clone());
    }

    println!("\n=== Training Complete ===");
    println!("Testing AND gate logic:");

    // Test all AND combinations
    let test_cases = [
        (vec![0.0, 0.0], "0 AND 0"),
        (vec![0.0, 1.0], "0 AND 1"),
        (vec![1.0, 0.0], "1 AND 0"),
        (vec![1.0, 1.0], "1 AND 1"),
    ];

    for (input_vals, description) in test_cases {
        let result = network.predict(Matrix::new(vec![input_vals.clone()]));
        let output = result.data[0][0];
        let expected = if input_vals[0] > 0.5 && input_vals[1] > 0.5 {
            1.0
        } else {
            0.0
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
    // Better biased network for AND gate
    // AND gate needs both inputs to be high, so we use positive weights
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![0.8, 0.8],   // Both inputs weighted positively
            vec![0.6, 0.6],   // Second neuron also positive
            vec![-0.3, -0.3], // Third neuron with negative bias
        ])),
        Layer::new(Matrix::new(vec![vec![1.2, 0.8, -0.5]])), // Output layer biased towards conjunction
    ];

    NeuralNetwork::new(layers)
}
