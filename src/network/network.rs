use super::layer::Layer;

type InputBinaries = Vec<u8>;
type AF = fn(f64) -> f32;

pub struct Network {
  pub layers: Vec<Layer>,
  pub learning_rate: f32
}

impl Network {
  pub fn create_value_zero_random(number_of_layers: usize, nodes_per_layer: usize, a_fn: AF, l_r: f32) -> Network {
    Network {
      layers: vec![Layer::create_value_zero_random(nodes_per_layer, nodes_per_layer, a_fn); number_of_layers],
      learning_rate: l_r
    }
  }
}