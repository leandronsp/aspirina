use std::cell::RefCell;
use std::rc::Rc;

mod calc {
    pub fn sigmoid(x: &f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn sigmoid_derivative(x: &f64) -> f64 {
        x * (1.0 - x)
    }

    pub fn tanh(x: &f64) -> f64 {
        x.tanh()
    }

    pub fn tanh_derivative(x: &f64) -> f64 {
        1.0 - x.tanh().powi(2)
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
    layers: Vec<Rc<RefCell<Layer>>>,
}

impl NeuralNetwork {
    fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers: layers.into_iter().map(|layer| Rc::new(RefCell::new(layer))).collect(),
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
        self.forward_propagation(input).last().unwrap().clone()
    }

    fn forward_propagation(&self, input: Matrix) -> Vec<Matrix> {
        let mut forwarded = Vec::new();
        let mut input = input;

        for layer in &self.layers {
            self.apply_activation(input.clone(), layer.clone());
            input = layer.borrow().forwarded.clone().unwrap();
            forwarded.push(input.clone());
        }

        forwarded
    }

    fn backward_propagation(&self, forwarded: Vec<Matrix>, input: Matrix, targets: Matrix) {
        let mut error = Matrix::subtract(targets.transpose(), forwarded.last().unwrap().clone());

        for (idx, layer) in self.layers.iter().enumerate().rev() {
            let input_to_layer = if idx == 0 { input.clone() } else { forwarded[idx - 1].clone() };

            let delta = Matrix::naive_multiply(
                forwarded[idx].clone().derivative(),
                error.clone()
            );

            if idx > 0 {
                error = Matrix::multiply(
                    delta.clone(),
                    layer.borrow().matrix.clone()
                );
            }

            self.adjust(input_to_layer, layer.clone(), delta);
        }
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
    let layers = vec![
        // Input Layer
        Layer {
            matrix: Matrix {
                data: vec![
                    vec![-0.16595599, -0.70648822, -0.20646505],
                    vec![0.44064899, -0.81532281, 0.07763347],
                    vec![-0.99977125, -0.62747958, -0.16161097],
                    vec![-0.39533485, -0.30887855, 0.370439],
                ],
            },
            forwarded: None,
        },
        // Hidden Layers
        Layer {
            matrix: Matrix {
                data: vec![
                    vec![-0.16595599, -0.70648822, -0.20646505, -0.34093502],
                    vec![0.44064899, -0.81532281, 0.07763347, 0.44093502],
                    vec![-0.99977125, -0.62747958, -0.16161097, 0.14093502],
                    vec![-0.39533485, -0.30887855, 0.370439, -0.54093502],
                ],
            },
            forwarded: None,
        },
        Layer {
            matrix: Matrix {
                data: vec![
                    vec![-0.23456789, 0.87654321, -0.34567891, 0.12345678],
                    vec![0.98765432, -0.45678912, 0.56789123, -0.67891234],
                    vec![-0.89123456, 0.78912345, -0.43219876, 0.32198765],
                    vec![0.65432109, -0.54321098, 0.21098765, -0.10987654],
                ],
            },
            forwarded: None,
        },
        // Output Layer
        Layer {
            matrix: Matrix {
                data: vec![
                    vec![-0.5910955, 0.75623487, -0.94522481, 0.64093502],
                ],
            },
            forwarded: None,
        },
    ];

    let network = NeuralNetwork::new(layers);

    let input = Matrix { data: vec![
        vec![0.0, 0.0, 1.0], 
        vec![0.0, 0.0, 0.0],
        vec![0.0, 1.0, 1.0], 
        vec![0.0, 1.0, 0.0], 
        vec![1.0, 0.0, 1.0], 
        vec![1.0, 0.0, 0.0], 
        vec![0.5, 0.5, 0.0],
        vec![0.5, 0.5, 1.0],
    ]};

    let targets = Matrix { data: vec![vec![0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0]] };

    for idx in 0..1_000 {
        println!("Iteration: {}", idx);
        network.train(input.clone(), targets.clone());
    }

    println!(
        "predict([[1.0, 1.0, 0.0]]) = {:?}", 
        network.predict(Matrix { data: vec![vec![1.0, 1.0, 0.0]] }).data
    );
}
