use aspirina::layer::Layer;
use aspirina::matrix::Matrix;
use aspirina::neural_network::NeuralNetwork;

#[test]
fn create_neural_network() {
    let layers = vec![
        Layer::new(Matrix::new(vec![vec![0.1, 0.2], vec![0.3, 0.4]])),
        Layer::new(Matrix::new(vec![vec![0.5, 0.6]])),
    ];

    let network = NeuralNetwork::new(layers);

    // Test prediction with simple input
    let input = Matrix::new(vec![vec![1.0, 0.0]]);
    let result = network.predict(input);

    // Just verify we get a result (exact values depend on implementation)
    assert!(!result.data.is_empty());
    assert_eq!(result.data.len(), 1);
    assert_eq!(result.data[0].len(), 1);
}
