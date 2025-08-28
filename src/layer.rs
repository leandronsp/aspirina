use crate::matrix::Matrix;

#[derive(Debug, Clone)]
pub struct Layer {
    pub matrix: Matrix,
    pub forwarded: Option<Matrix>,
}

impl Layer {
    pub fn new(matrix: Matrix) -> Self {
        Self {
            matrix,
            forwarded: None,
        }
    }
}
