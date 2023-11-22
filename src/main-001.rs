mod utils;
use crate::utils::Utils;

fn main() {
    // 4 x 3
    let mut input_layer = vec![
        vec![1.0, 1.0, 1.0], 
        vec![1.0, 1.0, 1.0], 
        vec![1.0, 1.0, 1.0], 
        vec![1.0, 1.0, 1.0], 
    ];

    // 4 x 4
    let mut hidden_layer = vec![
        vec![1.0, 1.0, 1.0, 1.0], 
        vec![1.0, 1.0, 1.0, 1.0], 
        vec![1.0, 1.0, 1.0, 1.0], 
        vec![1.0, 1.0, 1.0, 1.0], 
    ];

    // 1 x 4
    let mut output_layer = vec![
        vec![1.0, 1.0, 1.0, 1.0]
    ];

    // Input
    let input = vec![
        vec![0.0, 0.0, 1.0], vec![0.0, 1.0, 1.0], vec![1.0, 0.0, 1.0], vec![0.0, 1.0, 0.0], 
        vec![1.0, 0.0, 0.0], vec![1.0, 1.0, 1.0], vec![0.0, 0.0, 0.0]
    ];

    // Targets
    let targets = vec![vec![0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0]];
    let times = 100;

    println!("Going to train the network {} times", times);

    for _ in 0..times {
        let result = train(
            input.clone(), 
            targets.clone(), 
            input_layer.clone(), 
            hidden_layer.clone(), 
            output_layer.clone()
        );

        input_layer = result.0;
        hidden_layer = result.1;
        output_layer = result.2;
    }

    // Predict
    //
    let input = vec![vec![1.0, 1.0, 0.0]];
    println!("Going to predict the following input: {:?}", input);

    let input_layer_result = forward(input.clone(), input_layer.clone());
    let hidden_layer_result = forward(input_layer_result.clone(), hidden_layer.clone());
    let output_layer_result = forward(hidden_layer_result.clone(), output_layer.clone());

    println!("Prediction: {:?}", output_layer_result);
}

fn train(input: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, input_layer:Vec<Vec<f64>>, hidden_layer:Vec<Vec<f64>>, output_layer:Vec<Vec<f64>>) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let input_layer_result = forward(input.clone(), input_layer.clone());
    let hidden_layer_result = forward(input_layer_result.clone(), hidden_layer.clone());
    let output_layer_result = forward(hidden_layer_result.clone(), output_layer.clone());

    let derivative = |layer_result: Vec<Vec<f64>>| -> Vec<Vec<f64>> {
        layer_result
        .iter()
        .map(|row| { row.iter().map(Utils::sigmoid_derivative).collect() })
        .collect()
    };

    let output_layer_derivative = derivative(output_layer_result.clone());
    let output_layer_error = 
        Utils::matrix_subtract(Utils::matrix_transpose(targets.clone()), output_layer_result.clone());
    let output_layer_delta = 
        Utils::matrix_naive_multiply(output_layer_derivative.clone(), output_layer_error.clone());

    // Back Propagation: Hidden Layer + Input Layer
    let output_layer_factor = 
        Utils::matrix_multiply(output_layer_delta.clone(), output_layer.clone());

    let hidden_layer_derivative = derivative(hidden_layer_result.clone());
    let hidden_layer_delta = 
        Utils::matrix_naive_multiply(hidden_layer_derivative.clone(), output_layer_factor.clone());

    let input_layer_derivative = derivative(input_layer_result.clone());
    let input_layer_delta = Utils::matrix_naive_multiply(input_layer_derivative.clone(), output_layer_factor.clone());

    // Back Propagation: Adjustment
    let output_layer_adjustment = 
        Utils::matrix_multiply(Utils::matrix_transpose(hidden_layer_result.clone()), output_layer_delta.clone());
    let output_layer_adjusted = 
        Utils::matrix_transpose(
            Utils::matrix_add(Utils::matrix_transpose(output_layer.clone()), output_layer_adjustment.clone())
        );
    let hidden_layer_adjustment = 
        Utils::matrix_multiply(Utils::matrix_transpose(input_layer_result.clone()), hidden_layer_delta.clone());
    let hidden_layer_adjusted =
        Utils::matrix_transpose(
            Utils::matrix_add(Utils::matrix_transpose(hidden_layer.clone()), hidden_layer_adjustment.clone())
        );
    let input_layer_adjustment =
        Utils::matrix_multiply(Utils::matrix_transpose(input.clone()), input_layer_delta.clone());
    let input_layer_adjusted =
        Utils::matrix_transpose(
            Utils::matrix_add(Utils::matrix_transpose(input_layer.clone()), input_layer_adjustment.clone())
        );

    (input_layer_adjusted, hidden_layer_adjusted, output_layer_adjusted)
}

fn forward(input: Vec<Vec<f64>>, layer:Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    Utils::matrix_multiply(input, Utils::matrix_transpose(layer))
    .iter()
    .map(|row| { row.iter().map(Utils::sigmoid).collect() })
    .collect()
}
