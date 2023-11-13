#[derive(PartialEq)]
struct Neuron {
    weights: Vec<f64>,
}

#[derive(PartialEq)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn new(number_of_neurons: usize, number_of_weights: usize) -> Self {
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

    fn to_transposed_matrix(&self) -> Vec<Vec<f64>> {
        let mut matrix = Vec::new();
        let weights_size = self.neurons[0].weights.len();

        // Transpose
        for idx in 0..weights_size {
            let mut row = Vec::new();

            for neuron in &self.neurons {
                row.push(neuron.weights[idx]);
            }

            matrix.push(row);
        }

        matrix
    }
}

fn main() {
    let layer = Layer::new(4, 3);

    assert_eq!(layer.to_transposed_matrix(), vec![
       vec![1.0, 1.0, 1.0, 1.0],
       vec![1.0, 1.0, 1.0, 1.0],
       vec![1.0, 1.0, 1.0, 1.0]
    ]);

    let layer = Layer::new(1, 4);

    assert_eq!(layer.to_transposed_matrix(), vec![
       vec![1.0], vec![1.0], vec![1.0], vec![1.0]
    ]);
}
