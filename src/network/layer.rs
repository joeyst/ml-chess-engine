use super::node::Node;

pub struct Layer {
  nodes: Vec<Node>,
  activation_fn: fn(f64) -> f32,
  pub layer_number: u8
}

impl Layer {
  pub fn create(n: Vec<Node>, a_fn: fn(f64) -> f32, l_n: u8) -> Layer {
    Layer {
      nodes: n,
      activation_fn: a_fn,
      layer_number: l_n
    }
  }

  pub fn collect_weights(&self, node_index: u8) -> Vec<f32> {
    self.nodes[node_index as usize].weights.clone()
  }
}