type LayerNodeMatrix = Vec<Vec<f64>>;

#[derive(Clone)]
pub struct Net {
  pub values: LayerNodeMatrix,
  pub biases: LayerNodeMatrix,
  pub weights: Vec<Vec<Vec<f64>>>,
  pub act_fn: fn(f64) -> f64,
  pub der_fn: fn(f64) -> f64,
  pub error_signals: LayerNodeMatrix,
  pub learning_rate: f64
}

impl Net {
  pub fn get_forward_prop_error(&mut self, input: Vec<f64>, target: f64) -> f64 {
    self.values[0] = input;
    self.set_network_values();
    target - self.get_final_value()
  }

  pub fn create(input_values: Vec<f64>, layers: usize, nodes: usize, activation_fn: fn(f64) -> f64, derivative: fn(f64) -> f64, l_r: f64) -> Net {
    let number_of_nodes_in_input_layer = input_values.len();
    let mut values: LayerNodeMatrix = LayerNodeMatrix::new();
    values.append(&mut vec![input_values]);
    values.append(&mut vec![vec![0f64; nodes]; layers]);
    values.append(&mut vec![vec![0f64; 1]]);

    let mut biases: LayerNodeMatrix = LayerNodeMatrix::new();
    biases.append(&mut vec![vec![0f64; number_of_nodes_in_input_layer]]);
    biases.append(&mut vec![vec![0f64; nodes]; layers]);
    biases.append(&mut vec![vec![0f64; 1]]);

    let mut weights: Vec<Vec<Vec<f64>>> = Vec::new();
    weights.append(&mut vec![vec![vec![0f64; nodes]; number_of_nodes_in_input_layer]]);
    weights.append(&mut vec![vec![vec![0f64; nodes]; nodes]; layers - 1]);
    weights.append(&mut vec![vec![vec![0f64; 1]; nodes]]);

    let mut error_signals: LayerNodeMatrix = LayerNodeMatrix::new();
    error_signals.append(&mut vec![vec![0f64; number_of_nodes_in_input_layer]]);
    error_signals.append(&mut vec![vec![0f64; nodes]; layers]);
    error_signals.append(&mut vec![vec![0f64; 1]]);

    Net {
      values: values,
      biases: biases,
      weights: weights,
      act_fn: activation_fn,
      der_fn: derivative,
      error_signals: error_signals,
      learning_rate: l_r
    }
  }

  fn get_final_value(&self) -> f64 {
    self.values[self.values.len() - 1][0]
  }

  fn set_network_values(&mut self) {
    let network_length = self.values.len();
    for layer_index in 1..network_length {
      self.set_layer_values(layer_index);
    }
  }

  fn set_layer_values(&mut self, layer_index: usize) {
    let length_of_layer = self.values[layer_index].len();
    let length_of_prev_layer = self.values[layer_index - 1].len();
    for node_index in 0..length_of_layer {
      self.values[layer_index][node_index] = self.biases[layer_index][node_index];
      for previous_index in 0..length_of_prev_layer {
        self.values[layer_index][node_index] += self.values[layer_index - 1][previous_index] * self.weights[layer_index - 1][previous_index][node_index];
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::sync::Mutex;

  fn leaky_ReLu(input: f64) -> f64 {
    if input > 0f64 { input } else { input * 0.2f64 }
  }

  fn leaky_ReLu_der(input: f64) -> f64 {
    if input > 0f64 { 1f64 } else { 0.2f64 }
  }

  lazy_static! {
    static ref NN: Net = Net::create(vec![1f64; 2], 3, 6, leaky_ReLu, leaky_ReLu_der, 0.05);
    static ref NN_MUT: Mutex<Net> = Mutex::new(Net::create(vec![1f64; 2], 3, 6, leaky_ReLu, leaky_ReLu_der, 0.05));
  }

  #[test]
  fn creates_network_with_correct_dimensions_for_values() {
    assert_eq!(NN.values[0].len(), 2);
    assert_eq!(NN.values[1].len(), 6);
    assert_eq!(NN.values[4].len(), 1);
    assert_eq!(NN.values.len(), 5);
  }

  #[test]
  fn initializes_network_with_values() {
    assert_eq!(NN.values[0], vec![1f64; 2]);
  }

  #[test]
  fn creates_network_with_correct_dimensions_for_weights() {
    assert_eq!(NN.weights[0].len(), 2);
    assert_eq!(NN.weights[0][0].len(), NN.values[1].len());
    assert_eq!(NN.weights[1][0].len(), NN.values[2].len());
    assert_eq!(NN.weights[NN.weights.len() - 1][0].len(), NN.values[NN.values.len() - 1].len());
    assert_eq!(NN.weights.len(), NN.values.len() - 1);
    assert_eq!(NN.weights[NN.weights.len() - 1].len(), NN.values[NN.values.len() - 2].len());
  }

  #[test]
  fn sets_all_values() {
    let new = &mut NN_MUT.lock().unwrap().clone();
    for layer in new.weights.iter_mut() {
      for node in layer.iter_mut() {
        for weight in node.iter_mut() {
          *weight = 1f64;
        }
      }
    }
    new.set_network_values();
    println!("{:?}", new.values);
    println!("{:?}", new.weights);
    let length_of_network = new.values.len();
    assert_eq!(new.values[length_of_network - 1][0], 432f64);
  }

  #[test]
  fn bias_affects_values() {
    let new = &mut NN_MUT.lock().unwrap().clone();
    for layer in new.weights.iter_mut() {
      for node in layer.iter_mut() {
        for weight in node.iter_mut() {
          *weight = 1f64;
        }
      }
    }
    new.biases[1][0] = 1f64;
    new.set_network_values();
    println!("{:?}", new.values);
    println!("{:?}", new.weights);
    let length_of_network = new.values.len();
    assert_eq!(new.values[length_of_network - 1][0], 468f64);
  }

  #[test]
  fn gets_final_value() {
    let new = &mut NN_MUT.lock().unwrap().clone();
    for layer in new.weights.iter_mut() {
      for node in layer.iter_mut() {
        for weight in node.iter_mut() {
          *weight = 1f64;
        }
      }
    }
    new.biases[1][0] = 1f64;
    new.set_network_values();
    println!("{:?}", new.values);
    println!("{:?}", new.weights);
    assert_eq!(new.get_final_value(), 468f64);
  }

  #[test]
  fn gets_error_from_forward_prop() {
    let new = &mut NN_MUT.lock().unwrap().clone();
    for layer in new.weights.iter_mut() {
      for node in layer.iter_mut() {
        for weight in node.iter_mut() {
          *weight = 1f64;
        }
      }
    }
    new.biases[1][0] = 1f64;
    let target = 466f64;
    let input = new.values[0].clone();
    assert_eq!(new.get_forward_prop_error(input, target), -2f64);
  }
}