use std::cell::RefCell;
use std::rc::Rc;

mod calc {
    pub fn sigmoid(x: &f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn sigmoid_derivative(x: &f64) -> f64 {
        x * (1.0 - x)
    }
}

#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(data: Vec<Vec<f64>>) -> Self {
        Self { data }
    }

    fn transpose(&self) -> Self {
        let rows = self.data.len();
        let cols = self.data[0].len();

        let mut result = vec![vec![self.data[0][0]; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                result[j][i] = self.data[i][j];
            }
        }

        Self { data: result }
    }

    fn element_wise_operation(m1: Self, m2: Self, op: fn(f64, f64) -> f64) -> Self {
        let m1_rows_len = m1.data.len();
        let m2_rows_len = m2.data.len();
        let m1_cols_len = m1.data[0].len();
        let m2_cols_len = m2.data[0].len();

        if m1_rows_len != m2_rows_len || m1_cols_len != m2_cols_len {
            panic!("Incompatible dimensions")
        }

        let mut result = vec![vec![0.0; m2_cols_len]; m1_rows_len];

        for i in 0..m1_rows_len {
            for j in 0..m2_cols_len {
                result[i][j] += op(m1.data[i][j], m2.data[i][j]);
            }
        }

        Self { data: result }
    }

    fn add(m1: Self, m2: Self) -> Self {
        Self::element_wise_operation(m1, m2, |a, b| { a + b })
    }

    fn subtract(m1: Self, m2: Self) -> Self {
        Self::element_wise_operation(m1, m2, |a, b| { a - b })
    }

    fn naive_multiply(m1: Self, m2: Self) -> Self {
        Self::element_wise_operation(m1, m2, |a, b| { a * b })
    }

    fn multiply(m1: Self, m2: Self) -> Self {
        let m1_rows_len = m1.data.len();
        let m2_rows_len = m2.data.len();
        let m1_cols_len = m1.data[0].len();
        let m2_cols_len = m2.data[0].len();

        if m1_cols_len != m2_rows_len {
            panic!("Incompatible dimensions: {:?} and {:?}", m1, m2);
        }

        let mut result = vec![vec![0.0; m2_cols_len]; m1_rows_len];

        for i in 0..m1_rows_len {
            for j in 0..m2_cols_len {
                for k in 0..m1_cols_len {
                    result[i][j] += m1.data[i][k] * m2.data[k][j];
                }
            }
        }

        Self { data: result }
    }

    fn derivative(&self) -> Self {
        let result = 
            self
            .data
            .iter()
            .map(|row| { row.iter().map(calc::sigmoid_derivative).collect() })
            .collect();

        Self { data: result }
    }
}

#[derive(Debug, Clone)]
struct Layer {
    matrix: Matrix,
    forwarded: Option<Matrix>,
}

#[derive(Debug, Clone)]
struct NeuralNetwork {
    input_layer: Rc<RefCell<Layer>>,
    hidden_layer: Rc<RefCell<Layer>>,
    output_layer: Rc<RefCell<Layer>>,
}

impl NeuralNetwork {
    fn setup() -> Self {
        let input_layer = Layer {
            matrix: Matrix { data: vec![
                vec![-0.16595599, -0.70648822, -0.20646505],
                vec![0.44064899, -0.81532281, 0.07763347],
                vec![-0.99977125, -0.62747958, -0.16161097],
                vec![-0.39533485, -0.30887855, 0.370439]
            ]},
            forwarded: None,
        };

        let hidden_layer = Layer {
            matrix: Matrix { data: vec![
                vec![-0.16595599, -0.70648822, -0.20646505, -0.34093502],
                vec![0.44064899, -0.81532281, 0.07763347, 0.44093502],
                vec![-0.99977125, -0.62747958, -0.16161097, 0.14093502],
                vec![-0.39533485, -0.30887855, 0.370439, -0.54093502]
            ]},
            forwarded: None,
        };

        let output_layer = Layer {
            matrix: Matrix { data: vec![
                vec![-0.5910955, 0.75623487, -0.94522481, 0.64093502]
            ]},
            forwarded: None,
        };

        Self {
            input_layer: Rc::new(RefCell::new(input_layer)),
            hidden_layer: Rc::new(RefCell::new(hidden_layer)),
            output_layer: Rc::new(RefCell::new(output_layer)),
        }
    }

