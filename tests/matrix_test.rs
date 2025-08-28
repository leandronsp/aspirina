use aspirina::matrix::Matrix;

#[test]
fn transpose_2x3_matrix() {
    let matrix = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);

    let transposed = matrix.transpose();

    assert_eq!(
        transposed.data,
        vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0],]
    );
}

#[test]
fn transpose_square_matrix() {
    let matrix = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    let transposed = matrix.transpose();

    assert_eq!(transposed.data, vec![vec![1.0, 3.0], vec![2.0, 4.0],]);
}

#[test]
fn multiply_2x2_matrices() {
    let m1 = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    let m2 = Matrix::new(vec![vec![2.0, 0.0], vec![1.0, 2.0]]);

    let result = Matrix::multiply(m1, m2);

    assert_eq!(result.data, vec![vec![4.0, 4.0], vec![10.0, 8.0],]);
}

#[test]
fn multiply_2x3_with_3x2() {
    let m1 = Matrix::new(vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]);

    let m2 = Matrix::new(vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]]);

    let result = Matrix::multiply(m1, m2);

    assert_eq!(result.data, vec![vec![58.0, 64.0], vec![139.0, 154.0],]);
}

#[test]
fn naive_multiply_same_size_matrices() {
    let m1 = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    let m2 = Matrix::new(vec![vec![2.0, 3.0], vec![4.0, 5.0]]);

    let result = Matrix::naive_multiply(m1, m2);

    assert_eq!(result.data, vec![vec![2.0, 6.0], vec![12.0, 20.0],]);
}

#[test]
fn naive_multiply_with_zeros() {
    let m1 = Matrix::new(vec![vec![1.0, 0.0, 3.0], vec![4.0, 5.0, 0.0]]);

    let m2 = Matrix::new(vec![vec![2.0, 3.0, 4.0], vec![5.0, 0.0, 7.0]]);

    let result = Matrix::naive_multiply(m1, m2);

    assert_eq!(
        result.data,
        vec![vec![2.0, 0.0, 12.0], vec![20.0, 0.0, 0.0],]
    );
}

#[test]
fn add_same_size_matrices() {
    let m1 = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    let m2 = Matrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    let result = Matrix::matrix_add(m1, m2);

    assert_eq!(result.data, vec![vec![6.0, 8.0], vec![10.0, 12.0],]);
}

#[test]
fn subtract_same_size_matrices() {
    let m1 = Matrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);

    let m2 = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);

    let result = Matrix::subtract(m1, m2);

    assert_eq!(result.data, vec![vec![4.0, 4.0], vec![4.0, 4.0],]);
}

#[test]
fn derivative_applies_sigmoid_derivative() {
    let matrix = Matrix::new(vec![vec![0.0, 0.5, 1.0], vec![0.25, 0.75, 0.9]]);

    let result = matrix.derivative();

    // Check first row
    assert_eq!(result.data[0][0], 0.0);
    assert_eq!(result.data[0][1], 0.25);
    assert_eq!(result.data[0][2], 0.0);

    // Check second row with approximate comparison for floating point
    assert_eq!(result.data[1][0], 0.1875);
    assert_eq!(result.data[1][1], 0.1875);
    assert!((result.data[1][2] - 0.09).abs() < 1e-10);
}
