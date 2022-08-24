use crate::r#move::{wstate, bstate, states_for_turn};
use crate::constants::*;
use std::collections::HashMap;
use std::cmp;
use rand::thread_rng;
use crate::rand::Rng;
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
    (BKING, -50),
    (12, 0)
  ]);
}

#[inline]
fn collect_points(state: [u64; 13]) -> f64 {
  state.iter().enumerate().map(|(index, slice)| BASIC_PIECE_TO_POINT[&(index as u8)] as f64 * number_of_bits(*slice) as f64).sum::<f64>()
}

pub fn basic_eval(state: [u64; 13]) -> f64 {
  collect_points(state)
}

pub fn random_eval(_state: [u64; 13]) -> f64 {
  let mut rng = rand::thread_rng();
  rng.gen_range(-50f64..50f64)
}

pub fn center_squares_worth(state: [u64; 13]) -> f64 {
  let occ_fns: Vec<fn([u64; 13]) -> u64> = vec![get_black_occupation_except_king, get_white_occupation_except_king, get_black_occupation_except_king, get_white_occupation_except_king];
  let squares: Vec<u64> = vec![CENTER_FOUR_SQUARES, CENTER_FOUR_SQUARES, SECOND_CENTER_SQUARES, SECOND_CENTER_SQUARES];
  let weights: Vec<f64> = vec![-0.5, 0.5, -0.2, 0.2];
  let center_squares_value: f64 = occ_fns.iter().zip(squares.iter()).zip(weights.iter()).map(|((occ, sqr), w)| (number_of_bits(occ(state) & sqr)) as f64 * w).sum::<f64>();
  collect_points(state) + center_squares_value
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

  pub fn get_state_quiet(&self, state: [u64; 13], turn_number: u8) -> [u64; 13] {
    if turn_number % 2 == 0 {
      self.get_state_black(state)
    } else {
      self.get_state_white(state)
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
    let possible_states: Vec<[u64; 13]> = states_for_turn(state, turn_number);

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