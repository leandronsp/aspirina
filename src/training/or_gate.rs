use aspirina::layer::Layer;
use aspirina::matrix::Matrix;
use aspirina::neural_network::NeuralNetwork;

pub fn run() {
    println!("=== OR Gate Training ===");
    println!("Training neural network to learn OR logic gate...");

    let network = create_network();

    let input = Matrix::new(vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ]);

    let targets = Matrix::new(vec![vec![0.0, 1.0, 1.0, 1.0]]);

    // Training loop with progress
    let epochs = 10_000;
    for idx in 0..epochs {
        if idx % 2000 == 0 || idx == epochs - 1 {
            println!("Epoch: {}/{}", idx + 1, epochs);
        }
        network.train(input.clone(), targets.clone());
    }

    println!("\n=== Training Complete ===");
    println!("Testing OR gate logic:");

    // Test all OR combinations
    let test_cases = [
        (vec![0.0, 0.0], "0 OR 0"),
        (vec![0.0, 1.0], "0 OR 1"),
        (vec![1.0, 0.0], "1 OR 0"),
        (vec![1.0, 1.0], "1 OR 1"),
    ];

    for (input_vals, description) in test_cases {
        let result = network.predict(Matrix::new(vec![input_vals.clone()]));
        let output = result.data[0][0];
        let expected = if input_vals[0] > 0.5 || input_vals[1] > 0.5 {
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
    // OR gate needs at least one input to be high
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![1.0, 1.0],   // Positive weights for disjunction
            vec![0.7, 0.7],   // Second neuron
            vec![-0.2, -0.2], // Negative bias neuron
        ])),
        Layer::new(Matrix::new(vec![vec![1.5, 1.0, -0.3]])),
    ];

    NeuralNetwork::new(layers)
}
