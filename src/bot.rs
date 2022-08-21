use crate::r#move::{wstate, bstate, states_for_turn};
use crate::constants::*;
use std::collections::HashMap;
use std::cmp;
use rand::thread_rng;
use crate::rand::prelude::SliceRandom;
use crate::rand::prelude::IteratorRandom;
use crate::utility::{greater_than, less_than, min_f, max_f, number_of_bits};
use crate::board::*;
use crate::{print_board, print_board_pieces};


pub struct Bot {
  eval_fn: fn([u64; 13]) -> f64,
  depth: u8
}

lazy_static! {
  static ref BASIC_PIECE_TO_POINT: HashMap<u8, i8> = HashMap::from([
    (WROOK, 5),
    (WBISHOP, 3),
    (WQUEEN, 8),
    (BROOK, -5),
    (BBISHOP, -3),
    (BQUEEN, -8),

    (WPAWN, 1),
    (WKNIGHT, 3),
    (WKING, 50),
    (BPAWN, -1),
    (BKNIGHT, -3),
    (BKING, -50)
  ]);
}

pub fn basic_eval(mut state: [u64; 13]) -> f64 {
  let mut count: f64 = 0.0;
  for slice_index in 0..12 {
    while state[slice_index as usize] != 0 {
      count += BASIC_PIECE_TO_POINT[&slice_index] as f64;
      state[slice_index as usize] &= state[slice_index as usize] - 1;
    }
  }
  count
}

pub fn center_squares_worth(mut state: [u64; 13]) -> f64 {
  let mut count: f64 = 0.0;
  let mut temp_state = state.clone();
  for slice_index in 0..12 {
    while temp_state[slice_index as usize] != 0 {
      count += BASIC_PIECE_TO_POINT[&slice_index] as f64;
      temp_state[slice_index as usize] &= temp_state[slice_index as usize] - 1;
    }
  }

  let b_center_occ: u64 = get_black_occupation_except_king(state) & CENTER_FOUR_SQUARES;
  let w_center_occ: u64 = get_white_occupation_except_king(state) & CENTER_FOUR_SQUARES;
  let b_second_center_occ: u64 = get_black_occupation_except_king(state) & SECOND_CENTER_SQUARES;
  let w_second_center_occ: u64 = get_white_occupation_except_king(state) & SECOND_CENTER_SQUARES;

  count += number_of_bits(b_center_occ) as f64 * -0.5;
  count += number_of_bits(w_center_occ) as f64 * 0.5;
  count += number_of_bits(b_second_center_occ) as f64 * -0.2;
  count += number_of_bits(w_second_center_occ) as f64 * 0.2;
  
  count
}

pub fn make_bot(eval_function: fn([u64; 13]) -> f64, d: u8) -> Bot {
  Bot {
    eval_fn: eval_function,
    depth: d
  }
}

impl Bot {
  pub fn get_state(&self, state: [u64; 13], turn_number: u8) -> [u64; 13] {
    if turn_number % 2 == 0 {
      let new_state: [u64; 13] = self.get_state_black(state);
      println!("Evaluation: {:.32}", (self.eval_fn)(new_state));
      new_state
    } else {
      let new_state: [u64; 13] = self.get_state_white(state);
      println!("Evaluation: {:.32}", (self.eval_fn)(new_state));
      new_state
    }
  }

  

  fn get_state_general(&self, state: [u64; 13], turn_number: u8, more_or_less: fn(f64, f64) -> bool, starting_value: f64) -> [u64; 13] {
    let mut possible_states: Vec<[u64; 13]> = states_for_turn(state, turn_number);
    let mut candidate_state: [u64; 13] = [0; 13];
    let mut best_eval: f64 = starting_value;
    possible_states.shuffle(&mut thread_rng());
    let mut evaluation: f64;
    for possible_state in possible_states.iter() {
      evaluation = self.minimax(*possible_state, turn_number + 1, 0, -10000.0, 10000.0);
      if more_or_less(evaluation, best_eval) {
        candidate_state = *possible_state;
        best_eval = evaluation;
      }
    }

    candidate_state
  }

  fn get_state_white(&self, state: [u64; 13]) -> [u64; 13] {
    self.get_state_general(state, 1, greater_than, -10000.0)
  }

  fn get_state_black(&self, state: [u64; 13]) -> [u64; 13] {
    self.get_state_general(state, 0, less_than, 10000.0)
  }

  fn minimax(&self, state: [u64; 13], turn_number: u8, depth_gone: u8, mut alpha: f64, mut beta: f64) -> f64 {
    if depth_gone == self.depth {
      return (self.eval_fn)(state)
    }
    let mut possible_states: Vec<[u64; 13]> = states_for_turn(state, turn_number);

    if turn_number % 2 == 1 {
      let mut max: f64 = -10000.0;
      let mut current_value: f64;
      
      for p_state in possible_states {
        current_value = self.minimax(p_state, turn_number + 1, depth_gone + 1, alpha, beta);
        max = max_f(max, current_value);
        alpha = max_f(alpha, max);

        if alpha >= beta {
          break;
        }
      }
      max

    } else {
      let mut min: f64 = 10000.0;
      let mut current_value: f64;

      for p_state in possible_states {
        current_value = self.minimax(p_state, turn_number + 1, depth_gone + 1, alpha, beta);
        min = min_f(min, current_value);
        beta = min_f(beta, min);

        if alpha >= beta {
          break;
        }
      }
      min
    }    
  }
}