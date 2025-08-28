use crate::calc::Calc;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        Self { data }
    }

    pub fn transpose(&self) -> Self {
        let rows = self.data.len();
        let cols = self.data[0].len();

        let mut result = vec![vec![self.data[0][0]; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                result[j][i] = self.data[i][j];
            }
        }

        Self { data: result }
    }

    fn element_wise_operation(m1: Self, m2: Self, op: fn(f64, f64) -> f64) -> Self {
        let m1_rows_len = m1.data.len();
        let m2_rows_len = m2.data.len();
        let m1_cols_len = m1.data[0].len();
        let m2_cols_len = m2.data[0].len();

        if m1_rows_len != m2_rows_len || m1_cols_len != m2_cols_len {
            panic!("Incompatible dimensions")
        }

        let mut result = vec![vec![0.0; m2_cols_len]; m1_rows_len];

        for i in 0..m1_rows_len {
            for j in 0..m2_cols_len {
                result[i][j] = op(m1.data[i][j], m2.data[i][j]);
            }
        }

        Self { data: result }
    }

    pub fn add(m1: Self, m2: Self) -> Self {
        Self::element_wise_operation(m1, m2, |a, b| a + b)
    }

    pub fn subtract(m1: Self, m2: Self) -> Self {
        Self::element_wise_operation(m1, m2, |a, b| a - b)
    }

    pub fn naive_multiply(m1: Self, m2: Self) -> Self {
        Self::element_wise_operation(m1, m2, |a, b| a * b)
    }

    pub fn multiply(m1: Self, m2: Self) -> Self {
        let m1_rows_len = m1.data.len();
        let m2_rows_len = m2.data.len();
        let m1_cols_len = m1.data[0].len();
        let m2_cols_len = m2.data[0].len();

        if m1_cols_len != m2_rows_len {
            panic!("Incompatible dimensions for matrix multiplication");
        }

        let mut result = vec![vec![0.0; m2_cols_len]; m1_rows_len];

        for i in 0..m1_rows_len {
            for j in 0..m2_cols_len {
                for k in 0..m1_cols_len {
                    result[i][j] += m1.data[i][k] * m2.data[k][j];
                }
            }
        }

        Self { data: result }
    }

    pub fn derivative(&self) -> Self {
        let result = self
            .data
            .iter()
            .map(|row| row.iter().map(Calc::sigmoid_derivative).collect())
            .collect();

        Self { data: result }
    }
}