struct NN {
  values: Vec<Vec<f32>>,
  biases: Vec<Vec<f32>>,
  weights: Vec<Vec<Vec<f32>>>,
  act_fn: fn(f32) -> f32,
  der_fn: fn(f32) -> f32
}

impl NN {
  pub fn create(nodes: u16, layers: u16, a_fn: fn(f32) -> f32, d_fn: fn(f32) -> f32) -> NN {
    NN {
      values: vec![vec![0f32; nodes as usize]; layers as usize],
      biases: vec![vec![0f32; nodes as usize]; layers as usize],
      weights: vec![vec![vec![0f32; (nodes * layers) as usize]; nodes as usize]; layers as usize],
      act_fn: a_fn,
      der_fn: d_fn
    }
  }
}