use super::node::Node;

type AF = fn(f64) -> f32;
type ListOfNodes = Vec<Node>;
type BiasesForWholeLayer = Vec<f32>;
type WeightsForWholeLayer = Vec<Vec<f32>>;

#[derive(Clone)]
pub struct Layer {
  nodes: Vec<Node>,
  activation_fn: AF
}

impl Layer {
  pub fn create(n: Vec<Node>, a_fn: AF) -> Layer {
    Layer {
      nodes: n,
      activation_fn: a_fn
    }
  }

  // value = 0, bias = random of -1.0..1.0, weight in weights = random of -1.0..1.0
  pub fn create_value_zero_random(nodes_in_layer: usize, nodes_in_previous_layer: usize, a_fn: AF) -> Layer {
    Layer::create(vec![Node::create_random(nodes_in_previous_layer); nodes_in_layer], a_fn)
  }

  pub fn create_layer_from_weights_and_biases(weights: WeightsForWholeLayer, biases: BiasesForWholeLayer, a_fn: AF) -> Layer {
    assert_eq!(weights.len(), biases.len());

    Self::create(
      weights.into_iter().zip(biases.into_iter()).map(|(w, b)| Node::create(0f32, b, w)).collect::<ListOfNodes>(), 
      a_fn
    )
  }

  pub fn collect_weights_by_index(&self, node_index: u8) -> Vec<f32> {
    self.nodes[node_index as usize].weights.clone()
  }
}