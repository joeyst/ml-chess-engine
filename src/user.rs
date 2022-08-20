use std::io;
use std::collections::HashMap;
use crate::utility::{find_occupied_slice_index, print_board_pieces};
use crate::constants::WHOLE_BOARD;

pub fn get_legal_input_state(current_state: [u64; 13], legal_states: Vec<[u64; 13]>) -> [u64; 13] {
  let mut user_input: String;
  let mut user_state: [u64; 13];
  loop {
    user_input = get_format_matched_user_input();
    user_state = update_board_by_string(current_state, user_input.clone());
    print_board_pieces(user_state);
    if legal_states.iter().any(|&s| s == user_state) {
      return user_state;
    }
    println!("{} is not a legal move, please enter again.", user_input.clone());
  }
}

#[inline(always)]
fn input_wrapper() -> String {
  let mut user_input: String = String::new();
  io::stdin().read_line(&mut user_input).unwrap();
  (*user_input).trim().to_string()
}

lazy_static! {
  static ref FILE_TO_INDEX: HashMap<char, u8> = HashMap::from([
      ('a', 0),
      ('b', 1),
      ('c', 2),
      ('d', 3),
      ('e', 4),
      ('f', 5),
      ('g', 6),
      ('h', 7)
  ]);

  static ref RANK_TO_INDEX: HashMap<char, u8> = HashMap::from([
    ('1', 0),
    ('2', 1),
    ('3', 2),
    ('4', 3),
    ('5', 4),
    ('6', 5),
    ('7', 6),
    ('8', 7),
  ]);
}

fn is_file(ch: char) -> bool {
  FILE_TO_INDEX.contains_key(&ch)
}

fn is_rank(ch: char) -> bool {
  RANK_TO_INDEX.contains_key(&ch)
}

fn evaluate_characters(input: String) -> bool {
  let eval_fns: Vec<fn(char) -> bool> = vec![is_file, is_rank, is_file, is_rank];
  for (ch, ch_fn) in input.chars().zip(eval_fns.iter()) {
    if !(ch_fn)(ch) {
      return false;
    }
  }
  true
}

fn input_matches_format(input: String) -> bool {
  (input.len() == 4) && evaluate_characters(input)
}

fn get_format_matched_user_input() -> String {
  let mut user_input: String;
  loop {
    user_input = input_wrapper();
    println!("user_input: {}", user_input);
    if input_matches_format(user_input.clone()) {
      return user_input.clone();
    }
    println!("{} does not follow coordinate notation, please enter again.", user_input.clone());
  }
}

fn get_start_square(input: String) -> u8 {
  FILE_TO_INDEX.get(&input.chars().nth(0).unwrap()).unwrap() + (8 * RANK_TO_INDEX.get(&input.chars().nth(1).unwrap()).unwrap())
}

fn get_end_square(input: String) -> u8 {
  FILE_TO_INDEX.get(&input.chars().nth(2).unwrap()).unwrap() + (8 * RANK_TO_INDEX.get(&input.chars().nth(3).unwrap()).unwrap())
}

fn update_board_by_indices(mut state: [u64; 13], start: u8, end: u8) -> [u64; 13] {
  let slice_index: u8 = find_occupied_slice_index(state, start);
  state[slice_index as usize] ^= 1 << start;
  let all_but_end: u64 = WHOLE_BOARD ^ (1 << end);
  for slice_number in 0..12 {
    state[slice_number as usize] &= all_but_end;
  }
  state[slice_index as usize] |= 1 << end;
  state
}

fn update_board_by_string(state: [u64; 13], input: String) -> [u64; 13] {
  let start: u8 = get_start_square(input.clone());
  let end: u8 = get_end_square(input);
  update_board_by_indices(state, start, end)
}

#[cfg(test)]
mod test {
  use super::*;
  
  #[test]
  fn gets_square_index_from_string() {
    assert!(get_start_square("a2a3".to_string()) == 8);
  }
}