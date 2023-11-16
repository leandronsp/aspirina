use crate::utils::Utils;
use crate::layer::Layer;
    
pub struct ForwardPropagation {
    layers: Vec<Layer>,
}

impl ForwardPropagation {
    fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    fn forward(&self, input: Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let mut result = Vec::new();
        let mut current_input = input.clone();

        for layer in &self.layers {
            current_input = 
                Utils::matrix_multiply(current_input, layer.to_matrix())
                .iter()
                .map(|row| { row.iter().map(Utils::sigmoid).collect() })
                .collect();

            result.push(current_input.clone());
        }

        result.iter().flat_map(|row| { row.clone() }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_propagation() {
        let layers = vec![
            Layer::new(4, 3),
            Layer::new(4, 4),
            Layer::new(1, 4),
        ];

        let forward_propagation = ForwardPropagation::new(layers);

        assert_eq!(
            forward_propagation.forward(vec![vec![1.0, 1.0, 0.0]]),
            vec![
                vec![0.8807970779778823, 0.8807970779778823, 0.8807970779778823, 0.8807970779778823],
                vec![0.9713403945491743, 0.9713403945491743, 0.9713403945491743, 0.9713403945491743],
                vec![0.9798730158567087]
            ]
        );
    }
}
