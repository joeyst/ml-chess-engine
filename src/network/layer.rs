use super::node::Node;

pub struct Layer {
  nodes: Vec<Node>,
  activation_fn: fn(f64) -> f32
}

impl Layer {
  pub fn create(n: Vec<Node>, a_fn: fn(f64) -> f32) -> Layer {
    Layer {
      nodes: n,
      activation_fn: a_fn
    }
  }

  pub fn create_random_layer(nodes_in_layer: usize, nodes_in_next_layer: usize, a_fn: fn(f64) -> f32) -> Layer {
    Layer::create(vec![Node::create_random(nodes_in_next_layer); nodes_in_layer], a_fn)
  }

  pub fn collect_weights(&self, node_index: u8) -> Vec<f32> {
    self.nodes[node_index as usize].weights.clone()
  }
}