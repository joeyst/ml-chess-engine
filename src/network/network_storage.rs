use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;
use super::net::{Net, tanh, tanh_der_clipped};
use super::eval::NET;
use std::io::{Error, SeekFrom};
use std::collections::HashMap;

pub fn get_network_from_file(path: &str) -> Net {
  match std::fs::read_to_string(path) {
    Ok(string) => return get_network_from_string(string),
    Err(_) => NET.lock().unwrap().clone()
  }
}

fn is_hidden(text: &str) -> bool {
  text[0..6] == *"Hidden"
}

fn is_nodes_per_layer(text: &str) -> bool {
  text[0..5] == *"Nodes"
}

fn is_act_function(text: &str) -> bool {
  text[0..3] == *"Act"
}

fn is_der_function(text: &str) -> bool {
  text[0..3] == *"Der"
}

fn is_learning_rate(text: &str) -> bool {
  text[0..2] == *"Lr"
}

fn is_input_value(text: &str) -> bool {
  text[0..5] == *"Input"
}

fn is_weight(text: &str) -> bool {
  text[0..6] == *"Weight"
}

fn is_bias(text: &str) -> bool {
  text[0..4] == *"Bias"
}

fn parse_input_line(full_text: &str) -> (usize, f64) {
  let number_text = &full_text[6..];
  let numbers = number_text.split(" ")
                           .collect::<Vec<&str>>();
  (numbers[0].parse::<usize>().unwrap(), numbers[1].parse::<f64>().unwrap())
}

fn parse_weight(full_text: &str) -> (usize, usize, usize, f64) {
  let number_text = &full_text[7..];
  let numbers = number_text.split(" ")
                           .collect::<Vec<&str>>();
  (
    numbers[0].parse::<usize>().unwrap(),
    numbers[1].parse::<usize>().unwrap(),
    numbers[2].parse::<usize>().unwrap(),
    numbers[3].parse::<f64>().unwrap()
  )
}

fn parse_bias(full_text: &str) -> (usize, usize, f64) {
  let number_text = &full_text[5..];
  let numbers = number_text.split(" ")
                           .collect::<Vec<&str>>();
  (
    numbers[0].parse::<usize>().unwrap(),
    numbers[1].parse::<usize>().unwrap(),
    numbers[2].parse::<f64>().unwrap()
  )
}

lazy_static! {
  static ref NAME_TO_FN: HashMap<&'static str, fn(f64) -> f64> = HashMap::from([
    ("tanh", tanh as fn(f64) -> f64),
    ("tanh_der_clipped", tanh_der_clipped as fn(f64) -> f64)
  ]);
}

fn get_network_from_string(string: String) -> Net {
  let lines: Vec<_> = string.lines().collect();
  let mut nodes_per_hidden_layer: usize = 0;
  let mut number_of_hidden_layers: usize = 0;
  let mut act_fn: fn(f64) -> f64 = NAME_TO_FN["tanh"];
  let mut der_fn: fn(f64) -> f64 = NAME_TO_FN["tanh_der_clipped"];
  let mut lr: f64 = 0.2;

  assert!(is_hidden("Hidden 90"));

  let mut input_count = 0;
  for line in &lines {
    if is_hidden(line) { 
      number_of_hidden_layers = line[7..].parse::<usize>().unwrap();
    } 
    else if is_nodes_per_layer(line) {
      nodes_per_hidden_layer = line[6..].parse::<usize>().unwrap();
    }
    else if is_act_function(line) {
      act_fn = NAME_TO_FN[&line[4..]];
    }
    else if is_der_function(line) {
      der_fn = NAME_TO_FN[&line[4..]];
    }
    else if is_learning_rate(line) {
      lr = line[3..].parse::<f64>().unwrap();
    }
    else if is_input_value(line) {
      input_count += 1;
    }
  }

  let mut input_values: Vec<f64> = vec![0f64; input_count];
  let mut index_and_value: (usize, f64);
  for line in &lines {
    if is_input_value(line) {
      index_and_value = parse_input_line(line);
      input_values[index_and_value.0] = index_and_value.1;
    }
  }
  
  let mut network: Net = Net::create(input_values, number_of_hidden_layers, nodes_per_hidden_layer, act_fn, der_fn, lr);

  // weight indices and value
  let mut w: (usize, usize, usize, f64);
  // bias indices and value
  let mut b: (usize, usize, f64);

  for line in &lines {
    if is_weight(line) {
      w = parse_weight(line);
      network.weights[w.0][w.1][w.2] = w.3;
    } 
    else if is_bias(line) {
      b = parse_bias(line);
      network.biases[b.0][b.1] = b.2;
    }
  }
  network
}

