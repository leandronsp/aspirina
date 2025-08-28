pub struct Calc;

impl Calc {
    pub fn sigmoid(x: &f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    // Note: expects the OUTPUT of sigmoid, not the input
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
