pub struct Calc {}

impl Calc {
    pub fn sigmoid(x: &f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn sigmoid_derivative(x: &f64) -> f64 {
        x * (1.0 - x)
    }
}
