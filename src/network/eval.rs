use crate::network::net::{Net, tanh, tanh_der_clipped};
use std::sync::Mutex;
use crate::rand::Rng;

lazy_static! {
  static ref NET: Mutex<Net> = Mutex::new(Net::create_random(vec![1f64; 768], 80, 20, tanh, tanh_der_clipped, 0.02));
}

pub fn run_random_inputs_through_net(number: usize) {
  let random_inputs = generate_random_inputs(number);
  for random_input in random_inputs.iter() {
    NET.lock().unwrap().run_data(convert_positions_to_input_layer(*random_input), 0.75f64);
  }
}

pub fn generate_random_inputs(number: usize) -> Vec<[u64; 13]> {
  let mut inputs: Vec<[u64; 13]> = Vec::new();
  for _input_number in 0..number {
    inputs.push(generate_random_input());
  }
  inputs
}

fn generate_random_input() -> [u64; 13] {
  let mut input: [u64; 13] = [0; 13];
  let mut rng = rand::thread_rng();
  for slice_index in 0..12 {
    input[slice_index] = rng.gen_range(0u64..0xFFFFFFFFFFFFFFFF);
  }
  input
}

pub fn convert_positions_to_input_layer(position: [u64; 13]) -> Vec<f64> {
  let mut input: Vec<f64> = Vec::new();
  let mut current_slice: u64;
  for slice_index in 0..12 {
    current_slice = position[slice_index];
    for _bit in 0..64 {
      if (current_slice & 1) != 0 {
        input.push(1f64);
      } else {
        input.push(0f64);
      }
      current_slice >>= 1;
    }
  }
  input
}

pub fn evaluate_position(_position: [u64; 13]) -> f32 {
  0.0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_position_to_layer() {
    let mut position: [u64; 13] = [0; 13];
    position[0] = 255;
    let converted = convert_positions_to_input_layer(position);
    println!("{:?}", converted);
    assert!(converted[0] == 1f64);
    assert!(converted[7] == 1f64);
    assert!(converted[8] == 0f64);
  }

  #[test]
  #[ignore]
  fn network_catches_on_to_repeated_value () {
    NET.lock().unwrap().set_network_values();
    println!("{:?}", NET.lock().unwrap().get_final_value());
    run_random_inputs_through_net(200);
    // tested manually, seems to work
    println!("{:?}", NET.lock().unwrap().get_final_value());
  }
}