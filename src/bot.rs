use crate::r#move::{wstate, bstate, states_for_turn};
use crate::constants::*;
use std::collections::HashMap;
use std::cmp;
use rand::thread_rng;
use crate::rand::prelude::SliceRandom;
use crate::rand::prelude::IteratorRandom;
use crate::utility::{greater_than, less_than};


pub struct Bot {
  eval_fn: fn([u64; 13]) -> i16,
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

pub fn basic_eval(mut state: [u64; 13]) -> i16 {
  let mut count: i16 = 0;
  for slice_index in 0..12 {
    while state[slice_index as usize] != 0 {
      count += BASIC_PIECE_TO_POINT[&slice_index] as i16;
      state[slice_index as usize] &= state[slice_index as usize] - 1;
    }
  }
  count
}

pub fn make_bot(eval_function: fn([u64; 13]) -> i16, d: u8) -> Bot {
  Bot {
    eval_fn: eval_function,
    depth: d
  }
}

impl Bot {
  pub fn get_state(&self, state: [u64; 13], turn_number: u8) -> [u64; 13] {
    if turn_number % 2 == 0 {
      let new_state: [u64; 13] = self.get_state_black(state);
      println!("Evaluation: {}", (self.eval_fn)(new_state));
      new_state
    } else {
      let new_state: [u64; 13] = self.get_state_white(state);
      println!("Evaluation: {}", (self.eval_fn)(new_state));
      new_state
    }
  }

  

  fn get_state_general(&self, state: [u64; 13], turn_number: u8, more_or_less: fn(i16, i16) -> bool, starting_value: i16) -> [u64; 13] {
    let mut possible_states: Vec<[u64; 13]> = states_for_turn(state, turn_number);
    let mut candidate_state: [u64; 13] = [0; 13];
    let mut best_eval: i16 = starting_value;
    possible_states.shuffle(&mut thread_rng());

    for possible_state in possible_states.iter() {
      if more_or_less((self.eval_fn)(*possible_state), best_eval) {
        best_eval = self.minimax(*possible_state, turn_number + 1, 0, -10000, 10000);
        candidate_state = *possible_state;
      }
    }

    candidate_state
  }

  fn get_state_white(&self, state: [u64; 13]) -> [u64; 13] {
    self.get_state_general(state, 1, greater_than, -10000)
  }

  fn get_state_black(&self, state: [u64; 13]) -> [u64; 13] {
    self.get_state_general(state, 0, less_than, 10000)
  }

  fn minimax(&self, state: [u64; 13], turn_number: u8, depth_gone: u8, mut alpha: i16, mut beta: i16) -> i16 {
    if depth_gone == self.depth {
      return (self.eval_fn)(state)
    }
    let mut possible_states: Vec<[u64; 13]> = states_for_turn(state, turn_number);

    if turn_number % 2 == 1 {
      let mut max: i16 = -10000;
      let mut current_value: i16;
      
      for p_state in possible_states {
        current_value = self.minimax(p_state, turn_number + 1, depth_gone + 1, alpha, beta);
        max = cmp::max(max, current_value);
        alpha = cmp::max(alpha, max);

        if alpha >= beta {
          break;
        }
      }
      max

    } else {
      let mut min: i16 = 10000;
      let mut current_value: i16;

      for p_state in possible_states {
        current_value = self.minimax(p_state, turn_number + 1, depth_gone + 1, alpha, beta);
        min = cmp::min(min, current_value);
        beta = cmp::min(beta, min);

        if alpha >= beta {
          break;
        }
      }
      min
    }    
  }
}