use rand::Rng;

#[derive(Clone)]
pub struct Node {
  pub value: f32,
  pub bias: f32,
  // weights -> **previous** layer
  pub weights: Vec<f32>
}

impl Node {
  pub fn create(v: f32, b: f32, w: Vec<f32>) -> Node {
    Node {
      value: v, 
      bias: b, 
      weights: w
    }
  }

  pub fn create_random(length_of_previous_layer: usize) -> Node {
    Node {
      value: 0f32, 
      bias: rand::thread_rng().gen_range(-1.0..1.0), 
      weights: vec![rand::thread_rng().gen_range(-1.0..1.0); length_of_previous_layer]
    }
  }

  pub fn weight_at_index(&self, index: u8) -> f32 {
    self.weights[index as usize]
  }

  pub fn print(&self) {
    println!("Node~~");
    println!("  Value: {:.32}", self.value);
    println!("  Bias:  {:.32}", self.bias);
  }
}