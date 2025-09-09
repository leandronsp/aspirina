//! Neural Network Implementation
//!
//! This module provides a feedforward neural network implementation with backpropagation
//! training capabilities. The neural network uses sigmoid activation functions and
//! gradient descent for learning.
//!
//! # Neural Networks
//!
//! A neural network is a computational model inspired by biological neural networks.
//! It consists of interconnected layers of neurons (nodes) that process information
//! through weighted connections. This implementation supports:
//!
//! - **Feedforward architecture**: Information flows from input to output layers
//! - **Backpropagation training**: Gradient-based learning algorithm
//! - **Sigmoid activation**: Non-linear activation function for hidden and output layers
//! - **Multi-layer support**: Configurable number of hidden layers
//!
//! # Backpropagation Algorithm
//!
//! Backpropagation is a supervised learning algorithm that trains the network by:
//! 1. **Forward pass**: Computing predictions by propagating inputs through layers
//! 2. **Error calculation**: Measuring difference between predictions and targets
//! 3. **Backward pass**: Propagating errors back through the network
//! 4. **Weight updates**: Adjusting weights to minimize prediction errors
//!
//! # Example Usage
//!
//! ```rust
//! # use aspirina::neural_network::NeuralNetwork;
//! # use aspirina::layer::Layer;
//! # use aspirina::matrix::Matrix;
//! // Create a network with input(2) -> hidden(3) -> output(1) architecture
//! let layers = vec![
//!     Layer::new(Matrix::new(vec![vec![0.1, 0.2], vec![0.3, 0.4], vec![0.5, 0.6]])),
//!     Layer::new(Matrix::new(vec![vec![0.7, 0.8, 0.9]])),
//! ];
//! let network = NeuralNetwork::new(layers);
//!
//! // Training data for XOR gate
//! let input = Matrix::new(vec![vec![0.0, 0.0], vec![0.0, 1.0], vec![1.0, 0.0], vec![1.0, 1.0]]);
//! let targets = Matrix::new(vec![vec![0.0, 1.0, 1.0, 0.0]]);
//!
//! // Train the network
//! for _ in 0..10 {
//!     network.train(input.clone(), targets.clone());
//! }
//!
//! // Make predictions
//! let prediction = network.predict(Matrix::new(vec![vec![1.0, 0.0]]));
//! ```

use crate::calc::Calc;
use crate::layer::Layer;
use crate::matrix::Matrix;
use std::cell::RefCell;
use std::rc::Rc;

/// A feedforward neural network with backpropagation training capabilities.
///
/// The `NeuralNetwork` struct represents a multi-layer neural network that can learn
/// complex patterns through supervised training. It uses reference counting (`Rc`) and
/// interior mutability (`RefCell`) to allow shared ownership of layers while maintaining
/// mutability during training.
///
/// # Architecture
///
/// - **Layers**: Sequential arrangement of neural layers from input to output
/// - **Weights**: Each layer contains a weight matrix defining neuron connections
/// - **Activation**: Sigmoid activation function applied to all layer outputs
/// - **Training**: Gradient descent with backpropagation for weight optimization
///
/// # Memory Management
///
/// Uses `Rc<RefCell<Layer>>` for:
/// - **Shared ownership**: Multiple references to the same layer data
/// - **Interior mutability**: Modify layer weights during training
/// - **Efficient cloning**: Low-cost network duplication for parallel training
///
/// # Supported Architectures
///
/// - **Single layer**: Direct input-to-output mapping (linear problems)
/// - **Multi-layer**: Hidden layers for non-linear problem solving
/// - **Deep networks**: Multiple hidden layers for complex pattern recognition
///
/// # Fields
///
/// - `layers`: Vector of neural network layers wrapped in smart pointers
#[derive(Debug, Clone)]
pub struct NeuralNetwork {
    /// The neural network layers, each containing weights and optional forwarded results.
    /// Uses `Rc<RefCell<>>` for shared ownership and interior mutability during training.
    layers: Vec<Rc<RefCell<Layer>>>,
}

