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
  pub fn error(&self, target: f32) -> f32 {
    f32::powf(target - self.get_final_value(), 2f32)
  }

  pub fn backprop(&mut self, target: f32) {
    self.set_all_layer_influences(target);
    self.adjust_all_weights();
  }

  fn adjust_all_weights(&mut self) {
    let length: u16 = (self.values.len() - 1) as u16;
    (0u16..length).for_each(|index| self.adjust_weights_in_layer(index));
  }

  fn adjust_weights_in_layer(&mut self, layer_index: u16) {
    let influences: Vec<f32> = self.influence[(layer_index + 1) as usize].clone();
    let vals: Vec<f32> = self.values[layer_index as usize].clone();
    let lr: f32 = self.learning_rate.clone();

    for (val, val_index) in vals.iter().zip(0..) {
      for (inf, influence_index) in influences.iter().zip(0..) {
        self.weights[layer_index as usize][val_index as usize][influence_index as usize] += lr * (self.der_fn)(*val) * inf;
      }
    }
  }

  fn set_all_layer_influences(&mut self, target: f32) {
    self.set_starting_influence(target);
    (0u16..(self.values.len()) as u16).rev().for_each(|index| self.set_layer_influence(index));
  }

  fn set_layer_influence(&mut self, layer_index: u16) {
    let influences = self.get_layer_influence(layer_index);
    self.influence[layer_index as usize].iter_mut().zip(influences).for_each(|(old, new)| *old = new);
  }

  fn get_layer_influence(&self, layer_index: u16) -> Vec<f32> {
    let mut influences: Vec<f32> = vec![0f32; self.values[layer_index as usize].len()];
    let mut val_from_der_fn: f32;
    let old_influences = self.get_already_calculated_layer_influence(layer_index);

    for (node, index) in self.vals_immut(layer_index).zip(0..) {
      val_from_der_fn = (self.der_fn)(*node);
      for (weight, infl) in self.weights_in_immut(layer_index, index).zip(old_influences.clone()) {
        influences[index as usize] += *weight * *infl * val_from_der_fn;
      }
    }
    influences
  }

  fn get_already_calculated_layer_influence(&self, layer_index: u16) -> std::slice::Iter<f32> {
    //println!("Index {}: {:?}\n", layer_index, self.influence[(layer_index + 1) as usize].iter().clone().collect::<Vec<&f32>>());
    self.influence[(layer_index + 1) as usize].iter()
  }

  fn set_starting_influence(&mut self, target: f32) {
    let num_layers = self.influence.len();
    let infl = self.starting_influence(target);
    self.influence[num_layers - 1][0] = infl;
  }

  fn starting_influence(&self, target: f32) -> f32 {
    (self.der_fn)(target - self.get_final_value())
  }
}

/*
#[cfg(test)]
mod test {
  use super::*;

  fn sigmoid(input: f32) -> f32 {
    1f32 / (1f32 + f32::powf(std::f64::consts::E as f32, -input))
  }

  fn sigmoid_der(input: f32) -> f32 {
    input * (1f32 - input)
  }

  fn leaky_ReLu(input: f32) -> f32 {
    if input > 0f32 { input } else { input * 0.2f32 }
  }

  fn leaky_ReLu_der(input: f32) -> f32 {
    if input > 0f32 { 1f32 } else { 0.2f32 }
  }

  fn linear(input: f32) -> f32 {
    input
  }

  fn constant(_input: f32) -> f32 {
    1f32
  }

  #[test]
  fn creates_nn() {
    let mut nn: NN = NN::create_random_wb(31, 31, sigmoid, sigmoid_der, 0.001f32);
    /*
    for layer in nn.values.iter() {
      println!("{:?}", layer);
    }
    for layer in nn.weights.iter() {
      for node in layer.iter() {
        println!("Items in layer: {}", node.clone().len());
        println!("{:?}", node);
      }
    }
    */
    let first_prop = nn.forwardprop(vec![1f32; 768]);
    //for layer in nn.values.iter() {
    //  println!("{:?}", layer);
    //}
    //for layer in nn.weights.iter() {
    //  for node in layer.iter() {
    //    println!("Items in layer: {}", node.clone().len());
    //    println!("{:?}", node);
    //  }
    //}
    //for layer in nn.values.iter() {
    //  println!("{:?}", layer);
    //}
    let target: f32 = -100f32;

    let first_error = nn.error(target);
    nn.backprop(target);
    for _i in 0..30 {
      nn.forwardprop(vec![1f32; 768]);
      nn.backprop(target);
    }

    let second_prop = nn.forwardprop(vec![1f32; 768]);
    //for layer in nn.values.iter() {
    //  println!("{:?}", layer);
    //}
    /*
    for layer in nn.weights.iter() {
      for node in layer.iter() {
        println!("Items in layer: {}", node.clone().len());
        println!("{:?}", node);
      }
    }
    */

    //for (i, index) in nn.influence.iter().zip(0..) {
    //  println!("Index {}: {:?}\n", index, *i);
    //}

    println!("First prop: {:.32}", first_prop);
    println!("  Error: {:.32}", first_error);
    println!("Second prop: {:.32}", second_prop);
    println!("  Error: {:.32}", nn.error(target));
  
    
    
      /*
    for i in nn.values.iter() {
      println!("{:?}\n", *i);
    }*/

    /*

    for i in nn.biases.iter() {
      println!("{:?}\n", *i);
    }
    */

    //assert!(false);
  }
}
*/