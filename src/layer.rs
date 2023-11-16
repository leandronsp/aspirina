use crate::utils::Utils;

#[derive(PartialEq)]
struct Neuron {
    weights: Vec<f64>,
}

#[derive(PartialEq)]
pub struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(number_of_neurons: usize, number_of_weights: usize) -> Self {
        let mut neurons = Vec::with_capacity(number_of_neurons);

        for _ in 0..number_of_neurons {
            let mut weights = Vec::with_capacity(number_of_weights);

            for _ in 0..number_of_weights {
                weights.push(1.0);
            }

            neurons.push(Neuron { weights });
        }

        Self { neurons }
    }

    pub fn to_matrix(&self) -> Vec<Vec<f64>> {
        let matrix = 
            self
            .neurons
            .iter()
            .map(|neuron| { neuron.weights.clone() })
            .collect();

        Utils::matrix_transpose(matrix)
    }

    pub fn from_matrix(matrix: Vec<Vec<f64>>) -> Self {
        let neurons = 
            Utils::matrix_transpose(matrix)
            .iter()
            .map(|row| { Neuron { weights: row.to_vec() }})
            .collect();

        Self { neurons }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer() {
        let layer = Layer::new(4, 3);

        assert_eq!(layer.to_matrix(), vec![
           vec![1.0, 1.0, 1.0, 1.0],
           vec![1.0, 1.0, 1.0, 1.0],
           vec![1.0, 1.0, 1.0, 1.0],
        ]);

        let layer = Layer::new(1, 4);

        assert_eq!(layer.to_matrix(), vec![
            vec![1.0],
            vec![1.0], 
            vec![1.0],
            vec![1.0],
        ]);

        let matrix = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];

        let layer = Layer::from_matrix(matrix);

        assert_eq!(layer.neurons[0].weights, vec![1.0, 4.0, 7.0]);
    }
}