pub fn write_network_to_file(network: Net, act_fn_name: &str, der_fn_name: &str, path: &str) {
  assert!(network.values.len() > 2, "Network must have at least one hidden layer. network_storage.rs, network_to_string");
  let nodes_per_hidden_layer = network.values[1].len();
  let number_of_hidden_layers = network.values.len() - 2;
  
  let mut input_values_string = String::new();
  for (_value, value_index) in network.values[0].iter().zip(0..) {
    input_values_string.push_str("Input ");
    input_values_string.push_str(&value_index.to_string());
    input_values_string.push_str(" ");
    input_values_string.push_str(&network.values[0][value_index].to_string());
    input_values_string += "\n";
  }

  let mut weights_string = String::new();
  for (layer, layer_index) in network.weights.iter().zip(0..) {
    for (node, node_index) in layer.iter().zip(0..) {
      for (_weight, weight_index) in node.iter().zip(0..) {
        weights_string.push_str("Weight ");
        weights_string.push_str(&layer_index.to_string());
        weights_string.push_str(" ");
        weights_string.push_str(&node_index.to_string());
        weights_string.push_str(" ");
        weights_string.push_str(&weight_index.to_string());
        weights_string.push_str(" ");
        weights_string.push_str(&network.weights[layer_index][node_index][weight_index].to_string());
        weights_string += "\n";
      }
    }
  }
  
  let mut biases_string = String::new();
  for (layer, layer_index) in network.biases.iter().zip(0..) {
    for (_node, node_index) in layer.iter().zip(0..) {
      biases_string.push_str("Bias ");
      biases_string.push_str(&layer_index.to_string());
      biases_string.push_str(" ");
      biases_string.push_str(&node_index.to_string());
      biases_string.push_str(" ");
      biases_string.push_str(&network.biases[layer_index][node_index].to_string());
      biases_string += "\n";
    }
  }

  let mut net: String = String::new();
  net.push_str("Hidden ");
  net.push_str(&number_of_hidden_layers.to_string());
  net += "\n";
  net.push_str("Nodes ");
  net.push_str(&nodes_per_hidden_layer.to_string());
  net += "\n";
  net.push_str("Act ");
  net.push_str(&act_fn_name);
  net += "\n";
  net.push_str("Der ");
  net.push_str(&der_fn_name);
  net += "\n";
  net.push_str("Lr ");
  net.push_str(&network.learning_rate.to_string());
  net += "\n";

  net.push_str(&input_values_string);
  net.push_str(&weights_string);
  net.push_str(&biases_string);

  clear_file(path);
  std::fs::write(path, net).expect("Unable to write file");
}

fn clear_file(path: &str) {
  match std::fs::remove_file(path) {
    Ok(removed) => removed,
    Err(_) => {}
  };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn writes_network_to_string() {
    let input_values = vec![0f64; 2];
    let network = Net::create(input_values, 2, 2, tanh, tanh_der_clipped, 0.2);
    write_network_to_file(network, "tanh", "tanh_der_clipped", "test_network_storage.txt");
    get_network_from_file("test_network_storage.txt");
  }

  #[test]
  #[ignore]
  fn converts_network_to_file_and_back() {
    let input_values = vec![0f64; 2];
    let original_network = Net::create(input_values, 2, 2, tanh, tanh_der_clipped, 0.2);
    write_network_to_file(original_network.clone(), "tanh", "tanh_der_clipped", "text_network_storage.txt");
    let mut new_net = get_network_from_file("test_network_storage.txt");
    new_net.weights[2][1][0] = 0.0f64;
    // println!("{:?}", new_net);
    println!("Race conditions resulting in default network being returned.");
    assert!(original_network == new_net);
  }
}