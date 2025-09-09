//! # Aspirina
//!
//! A modular neural network library with matrix operations and backpropagation.
//!
//! Aspirina provides the core building blocks for creating and training neural networks,
//! including matrix operations with operator overloading, activation functions, and
//! a complete neural network implementation with backpropagation.
//!
//! ## Features
//!
//! - **Matrix operations** with operator overloading (`+`, `-`, `*`)
//! - **Activation functions** (sigmoid, tanh) with derivatives
//! - **Neural network layers** with shared ownership patterns
//! - **Complete neural network** with forward and backward propagation
//! - **Training and prediction** methods
//!
//! ## Quick Start
//!
//! ```rust
//! use aspirina::{matrix::Matrix, neural_network::NeuralNetwork, layer::Layer};
//!
//! // Create simple training data
//! let input = Matrix::new(vec![vec![1.0, 0.0]]);
//! let expected = Matrix::new(vec![vec![1.0]]);
//!
//! // Create a simple network
//! let layers = vec![
//!     Layer::new(Matrix::new(vec![vec![0.1, 0.2]])), // 1 neuron, 2 inputs
//! ];
//! let network = NeuralNetwork::new(layers);
//!
//! // Make predictions
//! let result = network.predict(input);
//! println!("Result: {:?}", result.data);
//! ```
//!
//! ## Architecture
//!
//! The library is organized into four main modules:
//!
//! - [`matrix`] - Matrix operations with operator overloading
//! - [`calc`] - Activation functions and their derivatives  
//! - [`layer`] - Neural network layer structure
//! - [`neural_network`] - Complete neural network implementation
//!
//! Each module is designed to work together while maintaining clear separation of concerns.

pub mod calc;
pub mod layer;
pub mod matrix;
pub mod neural_network;
