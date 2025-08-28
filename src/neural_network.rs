use crate::calc::Calc;
use crate::layer::Layer;
use crate::matrix::Matrix;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    layers: Vec<Rc<RefCell<Layer>>>,
}

impl NeuralNetwork {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers: layers
                .into_iter()
                .map(|layer| Rc::new(RefCell::new(layer)))
                .collect(),
        }
    }

    pub fn train(&self, input: Matrix, targets: Matrix) {
        let forwarded = self.forward_propagation(input.clone());
        self.back_propagation(forwarded, input, targets);
    }

    pub fn predict(&self, input: Matrix) -> Matrix {
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

    fn back_propagation(&self, forwarded: Vec<Matrix>, input: Matrix, targets: Matrix) {
        let mut error = Matrix::subtract(targets.transpose(), forwarded.last().unwrap().clone());

        for (idx, layer) in self.layers.iter().enumerate().rev() {
            let input_to_layer = if idx == 0 {
                input.clone()
            } else {
                forwarded[idx - 1].clone()
            };

            let delta = Matrix::naive_multiply(forwarded[idx].clone().derivative(), error.clone());

            if idx > 0 {
                error = Matrix::multiply(delta.clone(), layer.borrow().matrix.clone());
            }

            self.adjust(input_to_layer, layer.clone(), delta);
        }
    }

    fn apply_activation(&self, input: Matrix, layer: Rc<RefCell<Layer>>) {
        let mut layer_borrow = layer.borrow_mut();

        layer_borrow.forwarded = Some(Matrix::new(
            Matrix::multiply(input, layer_borrow.matrix.transpose())
                .data
                .iter()
                .map(|row| row.iter().map(Calc::sigmoid).collect())
                .collect(),
        ))
    }

    fn adjust(&self, input: Matrix, layer: Rc<RefCell<Layer>>, delta: Matrix) {
        let adjustment = Matrix::multiply(input.transpose(), delta.clone());
        let mut layer_borrow = layer.borrow_mut();
        layer_borrow.matrix =
            Matrix::matrix_add(layer_borrow.matrix.clone(), adjustment.transpose())
    }
}
