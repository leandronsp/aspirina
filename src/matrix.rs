use crate::calc::Calc;
use std::ops::Add;

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

        let mut result = vec![vec![0.0; rows]; cols];

        for (i, row) in self.data.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                result[j][i] = value;
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

        for (i, (row1, row2)) in m1.data.iter().zip(m2.data.iter()).enumerate() {
            for (j, (&val1, &val2)) in row1.iter().zip(row2.iter()).enumerate() {
                result[i][j] = op(val1, val2);
            }
        }

        Self { data: result }
    }

    pub fn matrix_add(m1: Self, m2: Self) -> Self {
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

        for (i, m1_row) in m1.data.iter().enumerate() {
            for j in 0..m2_cols_len {
                for (k, &m1_val) in m1_row.iter().enumerate() {
                    result[i][j] += m1_val * m2.data[k][j];
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

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::element_wise_operation(self, other, |a, b| a + b)
    }
}
