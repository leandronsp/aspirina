mod matrix;
mod calc;

use crate::calc::Calc;
use crate::matrix::Matrix;

fn main() {
    let layers = vec![
        Matrix { // Input Layer (4x3)
            data: vec![
                vec![1.0, 1.0, 1.0], 
                vec![1.0, 1.0, 1.0], 
                vec![1.0, 1.0, 1.0], 
                vec![1.0, 1.0, 1.0]
            ],
        },
        Matrix { // Hidden Layer (4x4)
            data: vec![
                vec![1.0, 1.0, 1.0, 1.0], 
                vec![1.0, 1.0, 1.0, 1.0],
                vec![1.0, 1.0, 1.0, 1.0], 
                vec![1.0, 1.0, 1.0, 1.0]
            ],
        },
        Matrix { data: vec![vec![1.0, 1.0, 1.0, 1.0]] } // Output Layer (1x4)
    ];

    let input = Matrix { data: vec![
        vec![0.0, 0.0, 1.0], vec![0.0, 1.0, 1.0], vec![1.0, 0.0, 1.0], vec![0.0, 1.0, 0.0], 
        vec![1.0, 0.0, 0.0], vec![1.0, 1.0, 1.0], vec![0.0, 0.0, 0.0]]
    };

    let targets = Matrix { data: vec![vec![0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0]] };

    let times = 100;

    println!("Going to train the network {} times", times);

    let new_layers = train(input.clone(), targets.clone(), layers, times);

    // Predict
    //
    let input = Matrix { data: vec![vec![1.0, 1.0, 0.0]] };
    println!("Going to predict the following input: {:?}", input);

    let output_layer_result = forward_propagation(input, new_layers.clone())[2].clone();
    println!("Prediction: {:?}", output_layer_result);
}

fn train(input: Matrix, targets: Matrix, layers: Vec<Matrix>, times: usize) -> Vec<Matrix> {
    let mut updated_layers = layers.clone();

    for _ in 0..times {
        let forwarded = forward_propagation(input.clone(), updated_layers.clone());
        updated_layers = back_propagation(input.clone(), targets.clone(), updated_layers.clone(), forwarded.clone());
    }

    updated_layers
}

fn output_layer_delta_factor(targets: Matrix, output_layer: Matrix, output_layer_forwarded: Matrix) -> (Matrix, Matrix) {
    let output_layer_error = Matrix::subtract(targets.transpose(), output_layer_forwarded.clone());
    let output_layer_delta = Matrix::naive_multiply(
        output_layer_forwarded.derivative(), 
        output_layer_error
    );

    let output_layer_factor = Matrix::multiply(
        output_layer_delta.clone(), 
        output_layer.clone()
    );

    (output_layer_delta, output_layer_factor)
}

fn back_propagation(input: Matrix, targets: Matrix, layers: Vec<Matrix>, forwarded: Vec<Matrix>) -> Vec<Matrix> {
    let mut input_layer: Matrix = layers[0].clone();
    let mut hidden_layer: Matrix = layers[1].clone();
    let mut output_layer: Matrix = layers[2].clone();

    let input_layer_forwarded = forwarded[0].clone();
    let hidden_layer_forwarded = forwarded[1].clone();
    let output_layer_forwarded = forwarded[2].clone();

    let (output_layer_delta, output_layer_factor) = output_layer_delta_factor(
        targets.clone(), 
        output_layer.clone(), 
        output_layer_forwarded.clone()
    );

    // Back Propagation: Hidden Layer + Input Layer
    let hidden_layer_delta = Matrix::naive_multiply(
        hidden_layer_forwarded.derivative(), 
        output_layer_factor.clone()
    );

    let input_layer_delta = Matrix::naive_multiply(
        input_layer_forwarded.derivative(), 
        output_layer_factor.clone()
    );

    // Back Propagation: Adjustment
    let output_layer_adjustment = Matrix::multiply(hidden_layer_forwarded.transpose(), output_layer_delta);
    output_layer = Matrix::add(output_layer.transpose(), output_layer_adjustment).transpose();

    let hidden_layer_adjustment = Matrix::multiply(input_layer_forwarded.transpose(), hidden_layer_delta);
    hidden_layer = Matrix::add(hidden_layer.transpose(), hidden_layer_adjustment).transpose();

    let input_layer_adjustment = Matrix::multiply(input.transpose(), input_layer_delta);
    input_layer = Matrix::add(input_layer.transpose(), input_layer_adjustment).transpose();

    vec![input_layer, hidden_layer, output_layer]
}

fn forward_propagation(input: Matrix, layers: Vec<Matrix>) -> Vec<Matrix> {
    let mut result = Vec::with_capacity(layers.len());

    layers.iter().fold(input.clone(), |acc, layer| {
        let layer_data: Matrix = forward_layer(acc, layer.clone());
        result.push(layer_data.clone());
        layer_data
    });

    result
}

fn forward_layer(input: Matrix, layer: Matrix) -> Matrix {
    Matrix { 
        data: Matrix::multiply(input.clone(), layer.transpose())
                .data
                .iter()
                .map(|row| { row.iter().map(Calc::sigmoid).collect() })
                .collect()
    }
}
