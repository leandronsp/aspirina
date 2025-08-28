use aspirina::calc::Calc;

#[test]
fn sigmoid_at_zero() {
    assert_eq!(Calc::sigmoid(0.0), 0.5);
}

#[test]
fn sigmoid_positive() {
    assert!(Calc::sigmoid(1.0) > 0.5);
    assert!(Calc::sigmoid(1.0) < 1.0);
}

#[test]
fn sigmoid_negative() {
    assert!(Calc::sigmoid(-1.0) < 0.5);
    assert!(Calc::sigmoid(-1.0) > 0.0);
}

#[test]
fn sigmoid_derivative_at_half() {
    assert_eq!(Calc::sigmoid_derivative(0.5), 0.25);
}

#[test]
fn sigmoid_derivative_boundaries() {
    assert_eq!(Calc::sigmoid_derivative(0.0), 0.0);
    assert_eq!(Calc::sigmoid_derivative(1.0), 0.0);
}

#[test]
fn tanh_at_zero() {
    assert_eq!(Calc::tanh(0.0), 0.0);
}

#[test]
fn tanh_positive() {
    assert!(Calc::tanh(1.0) > 0.0);
    assert!(Calc::tanh(1.0) < 1.0);
}

#[test]
fn tanh_negative() {
    assert!(Calc::tanh(-1.0) < 0.0);
    assert!(Calc::tanh(-1.0) > -1.0);
}

#[test]
fn tanh_derivative_at_zero() {
    assert_eq!(Calc::tanh_derivative(0.0), 1.0);
}

#[test]
fn tanh_derivative_positive() {
    assert!(Calc::tanh_derivative(1.0) > 0.0);
    assert!(Calc::tanh_derivative(1.0) < 1.0);
}