//! Neural Network Layer Module
//!
//! This module provides the `Layer` struct which represents individual layers in a neural network.
//! Each layer contains a weight matrix and can store forward propagation results for use during
//! backpropagation training.

use crate::matrix::Matrix;

/// Represents a single layer in a neural network.
///
/// A layer is a fundamental building block of neural networks that contains weights (connections)
/// between neurons and can store forward propagation results for backpropagation training.
///
/// # Structure
///
/// - `matrix`: The weight matrix for this layer. For a layer with `n` inputs and `m` outputs,
///   this is an `m × n` matrix where each row represents the weights for one output neuron.
/// - `forwarded`: Optional storage for forward propagation results. This is populated during
///   the forward pass and used during backpropagation to compute gradients.
///
/// # Usage in Neural Networks
///
/// Layers are typically organized in sequence to form a complete neural network:
/// - Input layer: Receives external data
/// - Hidden layers: Perform intermediate computations
/// - Output layer: Produces final results
///
/// # Examples
///
/// ```rust
/// use aspirina::matrix::Matrix;
/// use aspirina::layer::Layer;
///
/// // Create a layer with 2 inputs and 3 outputs
/// let weights = Matrix::new(vec![
///     vec![0.5, -0.2],  // Weights for first output neuron
///     vec![0.1, 0.8],   // Weights for second output neuron  
///     vec![-0.3, 0.4],  // Weights for third output neuron
/// ]);
/// let layer = Layer::new(weights);
///
/// // The forwarded field starts as None and is populated during forward propagation
/// assert!(layer.forwarded.is_none());
/// ```
#[derive(Debug, Clone)]
pub struct Layer {
    /// The weight matrix for this layer.
    ///
    /// Each row represents the weights connecting all inputs to one output neuron.
    /// For a layer with `n` inputs and `m` outputs, this is an `m × n` matrix.
    pub matrix: Matrix,

    /// Storage for forward propagation results.
    ///
    /// This field is `None` initially and gets populated with the layer's output
    /// during the forward pass. These values are essential for computing gradients
    /// during backpropagation training.
    pub forwarded: Option<Matrix>,
}

impl Layer {
    /// Creates a new neural network layer with the specified weight matrix.
    ///
    /// # Parameters
    ///
    /// * `matrix` - The weight matrix for this layer. Should be an `m × n` matrix where
    ///   `m` is the number of output neurons and `n` is the number of input neurons.
    ///
    /// # Returns
    ///
    /// A new `Layer` instance with the given weights and `forwarded` set to `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use aspirina::matrix::Matrix;
    /// use aspirina::layer::Layer;
    ///
    /// // Create a layer for XOR gate (2 inputs, 2 hidden neurons)
    /// let weights = Matrix::new(vec![
    ///     vec![0.5, -0.5],  // First hidden neuron weights
    ///     vec![-0.5, 0.5],  // Second hidden neuron weights
    /// ]);
    /// let hidden_layer = Layer::new(weights);
    ///
    /// // Layer is ready for use in neural network
    /// assert!(hidden_layer.forwarded.is_none());
    /// ```
    pub fn new(matrix: Matrix) -> Self {
        Self {
            matrix,
            forwarded: None,
        }
    }
}
