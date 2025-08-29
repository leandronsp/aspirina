use aspirina::layer::Layer;
use aspirina::matrix::Matrix;
use aspirina::neural_network::NeuralNetwork;

pub fn run() {
    println!("=== XOR Gate Training ===");
    println!("Training neural network to learn XOR logic gate...");

    let network = create_network();

    let input = Matrix::new(vec![
        vec![0.0, 0.0, 1.0],
        vec![0.0, 0.0, 0.0],
        vec![0.0, 1.0, 1.0],
        vec![0.0, 1.0, 0.0],
        vec![1.0, 0.0, 1.0],
        vec![1.0, 0.0, 0.0],
        vec![0.6, 0.6, 0.0],
        vec![0.6, 0.6, 1.0],
    ]);

    let targets = Matrix::new(vec![vec![0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0]]);

    // Training loop with progress
    let epochs = 10_000;
    for idx in 0..epochs {
        if idx % 2000 == 0 || idx == epochs - 1 {
            println!("Epoch: {}/{}", idx + 1, epochs);
        }
        network.train(input.clone(), targets.clone());
    }

    println!("\n=== Training Complete ===");
    println!("Testing XOR gate logic:");

    // Test all XOR combinations
    let test_cases = [
        (vec![0.0, 0.0, 0.0], "0 XOR 0"),
        (vec![0.0, 1.0, 0.0], "0 XOR 1"),
        (vec![1.0, 0.0, 0.0], "1 XOR 0"),
        (vec![1.0, 1.0, 0.0], "1 XOR 1"),
    ];

    for (input_vals, description) in test_cases {
        let result = network.predict(Matrix::new(vec![input_vals.clone()]));
        let output = result.data[0][0];
        let expected = if input_vals[0] != input_vals[1] {
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
    let layers = vec![
        Layer::new(Matrix::new(vec![
            vec![-0.16595599, -0.70648822, -0.20646505],
            vec![0.44064899, -0.81532281, 0.07763347],
            vec![-0.99977125, -0.62747958, -0.16161097],
            vec![-0.39533485, -0.30887855, 0.370439],
        ])),
        Layer::new(Matrix::new(vec![
            vec![-0.16595599, -0.70648822, -0.20646505, -0.34093502],
            vec![0.44064899, -0.81532281, 0.07763347, 0.44093502],
            vec![-0.99977125, -0.62747958, -0.16161097, 0.14093502],
            vec![-0.39533485, -0.30887855, 0.370439, -0.54093502],
        ])),
        Layer::new(Matrix::new(vec![
            vec![-0.23456789, 0.87654321, -0.34567891, 0.12345678],
            vec![0.98765432, -0.45678912, 0.56789123, -0.67891234],
            vec![-0.89123456, 0.78912345, -0.43219876, 0.32198765],
            vec![0.65432109, -0.54321098, 0.21098765, -0.10987654],
        ])),
        Layer::new(Matrix::new(vec![vec![
            -0.5910955,
            0.75623487,
            -0.94522481,
            0.64093502,
        ]])),
    ];

    NeuralNetwork::new(layers)
}
