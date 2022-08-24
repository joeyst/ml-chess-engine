use crate::{bot, game};
use crate::game::{RANDOM_BOT, play_engine_turn_quiet};
use super::{net, network_storage};
use super::network_storage::{get_network_from_file, write_network_to_file};
use std::collections::HashMap;
use super::eval::convert_positions_to_input_layer;
use crate::network::net::Net;

lazy_static! {
  pub static ref NET_FOR_EVAL: Net = get_network_from_file("text_network_storage.txt");
}

pub fn learn_bot_eval(state: [u64; 13]) -> f64 {
  let input = convert_positions_to_input_layer(state);
  NET_FOR_EVAL.clone().forward_prop_to_value(input)
}

pub fn train_network_with_games(number_of_games: usize) {
  let mut training_game_number = 0;
  let mut net = get_network_from_file("text_network_storage.txt");
  let mut states_and_evaluations: HashMap<[u64; 13], f64>;
  for _game_number in 0..number_of_games {
    println!("Training game #{}", training_game_number);
    training_game_number += 1;
    states_and_evaluations = get_random_game_states_with_adjustments();
    for (state, eval) in states_and_evaluations {
      net.run_data(convert_positions_to_input_layer(state), eval);
    }
  }
  write_network_to_file(net, "tanh", "tanh_der_clipped", "text_network_storage.txt");
}

fn train_network_with_one_game() {
  let mut net = get_network_from_file("text_network_storage.txt");
  let states_and_evaluations = get_random_game_states_with_adjustments();
  for (state, eval) in states_and_evaluations {
    net.run_data(convert_positions_to_input_layer(state), eval);
  }
}

fn normalize_evaluation(eval: f64) -> f64 {
  if eval > 15f64 {
    1f64
  } 
  else if eval < -15f64 {
    -1f64
  }
  else {
    eval / 15f64
  }
}

fn get_random_game_states_with_adjustments() -> HashMap<[u64; 13], f64> {
  let mut states: Vec<[u64; 13]> = Vec::new();
  let mut state = game::setup_board();
  let mut turn_number: u8 = 1;

  for _move_number in 1..31 {
    play_engine_turn_quiet(&RANDOM_BOT, &mut state, &mut turn_number);
    states.push(state.clone());
  }
  
  let mut states_and_values: HashMap<[u64; 13], f64> = HashMap::new();
  let final_evaluation = bot::center_squares_worth(states[states.len() - 1]);
  for (state, index) in states.iter().zip(0..) {
    states_and_values.insert(*state, normalize_evaluation(final_evaluation * 0.75f64.powf(index as f64)));
  }
  states_and_values
}