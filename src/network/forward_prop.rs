use super::nn::NN;
use crate::rand::Rng;

impl NN {
  pub fn forwardprop(&mut self, input: Vec<f32>) -> f32 {
    self.set_all_layers_from_input(input);
    self.get_final_value()
  }

  pub fn create(nodes: u16, layers: u16, a_fn: fn(f32) -> f32, d_fn: fn(f32) -> f32, l_r: f32) -> NN {
    let mut first_weights = vec![vec![vec![0f32; (768 * nodes) as usize]; nodes as usize]; 1];
    let mut rest_of_weights = vec![vec![vec![0f32; nodes as usize]; nodes as usize]; layers as usize];
    first_weights.append(&mut rest_of_weights);
    first_weights[layers as usize] = vec![vec![0f32; 1]; nodes as usize]; 
    first_weights.append(&mut vec![vec![vec![0f32; 1]]]);
    let mut infl = vec![vec![0f32; nodes as usize]; layers as usize];
    infl.append(&mut vec![vec![0f32; 1]; 1]);
    
    NN {
      values: vec![vec![0f32; nodes as usize]; layers as usize],
      biases: vec![vec![0f32; nodes as usize]; layers as usize],
      weights: first_weights,
      act_fn: a_fn,
      der_fn: d_fn,
      influence: infl,
      learning_rate: l_r
    }
  }

  pub fn create_random_wb(nodes: u16, layers: u16, a_fn: fn(f32) -> f32, d_fn: fn(f32) -> f32, l_r: f32) -> NN {
    let mut nn: NN = NN::create(nodes, layers, a_fn, d_fn, l_r);
    let mut rng = rand::thread_rng();
    for layer in &mut nn.weights {
      for node in layer {
        for weight in node {
          *weight = rng.gen_range(-1.0..1.0);
        }
      }
    }
    for layer in &mut nn.biases {
      for node in layer {
        *node = rng.gen_range(-1.0..1.0);
      }
    }
    nn
  }

  pub fn set_all_layers_from_input(&mut self, input: Vec<f32>) {
    self.set_zeroth_layer_from_input(input);
    self.set_all_layers_but_zeroth();
    self.set_last_layer()
  }

  pub fn get_final_value(&self) -> f32 {
    let length = self.values.len() - 1;
    self.values[length].iter()
                        .zip(self.weights[length + 1].clone())
                        .map(|(value, weight)| *value * weight[0])
                        .sum::<f32>()
  }

  pub fn vals_mut(&mut self, layer_index: u16) -> std::slice::IterMut<f32> {
    self.values[layer_index as usize].iter_mut()
  }

  fn weights_in_mut(&mut self, layer_index: u16, node_index: u16) -> std::slice::IterMut<f32> {
    self.weights[layer_index as usize][node_index as usize].iter_mut()
  }

  pub fn vals_immut(&self, layer_index: u16) -> std::slice::Iter<f32> {
    self.values[layer_index as usize].iter()
  }

  pub fn weights_in_immut(&self, layer_index: u16, node_index: u16) -> std::slice::Iter<f32> {
    self.weights[layer_index as usize][node_index as usize].iter()
  }

  fn set_layer_sum(&mut self, layer_index: u16) {
    let length = self.values[layer_index as usize].len();
    let node_values = (1u16..length as u16).map(|index| self.get_node_sum(layer_index, index))
                           .collect::<Vec<f32>>();
    self.vals_mut(layer_index).zip(node_values)
                              .for_each(|(old, new)| *old = new);
  }

  fn get_node_sum(&self, layer_index: u16, node_index: u16) -> f32 {
    assert!(layer_index > 0);
    let previous_values = self.vals_immut(layer_index - 1);
    let previous_weights = self.weights_in_immut(layer_index, node_index);
    previous_values.zip(previous_weights)
                   .map(|(v, w)| *v * *w)
                   .sum::<f32>()
  }

  fn set_layer_val(&mut self, layer_index: u16) {
    self.set_layer_sum(layer_index);
    let activated_values = self.vals_immut(layer_index)
                               .map(|val| (self.act_fn)(*val)).collect::<Vec<f32>>();
    self.vals_mut(layer_index).zip(activated_values)
                              .for_each(|(old, new)| *old = new);
  }

  fn set_last_layer(&mut self) {
    let max_index = self.values.len() - 1;
    let values_of_previous_layer = self.values[max_index - 1].clone();
    let weights = self.weights[max_index].clone();

    for (value, node_index) in self.values[max_index].iter_mut().zip(0..) {
      *value = weights[node_index as usize].clone().iter()
                                         .zip(values_of_previous_layer.clone())
                                         .map(|(w, v)| w * v)
                                         .sum::<f32>();
    }
  }

  fn set_all_layers_but_zeroth(&mut self) {
    let length = self.values.len();
    (1u16..((length - 1) as u16)).for_each(|index| self.set_layer_val(index));
  }

  pub fn set_zeroth_layer_from_input(&mut self, input: Vec<f32>) {
    let length: u16 = self.values[0 as usize].len() as u16;
    let values = (0u16..length).map(|index| self.get_node_sum_from_input(index, input.clone())).collect::<Vec<f32>>();
    self.values[0].iter_mut()
                  .zip(values)
                  .for_each(|(old, new)| *old = (self.act_fn)(new));
  }

  fn get_node_sum_from_input(&self, node_index: u16, input: Vec<f32>) -> f32 {
    let previous_weights = self.weights_in_immut(0, node_index);
    input.iter().zip(previous_weights)
         .map(|(v, w)| *v * *w)
         .sum::<f32>()
  }
  
}