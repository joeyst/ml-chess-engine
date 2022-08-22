use super::layer::Layer;
use super::node::Node;

type InputBinaries = Vec<u8>;
type AF = fn(f64) -> f32;
type ListOfLayers = Vec<Layer>;

pub struct Network {
  pub layers: ListOfLayers,
  pub learning_rate: f32,
  pub input_to_network: Vec<u8>
}

impl Network {
  pub fn get_output(&mut self) -> f32 {
    self.set_values_for_network();
    self.get_value_of_final_node()
  }

  fn get_value_of_final_node(&self) -> f32 {
    self.layers.last().unwrap().nodes[0].value
  }

  fn set_values_for_network(&mut self) {
    for layer_index in 0u8..(self.layers.len()) as u8 {
      self.set_values_for_layer(layer_index);
    }
  }

  fn set_values_for_layer(&mut self, layer_index: u8) {
    let values: Vec<f32> = self.calculate_values_for_layer(layer_index);
    let nodes: &mut Vec<Node> = &mut self.get_layer_reference(layer_index).nodes;
    for (node, new_value) in nodes.iter_mut().zip(values.iter()) {
      node.value = *new_value;
    }
  }

  fn set_values_for_layer_from_vector(&mut self, layer_index: u8, values: Vec<f32>) {
    let nodes: &mut Vec<Node> = &mut self.get_layer_reference(layer_index).nodes;
    for (node, new_value) in nodes.iter_mut().zip(values.iter()) {
      node.value = *new_value;
    }
  }

   pub fn create_random_with_input(number_of_layers: usize, nodes_per_layer: usize, a_fn: AF, l_r: f32, itn: Vec<u8>) -> Network {
    let mut layers: ListOfLayers = vec![Layer::create_value_zero_random(nodes_per_layer, 0, a_fn); number_of_layers];
    let first_layer: Layer = Layer::create_value_zero_random(nodes_per_layer, 768, a_fn);
    layers[0] = first_layer;
    for layer_number in 1..number_of_layers {
      layers[layer_number] = Layer::create_value_zero_random(nodes_per_layer, nodes_per_layer, a_fn);
    }
    layers.append(&mut vec![Layer::create_value_zero_random(1, nodes_per_layer, a_fn)]);

    Network {
      layers: layers,
      learning_rate: l_r,
      input_to_network: itn
    }
  }

  pub fn create_random(number_of_layers: usize, nodes_per_layer: usize, a_fn: AF, l_r: f32) -> Network {
    let mut layers: ListOfLayers = vec![Layer::create_value_zero_random(nodes_per_layer, 0, a_fn); number_of_layers];
    let first_layer: Layer = Layer::create_value_zero_random(nodes_per_layer, 768, a_fn);
    layers[0] = first_layer;
    for layer_number in 1..number_of_layers {
      layers[layer_number] = Layer::create_value_zero_random(nodes_per_layer, nodes_per_layer, a_fn);
    }
    layers.append(&mut vec![Layer::create_value_zero_random(1, nodes_per_layer, a_fn)]);

    Network {
      layers: layers,
      learning_rate: l_r,
      input_to_network: vec![0; 768]
    }
  }

  fn calculate_values_for_layer(&self, layer_index: u8) -> Vec<f32> {
    (0u8..self.get_layer_nodes(layer_index).len() as u8)
                      .map(|node_index| self.calculate_value_for_node(layer_index, node_index))
                      .collect::<Vec<f32>>()
  }

  

  fn calculate_value_for_node(&self, layer_index: u8, node_index: u8) -> f32 {
    match layer_index {
      0 => self.calculate_value_for_node_of_first_layer(layer_index, node_index),
      _ => self.calculate_value_for_node_beyond_first_layer(layer_index, node_index)
    }
  }

  #[inline]
  fn calculate_value_for_node_of_layer_general(&self, layer_index: u8, node_index: u8, inputs: Vec<f32>) -> f32 {
    let sum: f64 = self.get_weights_of_node(layer_index, node_index).iter()
                      .zip(inputs)
                      .map(|(w, v)| (*w as f64) * (v as f64))
                      .sum();
    
    let bias: f64 = self.get_bias_of_node(layer_index, node_index) as f64;
    let activation_function: AF = self.get_activation_function(layer_index);

    (activation_function)(sum + bias)
  }

