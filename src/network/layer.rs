use super::node::Node;

type AF = fn(f64) -> f32;
type ListOfNodes = Vec<Node>;
type BiasesForWholeLayer = Vec<f32>;
type WeightsForWholeLayer = Vec<Vec<f32>>;

#[derive(Clone)]
pub struct Layer {
  pub nodes: Vec<Node>,
  pub activation_fn: AF
}

impl Layer {
  // constructors
  pub fn create(n: Vec<Node>, a_fn: AF) -> Layer {
    Layer {
      nodes: n,
      activation_fn: a_fn
    }
  }

        // value = 0, bias = random of -1.0..1.0, weight in weights = random of -1.0..1.0
  pub fn create_value_zero_random(nodes_in_layer: usize, nodes_in_previous_layer: usize, a_fn: AF) -> Layer {
    let mut nodes: Vec<Node> = vec![Node::create_random(0); nodes_in_layer];
    for index in 0..nodes_in_layer {
      nodes[index] = Node::create_random(nodes_in_previous_layer);
    }
    Layer::create(nodes, a_fn)
  }

  pub fn create_layer_from_weights_and_biases(weights: WeightsForWholeLayer, biases: BiasesForWholeLayer, a_fn: AF) -> Layer {
    assert_eq!(weights.len(), biases.len());

    Self::create(
      weights.into_iter().zip(biases.into_iter()).map(|(w, b)| Node::create(0f32, b, w)).collect::<ListOfNodes>(), 
      a_fn
    )
  }

  // aggregate
  pub fn collect_values(&self) -> Vec<f32> {
    self.nodes.iter().map(|node| node.value).collect::<Vec<f32>>()
  }

  pub fn collect_biases(&self) -> Vec<f32> {
    self.nodes.iter().map(|node| node.bias).collect::<Vec<f32>>()
  }

  pub fn collect_mutable_nodes(&mut self) -> &mut Vec<Node> {
    &mut self.nodes
  }

  // singular
  pub fn get_current_value_by_index(&self, node_index: u8) -> f32 {
    self.nodes[node_index as usize].value
  }

  pub fn get_bias_by_index(&self, node_index: u8) -> f32 {
    self.nodes[node_index as usize].bias
  }

  pub fn collect_weights_by_index(&self, node_index: u8) -> Vec<f32> {
    self.nodes[node_index as usize].weights.clone()
  }
}