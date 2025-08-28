use aspirina::layer::Layer;
use aspirina::matrix::Matrix;

#[test]
fn create_layer_with_matrix() {
    let matrix = Matrix::new(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    
    let layer = Layer::new(matrix);
    
    assert_eq!(layer.matrix.data, vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);
    assert!(layer.forwarded.is_none());
}