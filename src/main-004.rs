use std::cell::RefCell;
use std::rc::Rc;

struct Calc {}

impl Calc {
    fn sigmoid(x: &f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    fn sigmoid_derivative(x: &f64) -> f64 {
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
            .map(|row| { row.iter().map(Calc::sigmoid_derivative).collect() })
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
    fn new(input_shape: (usize, usize), hidden_shape: (usize, usize), output_shape: (usize, usize)) -> Self {
        let input_layer = Layer {
            matrix: Matrix { data: vec![vec![0.0; input_shape.1]; input_shape.0] },
            forwarded: None,
        };

        let hidden_layer = Layer {
            matrix: Matrix { data: vec![vec![0.0; hidden_shape.1]; hidden_shape.0] },
            forwarded: None,
        };

        let output_layer = Layer {
            matrix: Matrix { data: vec![vec![0.0; output_shape.1]; output_shape.0] },
            forwarded: None,
        };

        Self {
            input_layer: Rc::new(RefCell::new(input_layer)),
            hidden_layer: Rc::new(RefCell::new(hidden_layer)),
            output_layer: Rc::new(RefCell::new(output_layer)),
        }
    }

    fn train(&self, input: Matrix, targets: Matrix) {
        self.backward_propagate(
            self.forward_propagate(input.clone()),
            input, 
            targets
        );
    }

    fn predict(&self, input: Matrix) -> Matrix {
        let (_, _, output_forwarded) = self.forward_propagate(input);

        output_forwarded
    }

    fn forward_propagate(&self, input: Matrix) -> (Matrix, Matrix, Matrix) {
        let input_forwarded = {
            self.forward(input.clone(), self.input_layer.clone());
            self.input_layer.borrow().forwarded.clone().unwrap()
        };

        let hidden_forwarded = {
            self.forward(input_forwarded.clone(), self.hidden_layer.clone());
            self.hidden_layer.borrow().forwarded.clone().unwrap()
        };

        let output_forwarded = {
            self.forward(hidden_forwarded.clone(), self.output_layer.clone());
            self.output_layer.borrow().forwarded.clone().unwrap()
        };

        (input_forwarded, hidden_forwarded, output_forwarded)
    }

    fn backward_propagate(&self, forward: (Matrix, Matrix, Matrix), input: Matrix, targets: Matrix) {
        let (input_forwarded, hidden_forwarded, output_forwarded) = forward;

        let error = Matrix::subtract(targets.transpose(), output_forwarded.clone());

        let factor = Matrix::multiply(
            Matrix::naive_multiply(output_forwarded.clone(), error.clone()),
            self.output_layer.borrow().matrix.clone()
        );

        self.backward(
            hidden_forwarded.clone(),
            self.output_layer.clone(),
            Matrix::naive_multiply(output_forwarded.clone().derivative(), error.clone())
        );

        self.backward(
            input_forwarded.clone(),
            self.hidden_layer.clone(),
            Matrix::naive_multiply(hidden_forwarded.clone().derivative(), factor.clone())
        );

        self.backward(
            input.clone(),
            self.input_layer.clone(),
            Matrix::naive_multiply(input_forwarded.clone().derivative(), factor.clone())
        );
    }

    fn forward(&self, input: Matrix, layer: Rc<RefCell<Layer>>) {
        let mut layer_borrow = layer.borrow_mut();

        layer_borrow.forwarded = Some(
            Matrix::new(
                Matrix::multiply(input, layer_borrow.matrix.transpose())
                .data.iter().map(|row| { row.iter().map(Calc::sigmoid).collect() }).collect()
            )
        )
    }

    fn backward(&self, input: Matrix, layer: Rc<RefCell<Layer>>, delta: Matrix) {
        let mut layer_borrow = layer.borrow_mut();

        let adjustment = Matrix::multiply(input.transpose(), delta.clone());
        layer_borrow.matrix = Matrix::add(layer_borrow.matrix.clone(), adjustment.transpose());
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

    let network = NeuralNetwork::new((4, 3), (4, 4), (1, 4));

    for _ in 0..3 {
        network.train(input.clone(), targets.clone());
    }

    println!("Predictions using input [[1.0, 1.0, 0.0]]: {:?}", network.predict(Matrix { data: vec![vec![1.0, 1.0, 0.0]] }));
}
