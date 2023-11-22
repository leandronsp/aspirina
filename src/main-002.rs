mod matrix;
mod calc;

use crate::calc::Calc;
use crate::matrix::Matrix;

fn main() {
    let mut input_layer = Matrix {
        data: vec![vec![1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0]],
    };

    let mut hidden_layer = Matrix {
        data: vec![vec![1.0, 1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0, 1.0]],
    };

    let mut output_layer = Matrix {
        data: vec![vec![1.0, 1.0, 1.0, 1.0]],
    };

    let input = Matrix { data: vec![
        vec![0.0, 0.0, 1.0], vec![0.0, 1.0, 1.0], vec![1.0, 0.0, 1.0], vec![0.0, 1.0, 0.0], 
        vec![1.0, 0.0, 0.0], vec![1.0, 1.0, 1.0], vec![0.0, 0.0, 0.0]]
    };

    let targets = Matrix { data: vec![vec![0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0]] };

    let times = 100;

    println!("Going to train the network {} times", times);

    for _ in 0..times {
        let result = train(input.clone(), targets.clone(), input_layer, hidden_layer, output_layer);

        input_layer = result.0;
        hidden_layer = result.1;
        output_layer = result.2;
    }

    // Predict
    //
    let input = Matrix { data: vec![vec![1.0, 1.0, 0.0]] };
    println!("Going to predict the following input: {:?}", input);

    let input_layer_result = forward(input, input_layer);
    let hidden_layer_result = forward(input_layer_result, hidden_layer);
    let output_layer_result = forward(hidden_layer_result, output_layer);

    println!("Prediction: {:?}", output_layer_result);
}

fn train(input: Matrix, targets: Matrix, input_layer: Matrix, 
         hidden_layer: Matrix, output_layer: Matrix) -> (Matrix, Matrix, Matrix) {

    let input_layer_result = forward(input.clone(), input_layer.clone());
    let hidden_layer_result = forward(input_layer_result.clone(), hidden_layer.clone());
    let output_layer_result = forward(hidden_layer_result.clone(), output_layer.clone());

    let output_layer_error = Matrix::subtract(targets.transpose(), output_layer_result.clone());
    let output_layer_delta = Matrix::naive_multiply(
        output_layer_result.derivative(), 
        output_layer_error
    );

    // Back Propagation: Hidden Layer + Input Layer
    let output_layer_factor = Matrix::multiply(output_layer_delta.clone(), output_layer.clone());

    let hidden_layer_delta = Matrix::naive_multiply(
        hidden_layer_result.derivative(), 
        output_layer_factor.clone()
    );

    let input_layer_delta = Matrix::naive_multiply(
        input_layer_result.derivative(), 
        output_layer_factor.clone()
    );

    // Back Propagation: Adjustment
    let output_layer_adjustment = Matrix::multiply(hidden_layer_result.transpose(), output_layer_delta);
    let output_layer_adjusted = Matrix::add(output_layer.transpose(), output_layer_adjustment).transpose();

    let hidden_layer_adjustment = Matrix::multiply(input_layer_result.transpose(), hidden_layer_delta);
    let hidden_layer_adjusted = Matrix::add(hidden_layer.transpose(), hidden_layer_adjustment).transpose();

    let input_layer_adjustment = Matrix::multiply(input.transpose(), input_layer_delta);
    let input_layer_adjusted = Matrix::add(input_layer.transpose(), input_layer_adjustment).transpose();

    (input_layer_adjusted, hidden_layer_adjusted, output_layer_adjusted)
}

fn forward(input: Matrix, layer: Matrix) -> Matrix {
    let result = 
        Matrix::multiply(input, layer.transpose())
        .data
        .iter()
        .map(|row| { row.iter().map(Calc::sigmoid).collect() })
        .collect();

    Matrix { data: result }
}
