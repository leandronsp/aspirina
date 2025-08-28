use aspirina::layer::Layer;
use aspirina::matrix::Matrix;
use aspirina::neural_network::NeuralNetwork;

fn main() {
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

    let network = NeuralNetwork::new(layers);

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

    for idx in 0..100_000 {
        println!("Iteration: {}", idx);
        network.train(input.clone(), targets.clone());
    }

    println!(
        "predict([[1.0, 1.0, 0.0]]) = {:?}",
        network.predict(Matrix::new(vec![vec![1.0, 1.0, 0.0]])).data
    );
}