    fn train(&self, input: Matrix, targets: Matrix) {
        self.backward_propagation(
            self.forward_propagation(input.clone()),
            input, 
            targets
        );
    }

    fn predict(&self, input: Matrix) -> Matrix {
        let (_, _, output_forwarded) = self.forward_propagation(input);

        output_forwarded
    }

    fn forward_propagation(&self, input: Matrix) -> (Matrix, Matrix, Matrix) {
        let input_forwarded = {
            self.apply_activation(input.clone(), self.input_layer.clone());
            self.input_layer.borrow().forwarded.clone().unwrap()
        };

        let hidden_forwarded = {
            self.apply_activation(input_forwarded.clone(), self.hidden_layer.clone());
            self.hidden_layer.borrow().forwarded.clone().unwrap()
        };

        let output_forwarded = {
            self.apply_activation(hidden_forwarded.clone(), self.output_layer.clone());
            self.output_layer.borrow().forwarded.clone().unwrap()
        };

        (input_forwarded, hidden_forwarded, output_forwarded)
    }

    fn backward_propagation(&self, forward: (Matrix, Matrix, Matrix), input: Matrix, targets: Matrix) {
        let (input_forwarded, hidden_forwarded, output_forwarded) = forward;

        let error = Matrix::subtract(targets.transpose(), output_forwarded.clone());

        let factor = Matrix::multiply(
            Matrix::naive_multiply(output_forwarded.clone().derivative(), error.clone()),
            self.output_layer.borrow().matrix.clone()
        );

        self.adjust(
            hidden_forwarded.clone(),
            self.output_layer.clone(),
            Matrix::naive_multiply(output_forwarded.clone().derivative(), error.clone())
        );

        self.adjust(
            input_forwarded.clone(),
            self.hidden_layer.clone(),
            Matrix::naive_multiply(hidden_forwarded.clone().derivative(), factor.clone())
        );

        self.adjust(
            input.clone(),
            self.input_layer.clone(),
            Matrix::naive_multiply(input_forwarded.clone().derivative(), factor.clone())
        );
    }

    fn apply_activation(&self, input: Matrix, layer: Rc<RefCell<Layer>>) {
        let mut layer_borrow = layer.borrow_mut();

        layer_borrow.forwarded = Some(
            Matrix::new(
                Matrix::multiply(input, layer_borrow.matrix.transpose())
                .data.iter().map(|row| { row.iter().map(calc::sigmoid).collect() }).collect()
            )
        )
    }

    fn adjust(&self, input: Matrix, layer: Rc<RefCell<Layer>>, delta: Matrix) {
        let adjustment = Matrix::multiply(input.transpose(), delta.clone());

        let mut layer_borrow = layer.borrow_mut();

        layer_borrow.matrix = 
            Matrix::add(layer_borrow.matrix.clone(), adjustment.transpose())
    }
}

fn main() {
    let input = Matrix { data: vec![
        vec![0.0, 0.0, 1.0], 
        vec![0.0, 1.0, 1.0], 
        vec![1.0, 0.0, 1.0], 
        vec![0.0, 1.0, 0.0], 
        vec![1.0, 0.0, 0.0], 
        vec![1.0, 1.0, 1.0], 
        vec![0.0, 0.0, 0.0]
    ]};

    let targets = Matrix { data: vec![vec![0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0]] };

    let network = NeuralNetwork::setup();

    for _ in 0..1_000 {
        network.train(input.clone(), targets.clone());
    }

    println!(
        "predict([[1.0, 1.0, 0.0]]) = {:?}", 
        network.predict(Matrix { data: vec![vec![1.0, 1.0, 0.0]] }).data
    );
}
