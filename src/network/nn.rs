use super::forward_prop;

pub struct NN {
  pub values: Vec<Vec<f32>>,
  pub biases: Vec<Vec<f32>>,
  pub weights: Vec<Vec<Vec<f32>>>,
  pub act_fn: fn(f32) -> f32,
  pub der_fn: fn(f32) -> f32,
  pub influence: Vec<Vec<f32>>,
  pub learning_rate: f32
}

impl NN {
  pub fn backprop(&mut self, target: f32) {
    self.set_all_layer_influences(target);
    self.adjust_all_weights();
  }

  fn adjust_all_weights(&mut self) {
    (0u16..).for_each(|index| self.adjust_weights_in_layer(index));
  }

  fn adjust_weights_in_layer(&mut self, layer_index: u16) {
    let influences: Vec<f32> = self.influence[(layer_index + 1) as usize].clone();
    let vals: Vec<f32> = self.values[layer_index as usize].clone();
    let lr: f32 = self.learning_rate.clone();

    for (val, val_index) in vals.iter().zip(0..) {
      for (inf, influence_index) in influences.iter().zip(0..) {
        self.weights[layer_index as usize][val_index as usize][influence_index as usize] += lr * val * inf;
      }
    }
  }

  fn set_all_layer_influences(&mut self, target: f32) {
    self.set_starting_influence(target);
    (0u16..(self.values.len()-1) as u16).rev().for_each(|index| self.set_layer_influence(index));
  }

  fn set_layer_influence(&mut self, layer_index: u16) {
    let influences = self.get_layer_influence(layer_index);
    self.influence[layer_index as usize].iter_mut().zip(influences).for_each(|(old, new)| *old = new);
  }

  fn get_layer_influence(&self, layer_index: u16) -> Vec<f32> {
    let mut influences: Vec<f32> = vec![0f32; self.values[layer_index as usize].len()];
    let mut val_from_der_fn: f32;
    let old_influences = self.get_already_calculated_layer_influence(layer_index + 1);

    for (node, index) in self.vals_immut(layer_index).zip(0..) {
      val_from_der_fn = (self.der_fn)(*node);
      for (weight, infl) in self.weights_in_immut(layer_index, index).zip(old_influences.clone()) {
        influences[index as usize] += *weight * *infl * val_from_der_fn;
      }
    }
    influences
  }

  fn get_already_calculated_layer_influence(&self, layer_index: u16) -> std::slice::Iter<f32> {
    self.influence[(layer_index + 1) as usize].iter()
  }

  fn set_starting_influence(&mut self, target: f32) {
    let num_layers = self.values.len();
    let infl = self.starting_influence(target);
    self.influence[num_layers - 1][0] = infl;
  }

  fn starting_influence(&self, target: f32) -> f32 {
    (self.der_fn)(target - self.get_final_value())
  }
}