impl NeuralNetwork {
    /// Creates a new neural network from a vector of layers.
    ///
    /// This constructor takes ownership of the provided layers and wraps them in
    /// `Rc<RefCell<>>` smart pointers to enable shared ownership and interior mutability
    /// required for the training process.
    ///
    /// # Parameters
    ///
    /// * `layers` - A vector of `Layer` structs defining the network architecture.
    ///   Each layer contains a weight matrix where:
    ///   - Rows represent neurons in the current layer
    ///   - Columns represent inputs from the previous layer (or input data for first layer)
    ///
    /// # Returns
    ///
    /// A new `NeuralNetwork` instance ready for training and prediction.
    ///
    /// # Architecture Guidelines
    ///
    /// For a network with input size `n`, hidden layer size `h`, and output size `o`:
    /// - First layer: `h × n` weight matrix (hidden neurons × input features)
    /// - Output layer: `o × h` weight matrix (output neurons × hidden neurons)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aspirina::neural_network::NeuralNetwork;
    /// # use aspirina::layer::Layer;
    /// # use aspirina::matrix::Matrix;
    /// // Create a 2-3-1 network (2 inputs, 3 hidden, 1 output)
    /// let layers = vec![
    ///     // Hidden layer: 3 neurons, 2 inputs each
    ///     Layer::new(Matrix::new(vec![
    ///         vec![0.1, 0.2],
    ///         vec![0.3, 0.4],
    ///         vec![0.5, 0.6],
    ///     ])),
    ///     // Output layer: 1 neuron, 3 inputs from hidden layer
    ///     Layer::new(Matrix::new(vec![
    ///         vec![0.7, 0.8, 0.9],
    ///     ])),
    /// ];
    /// let network = NeuralNetwork::new(layers);
    /// ```
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers: layers
                .into_iter()
                .map(|layer| Rc::new(RefCell::new(layer)))
                .collect(),
        }
    }

    /// Trains the neural network using a single training batch.
    ///
    /// This method performs one complete training iteration using the backpropagation
    /// algorithm. It combines forward propagation (to compute predictions) and backward
    /// propagation (to update weights based on prediction errors).
    ///
    /// # Training Process
    ///
    /// 1. **Forward Propagation**: Input data flows through each layer, applying weights
    ///    and sigmoid activation to produce predictions
    /// 2. **Error Calculation**: Compute the difference between predictions and target values
    /// 3. **Backward Propagation**: Calculate gradients and propagate errors backward
    /// 4. **Weight Updates**: Adjust layer weights to minimize prediction errors
    ///
    /// # Parameters
    ///
    /// * `input` - Input data matrix where:
    ///   - Each row represents a training sample
    ///   - Each column represents an input feature
    ///   - Shape: `[batch_size, input_features]`
    ///
    /// * `targets` - Target output matrix where:
    ///   - Each row represents the expected output for corresponding input
    ///   - Each column represents an output feature  
    ///   - Shape: `[batch_size, output_features]`
    ///
    /// # Training Strategy
    ///
    /// For effective training, call this method repeatedly with:
    /// - Multiple epochs (thousands of iterations)
    /// - Consistent input/target data shapes
    /// - Properly normalized input values (typically 0.0 to 1.0)
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aspirina::neural_network::NeuralNetwork;
    /// # use aspirina::layer::Layer;
    /// # use aspirina::matrix::Matrix;
    /// // Create a simple network
    /// let layers = vec![
    ///     Layer::new(Matrix::new(vec![vec![0.1, 0.2], vec![0.3, 0.4]])),
    ///     Layer::new(Matrix::new(vec![vec![0.5, 0.6]])),
    /// ];
    /// let network = NeuralNetwork::new(layers);
    ///
    /// // XOR training data
    /// let input = Matrix::new(vec![
    ///     vec![0.0, 0.0],
    ///     vec![0.0, 1.0],
    ///     vec![1.0, 0.0],
    ///     vec![1.0, 1.0],
    /// ]);
    /// let targets = Matrix::new(vec![vec![0.0, 1.0, 1.0, 0.0]]);
    ///
    /// // Train for multiple epochs
    /// for _ in 0..10 {
    ///     network.train(input.clone(), targets.clone());
    /// }
    /// ```
    pub fn train(&self, input: Matrix, targets: Matrix) {
        let forwarded = self.forward_propagation(input.clone());
        self.back_propagation(forwarded, input, targets);
    }

    /// Makes predictions on new input data using the trained neural network.
    ///
    /// This method performs forward propagation through all network layers without
    /// updating weights. It's used for inference after the network has been trained.
    ///
    /// # Process
    ///
    /// 1. **Forward Pass**: Input flows through each layer sequentially
    /// 2. **Weight Application**: Each layer applies its learned weight matrix
    /// 3. **Activation**: Sigmoid function applied to layer outputs
    /// 4. **Final Output**: Returns the output from the last layer
    ///
    /// # Parameters
    ///
    /// * `input` - Input data matrix where:
    ///   - Each row represents a sample to predict
    ///   - Each column represents an input feature
    ///   - Shape must match the network's input layer: `[samples, input_features]`
    ///
    /// # Returns
    ///
    /// A `Matrix` containing predictions where:
    /// - Each row corresponds to the prediction for the input sample in the same row
    /// - Each column represents an output feature
    /// - Values are sigmoid-activated (typically between 0.0 and 1.0)
    /// - Shape: `[samples, output_features]`
    ///
    /// # Usage Notes
    ///
    /// - Input data should be normalized using the same scale as training data
    /// - For binary classification, apply threshold (e.g., 0.5) to outputs
    /// - For multi-class problems, use argmax to find the highest probability class
    ///
    /// # Example
    ///
    /// ```rust
    /// # use aspirina::neural_network::NeuralNetwork;
    /// # use aspirina::layer::Layer;
    /// # use aspirina::matrix::Matrix;
    /// // Create and train a simple network
    /// let layers = vec![
    ///     Layer::new(Matrix::new(vec![vec![0.1, 0.2], vec![0.3, 0.4]])),
    ///     Layer::new(Matrix::new(vec![vec![0.5, 0.6]])),
    /// ];
    /// let network = NeuralNetwork::new(layers);
    ///
    /// // Single prediction
    /// let input = Matrix::new(vec![vec![1.0, 0.0]]);
    /// let prediction = network.predict(input);
    /// println!("Prediction: {:.3}", prediction.data[0][0]);
    ///
    /// // Batch predictions
    /// let batch_input = Matrix::new(vec![
    ///     vec![0.0, 0.0],
    ///     vec![0.0, 1.0],
    ///     vec![1.0, 0.0],
    ///     vec![1.0, 1.0],
    /// ]);
    /// let batch_predictions = network.predict(batch_input);
    ///
    /// // Apply threshold for binary classification
    /// for (i, row) in batch_predictions.data.iter().enumerate() {
    ///     let binary_result = if row[0] > 0.5 { 1 } else { 0 };
    ///     println!("Sample {}: {}", i, binary_result);
    /// }
    /// ```
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
        let mut error = targets.transpose() - forwarded.last().unwrap().clone();

        for (idx, layer) in self.layers.iter().enumerate().rev() {
            let input_to_layer = if idx == 0 {
                input.clone()
            } else {
                forwarded[idx - 1].clone()
            };

            let delta = Matrix::naive_multiply(forwarded[idx].clone().derivative(), error.clone());

            if idx > 0 {
                error = delta.clone() * layer.borrow().matrix.clone();
            }

            self.adjust(input_to_layer, layer.clone(), delta);
        }
    }

    fn apply_activation(&self, input: Matrix, layer: Rc<RefCell<Layer>>) {
        let mut layer_borrow = layer.borrow_mut();

        layer_borrow.forwarded = Some(Matrix::new(
            (input * layer_borrow.matrix.transpose())
                .data
                .iter()
                .map(|row| row.iter().map(Calc::sigmoid).collect())
                .collect(),
        ))
    }

    fn adjust(&self, input: Matrix, layer: Rc<RefCell<Layer>>, delta: Matrix) {
        let adjustment = input.transpose() * delta.clone();
        let mut layer_borrow = layer.borrow_mut();
        layer_borrow.matrix = layer_borrow.matrix.clone() + adjustment.transpose()
    }
}
