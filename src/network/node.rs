pub struct Node {
  pub value: f32,
  pub bias: f32,
  // weights -> **next** layer
  pub weights: Vec<f32>
}

impl Node {
  pub fn weight_at_index(&self, index: u8) -> f32 {
    self.weights[index as usize]
  }
}