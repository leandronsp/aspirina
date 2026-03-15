//! # Activation Functions Module
//!
//! This module provides activation functions and their derivatives essential for neural network
//! computation. Activation functions introduce non-linearity into neural networks, enabling them
//! to learn complex patterns and solve non-linearly separable problems.
//!
//! ## Key Concepts
//!
//! **Activation Functions**: Mathematical functions that determine the output of a neural network
//! node given a set of inputs. They help neural networks learn complex patterns by introducing
//! non-linear transformations.
//!
//! **Derivatives**: Required for backpropagation during training. The derivative of an activation
//! function indicates how much the output changes with respect to small changes in input, which
//! is crucial for gradient-based optimization.
//!
//! ## Available Functions
//!
//! - **Sigmoid**: Maps any real number to a value between 0 and 1, commonly used for binary
//!   classification and output layers.
//! - **Tanh**: Maps any real number to a value between -1 and 1, often preferred over sigmoid
//!   for hidden layers due to zero-centered output.

/// Utility struct providing static methods for activation functions and their derivatives.
///
/// All functions are implemented as static methods to avoid unnecessary instantiation overhead.
/// The functions take references to `f64` values for memory efficiency when working with large
/// matrices in neural network computations.
pub struct Calc;

impl Calc {
    /// Computes the sigmoid activation function.
    ///
    /// The sigmoid function maps any real-valued input to a value between 0 and 1, making it
    /// particularly useful for binary classification tasks and as an activation function in
    /// neural network output layers.
    ///
    /// # Mathematical Formula
    ///
    /// σ(x) = 1 / (1 + e^(-x))
    ///
    /// # Parameters
    ///
    /// * `x` - A reference to the input value
    ///
    /// # Returns
    ///
    /// A value between 0.0 and 1.0 representing the sigmoid of the input
    ///
    /// # Examples
    ///
    /// ```
    /// use aspirina_core::calc::Calc;
    ///
    /// let input = 0.0;
    /// let result = Calc::sigmoid(&input);
    /// assert_eq!(result, 0.5); // sigmoid(0) = 0.5
    ///
    /// let large_positive = 10.0;
    /// let result = Calc::sigmoid(&large_positive);
    /// assert!(result > 0.99); // approaches 1 for large positive values
    /// ```
    pub fn sigmoid(x: &f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    /// Computes the derivative of the sigmoid function.
    ///
    /// This function calculates the derivative of sigmoid with respect to its input, which is
    /// essential for backpropagation during neural network training. The derivative indicates
    /// how sensitive the sigmoid output is to changes in its input.
    ///
    /// # Mathematical Formula
    ///
    /// σ'(x) = σ(x) * (1 - σ(x))
    ///
    /// # Important Note
    ///
    /// This function expects the **output** of the sigmoid function, not the original input.
    /// This optimization avoids recomputing the sigmoid during backpropagation.
    ///
    /// # Parameters
    ///
    /// * `x` - A reference to the sigmoid output value (should be between 0 and 1)
    ///
    /// # Returns
    ///
    /// The derivative of sigmoid at the given point, maximum value is 0.25 at x = 0.5
    ///
    /// # Examples
    ///
    /// ```
    /// use aspirina_core::calc::Calc;
    ///
    /// let sigmoid_output = 0.5; // sigmoid(0) = 0.5
    /// let derivative = Calc::sigmoid_derivative(&sigmoid_output);
    /// assert_eq!(derivative, 0.25); // maximum derivative at x = 0.5
    ///
    /// // Typical usage in backpropagation
    /// let input = 2.0;
    /// let output = Calc::sigmoid(&input);
    /// let grad = Calc::sigmoid_derivative(&output);
    /// ```
    pub fn sigmoid_derivative(x: &f64) -> f64 {
        x * (1.0 - x)
    }

    /// Computes the hyperbolic tangent (tanh) activation function.
    ///
    /// The tanh function maps any real-valued input to a value between -1 and 1. It's often
    /// preferred over sigmoid for hidden layers because its output is zero-centered, which can
    /// help with gradient flow during training and generally leads to faster convergence.
    ///
    /// # Mathematical Formula
    ///
    /// tanh(x) = (e^x - e^(-x)) / (e^x + e^(-x))
    ///
    /// # Parameters
    ///
    /// * `x` - A reference to the input value
    ///
    /// # Returns
    ///
    /// A value between -1.0 and 1.0 representing the tanh of the input
    ///
    /// # Examples
    ///
    /// ```
    /// use aspirina_core::calc::Calc;
    ///
    /// let input = 0.0;
    /// let result = Calc::tanh(&input);
    /// assert_eq!(result, 0.0); // tanh(0) = 0
    ///
    /// let positive = 1.0;
    /// let result = Calc::tanh(&positive);
    /// assert!(result > 0.0 && result < 1.0);
    /// ```
    pub fn tanh(x: &f64) -> f64 {
        x.tanh()
    }

    /// Computes the derivative of the hyperbolic tangent function.
    ///
    /// This function calculates the derivative of tanh with respect to its input, used in
    /// backpropagation for neural networks that use tanh activation. The derivative has a
    /// maximum value of 1.0 at x = 0.
    ///
    /// # Mathematical Formula
    ///
    /// tanh'(x) = 1 - tanh²(x)
    ///
    /// # Parameters
    ///
    /// * `x` - A reference to the input value (not the tanh output, unlike sigmoid_derivative)
    ///
    /// # Returns
    ///
    /// The derivative of tanh at the given input, maximum value is 1.0 at x = 0
    ///
    /// # Examples
    ///
    /// ```
    /// use aspirina_core::calc::Calc;
    ///
    /// let input = 0.0;
    /// let derivative = Calc::tanh_derivative(&input);
    /// assert_eq!(derivative, 1.0); // maximum derivative at x = 0
    ///
    /// let large_input = 5.0;
    /// let derivative = Calc::tanh_derivative(&large_input);
    /// assert!(derivative < 0.01); // approaches 0 for large inputs
    /// ```
    pub fn tanh_derivative(x: &f64) -> f64 {
        1.0 - x.tanh().powi(2)
    }
}