  fn calculate_value_for_node_beyond_first_layer(&self, layer_index: u8, node_index: u8) -> f32 {
    self.calculate_value_for_node_of_layer_general(layer_index, node_index, self.get_values_of_old_layer(layer_index))
  }

  fn calculate_value_for_node_of_first_layer(&self, layer_index: u8, node_index: u8) -> f32 {
    let input: Vec<f32> = self.get_input_to_network().into_iter().map(|val| val as f32).collect::<Vec<f32>>();
    self.calculate_value_for_node_of_layer_general(layer_index, node_index, input)
  }

  fn get_values_of_old_layer(&self, layer_index: u8) -> Vec<f32> {
    self.layers[layer_index as usize].collect_values()
  }

  fn get_weights_of_node(&self, layer_index: u8, node_index: u8) -> Vec<f32> {
    self.layers[layer_index as usize].collect_weights_by_index(node_index)
  }

  fn get_bias_of_node(&self, layer_index: u8, node_index: u8) -> f32 {
    self.layers[layer_index as usize].get_bias_by_index(node_index)
  }

  fn get_activation_function(&self, layer_index: u8) -> AF {
    self.layers[layer_index as usize].activation_fn
  }

  #[inline(always)]
  fn get_layer(&self, layer_index: u8) -> Layer {
    self.layers[layer_index as usize].clone()
  }

  #[inline]
  fn get_layer_reference(&mut self, layer_index: u8) -> &mut Layer {
    &mut self.layers[layer_index as usize]
  }

  #[inline(always)]
  fn get_layer_nodes(&self, layer_index: u8) -> Vec<Node> {
    self.get_layer(layer_index).nodes
  }

  #[inline(always)]
  fn get_input_to_network(&self) -> Vec<u8> {
    self.input_to_network.clone()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn relu(input: f64) -> f32 {
    if input > 0f64 { input as f32 } else { 0f32 }
  }

  mod set_values_for_layer_tests {
    use super::*;
    #[test]
    fn successfully_mutates_node_values() {
      let mut network: Network = Network::create_random(4, 10, relu, 1f32);
      let values_before: Vec<f32> = network.get_layer(1).collect_values();
      network.set_values_for_layer_from_vector(0, vec![1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32, 1f32]);
      network.set_values_for_layer(1);
      let mut values_after: Vec<f32> = network.get_layer(1).collect_values();
      values_after[0] = 1f32;
      print!("\n\n");
      println!("Before: {:?}", values_before);
      println!("After: {:?}", values_after);
      print!("\n\n");
      assert_ne!(values_before, values_after);
    }

    #[test]
    fn creates_random_biases() {
      let network: Network = Network::create_random(3, 10, relu, 1f32);
      let bias1 = network.get_layer(1).collect_biases();
      let bias2 = network.get_layer(2).collect_biases();
      print!("\n\n");
      println!("Layer 1 biases: {:?}", bias1);
      println!("Layer 2 biases: {:?}", bias2);
      print!("\n\n");
      assert_ne!(bias1, bias2);
    }

    #[test]
    fn creates_random_weights() {
      let network: Network = Network::create_random(3, 10, relu, 1f32);
      let weights1 = network.get_weights_of_node(1, 0);
      let weights2 = network.get_weights_of_node(1, 1);
      let weights3 = network.get_weights_of_node(0, 0);

      print!("\n\n");
      println!("Node 1 weights: {:?}", weights1);
      println!("Node 2 weights: {:?}", weights2);
      println!("Node 3 weights: {:?}", weights3);
      print!("\n\n");
      assert_ne!(weights1, weights2);
      assert_ne!(weights2, weights3);
    }
  }

  #[test]
  fn ends_with_layer_of_single_node() {
    let network: Network = Network::create_random(1, 10, relu, 1f32);
    assert_eq!(network.layers.last().unwrap().nodes.len(), 1);
  }
}