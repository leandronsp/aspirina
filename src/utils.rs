#[derive(PartialEq)]
pub struct Utils {}

impl Utils {
    pub fn matrix_transpose(matrix: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut result = vec![vec![matrix[0][0]; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                result[j][i] = matrix[i][j];
            }
        }

        result
    }

    pub fn matrix_element_wise_operation(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>, operation: fn(f64, f64) -> f64) -> Vec<Vec<f64>> {
        let m1_rows_len = m1.len();
        let m2_rows_len = m2.len();
        let m1_cols_len = m1[0].len();
        let m2_cols_len = m2[0].len();

        if m1_rows_len != m2_rows_len || m1_cols_len != m2_cols_len {
            panic!("Incompatible dimensions")
        }

        let mut result = vec![vec![0.0; m2_cols_len]; m1_rows_len];

        for i in 0..m1_rows_len {
            for j in 0..m2_cols_len {
                result[i][j] += operation(m1[i][j], m2[i][j]);
            }
        }

        result
    }

    pub fn matrix_add(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        Self::matrix_element_wise_operation(m1, m2, |a, b| { a + b })
    }

    pub fn matrix_subtract(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        Self::matrix_element_wise_operation(m1, m2, |a, b| { a - b })
    }

    pub fn matrix_naive_multiply(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        Self::matrix_element_wise_operation(m1, m2, |a, b| { a * b })
    }

    pub fn matrix_multiply(m1: Vec<Vec<f64>>, m2: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let m1_rows_len = m1.len();
        let m2_rows_len = m2.len();
        let m1_cols_len = m1[0].len();
        let m2_cols_len = m2[0].len();

        if m1_cols_len != m2_rows_len {
            panic!("Incompatible dimensions: {:?} and {:?}", m1, m2);
        }

        let mut result = vec![vec![0.0; m2_cols_len]; m1_rows_len];

        for i in 0..m1_rows_len {
            for j in 0..m2_cols_len {
                for k in 0..m1_cols_len {
                    result[i][j] += m1[i][k] * m2[k][j];
                }
            }
        }

        result
    }

    pub fn sigmoid(x: &f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    pub fn sigmoid_derivative(x: &f64) -> f64 {
        x * (1.0 - x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_multiply() {
        assert_eq!(
            Utils::matrix_multiply(
                vec![vec![1.0, 1.0, 0.0]],
                vec![
                    vec![1.0, 1.0, 1.0, 1.0], 
                    vec![1.0, 1.0, 1.0, 1.0], 
                    vec![1.0, 1.0, 1.0, 1.0], 
                ]
            ), 
            vec![vec![2.0, 2.0, 2.0, 2.0]]
        );
    }

    #[test]
    fn test_matrix_naive_multiply() {
        assert_eq!(
            Utils::matrix_element_wise_operation(
                vec![vec![1.0, 1.0, 0.0, 2.0]],
                vec![vec![2.0, 3.0, 4.0, 5.0]],
                |a, b| { a * b }
            ),
            vec![vec![2.0, 3.0, 0.0, 10.0]]
        );
    }

    #[test]
    fn test_matrix_addition() {
        assert_eq!(
            Utils::matrix_element_wise_operation(
                vec![vec![1.0, 1.0, 0.0, 2.0]],
                vec![vec![2.0, 3.0, 4.0, -5.0]],
                |a, b| { a + b }
            ),
            vec![vec![3.0, 4.0, 4.0, -3.0]]
        );
    }

    #[test]
    fn test_matrix_subtraction() {
        assert_eq!(
            Utils::matrix_element_wise_operation(
                vec![vec![1.0, 1.0, 0.0, 2.0]],
                vec![vec![2.0, 3.0, 4.0, -5.0]],
                |a, b| { a - b }
            ),
            vec![vec![-1.0, -2.0,- 4.0, 7.0]]
        );
    }

    // a => 3 x 2
    // b => 2 x 3
    #[test]
    fn test_matrix_transpose() {
        assert_eq!(
            Utils::matrix_transpose(vec![
                vec![1.0, 2.0],
                vec![4.0, 5.0],
                vec![7.0, 8.0],
            ]),
            vec![
                vec![1.0, 4.0, 7.0],
                vec![2.0, 5.0, 8.0],
            ]
        );
    }

    #[test]
    fn test_sigmoid() {
        assert_eq!(Utils::sigmoid(&0.42), 0.6034832498647263);
    }

    #[test]
    fn test_sigmoid_derivative() {
        assert_eq!(Utils::sigmoid_derivative(&0.85), 0.1275);
    }
}
