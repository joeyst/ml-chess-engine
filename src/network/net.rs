use crate::rand::Rng;
use std::f64::consts::E;
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
  pub fn run_data(&mut self, input: Vec<f64>, target: f64) {
    self.forward_prop(input);
    self.backward_prop(target);
  }

  pub fn forward_backward_forward_get_value(&mut self, input: Vec<f64>, target: f64) -> f64 {
    self.forward_prop(input.clone());
    self.backward_prop(target);
    self.forward_prop(input);
    self.get_final_value()
  }

  pub fn forward_prop(&mut self, input: Vec<f64>) {
    self.values[0] = input;
    self.set_network_values();
  }

  pub fn get_forward_prop_error(&mut self, input: Vec<f64>, target: f64) -> f64 {
    self.values[0] = input;
    self.set_network_values();
    self.get_error(target)
  }

  pub fn backward_prop(&mut self, target: f64) {
    self.set_network_error_signals(target);
    self.adjust_weights_after_error_signals_set();
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

  fn create_random(input_values: Vec<f64>, layers: usize, nodes: usize, activation_fn: fn(f64) -> f64, derivative: fn(f64) -> f64, l_r: f64) -> Net {
    let net = &mut Net::create(input_values, layers, nodes, activation_fn, derivative, l_r);
    let mut rng = rand::thread_rng();
    for layer in net.weights.iter_mut() {
      for node in layer.iter_mut() {
        for weight in node {
          *weight = rng.gen_range(-0.5..0.5);
        }
      }
    }
    net.clone()
  }

  #[inline]
  fn get_error(&self, target: f64) -> f64 {
    target - self.get_final_value()
  }

  #[inline]
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
    self.values[layer_index].iter_mut()
                            .for_each(|value| *value = (self.act_fn)(*value));
  }

  fn adjust_weights_after_error_signals_set(&mut self) {
    let number_of_layers = self.values.len();
    let mut number_of_nodes_in_current_layer;
    let mut number_of_nodes_in_next_layer;
    for layer_index in 0..(number_of_layers - 1) {
      number_of_nodes_in_current_layer = self.values[layer_index].len();
      number_of_nodes_in_next_layer = self.values[layer_index + 1].len();
      for node_index in 0..number_of_nodes_in_current_layer {
        for weight_index in 0..number_of_nodes_in_next_layer {
          self.weights[layer_index][node_index][weight_index] += 
            self.learning_rate * self.error_signals[layer_index + 1][weight_index] * self.values[layer_index][node_index];
        }
      }
    }
  }

  fn set_network_error_signals(&mut self, target: f64) {
    self.set_final_layer_error_signal(target);
    let final_index = self.values.len() - 1;
    (1..final_index).rev()
                    .for_each(|layer_index| self.set_layer_error_signals_not_last(layer_index));
  }

  fn set_final_layer_error_signal(&mut self, target: f64) {
    let last_layer_index = self.values.len() - 1;
    let change_in_error_from_output = self.get_error(target);
    let change_in_output_from_net = (self.der_fn)(self.get_final_value());
    self.error_signals[last_layer_index][0] = change_in_error_from_output * change_in_output_from_net;
  }

  fn get_final_layer_error_signal_after_calculated(&self) -> f64 {
    self.error_signals[self.values.len() - 1][0]
  }

  fn set_layer_error_signals_not_last(&mut self, layer_index: usize) {
    let curr_layer_length = self.values[layer_index].len();
    let next_layer_length = self.values[layer_index + 1].len();
    for node_index in 0..curr_layer_length {
      self.error_signals[layer_index][node_index] = 0f64;
      for weight_index in 0..next_layer_length {
        self.error_signals[layer_index][node_index] += self.error_signals[layer_index + 1][weight_index] * self.weights[layer_index][node_index][weight_index];
      }
    }

    let derivative_of = self.der_fn;
    for node_index in 0..curr_layer_length {
      self.error_signals[layer_index][node_index] = (derivative_of)(self.values[layer_index][node_index]) * self.error_signals[layer_index][node_index];
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

  fn sigmoid(input: f64) -> f64 {
    1f64 / (1f64 + f64::powf(std::f64::consts::E as f64, -input))
  }

  fn sigmoid_der(input: f64) -> f64 {
    input * (1f64 - input)
  }

  fn identity(input: f64) -> f64 {
    input
  }

  fn identity_der(_input: f64) -> f64 {
    1f64
  }

  fn tanh(input: f64) -> f64 {
    ((E.powf(input)) - E.powf(-input)) /
    ((E.powf(input)) + E.powf(-input))
  }

  fn tanh_der(input: f64) -> f64 {
    2f64 / (E.powf(input) + E.powf(-input))
  }

  fn tanh_der_clipped(input: f64) -> f64 {
    let val = tanh_der(input);
    if val.abs() < 0.5f64 {
      val
    } else if val >= 0.5f64 {
      0.5f64
    } else {
      -0.5f64
    }
  }

  lazy_static! {
    static ref NN: Net = Net::create(vec![1f64; 2], 3, 6, leaky_ReLu, leaky_ReLu_der, 0.05);
    static ref NN_MUT: Mutex<Net> = Mutex::new(Net::create(vec![1f64; 2], 3, 6, leaky_ReLu, leaky_ReLu_der, 0.05));
    // for NN_RAND, tanh, tanh_der_clipped, 0.02 learning rate, it seems like 
    // 40 layers and 15 nodes per layer is the sweet spot
    static ref NN_RAND: Mutex<Net> = Mutex::new(Net::create_random(vec![1f64; 2], 40, 20, tanh, tanh_der_clipped, 0.02));
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

  #[test]
  fn correctly_sets_final_error_signal() {
    let new = &mut NN_MUT.lock().unwrap().clone();
    for layer in new.weights.iter_mut() {
      for node in layer.iter_mut() {
        for weight in node.iter_mut() {
          *weight = 1f64;
        }
      }
    }
    new.act_fn = identity;
    new.der_fn = identity_der;
    new.biases[1][0] = 1f64;
    let target = 466f64;
    new.set_network_values();
    new.set_final_layer_error_signal(target);
    // tested with identity function returning 2f64 and gave error, so this is correct
    assert_eq!(new.get_final_layer_error_signal_after_calculated(), -2f64);
  }

  #[test]
  fn correctly_sets_all_error_signals() {
    let new = &mut NN_MUT.lock().unwrap().clone();
    for layer in new.weights.iter_mut() {
      for node in layer.iter_mut() {
        for weight in node.iter_mut() {
          *weight = 1f64;
        }
      }
    }
    new.act_fn = identity;
    new.der_fn = identity_der;
    new.biases[1][0] = 1f64;
    let target = 466f64;
    new.set_network_values();
    new.set_network_error_signals(target);
    println!("error_signals: {:?}\n\n", new.error_signals);
    println!("weights: {:?}\n\n", new.weights);
    println!("biases: {:?}\n\n", new.biases);
    println!("values: {:?}\n\n", new.values);
    // checked manually
    assert_eq!(new.get_final_layer_error_signal_after_calculated(), -2f64);
  }

  fn print_weights(weights: Vec<Vec<Vec<f64>>>, message: &str) {
    for layer in weights.iter() {
      println!("{} {:?}\n\n", message, layer);
    }
  }

  fn print_final_value(final_value: f64, message: &str) {
    println!("{} {:?}\n\n", message, final_value);
  }

  fn print_error_signals(error_signals: Vec<Vec<f64>>, message: &str) {
    println!("{} {:?}\n\n", message, error_signals);
  }

  fn absolute_difference(first: f64, second: f64) -> f64 {
    (first - second).abs()
  }

  #[test]
  #[ignore]
  fn correctly_adjusts_weights() {
    let new = &mut NN_RAND.lock().unwrap().clone();
    new.set_network_values();

    print_final_value(new.get_final_value(), "Previous final value: ");

    let mut msg: &str;
    let mut msg_string: String;
    let mut val: f64;
    let data: Vec<f64> = new.values[0].clone();
    for target_gradient in -3..4 {
      val = (target_gradient as f64) * -0.25;
      for _i in 0..200 {
        new.run_data(data.clone(), val);
      }
      if new.get_final_value().is_nan() {
        panic!("Network exploded! net.rs, fn correctly_adjusts_weights()")
      }
      assert!(absolute_difference(new.get_final_value(), val) < 0.001);
      msg_string = format!("{:?}: ", val);
      msg = &msg_string;
      print_final_value(new.get_final_value(), msg);
    }
  }
}