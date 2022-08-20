use crate::user::get_legal_input_state;
use crate::r#move::states_for_turn;
use crate::constants::*;
use crate::utility::print_board_pieces;

pub fn game() {
  let mut state: [u64; 13] = setup_board();
  let mut turn_number: u8 = 0;
  loop {
    play_player_turn(&mut state, &mut turn_number);
  }
}

fn setup_board() -> [u64; 13] {
  let mut state: [u64; 13] = [0; 13];
  state[WPAWN as usize] = STARTING_WPAWNS;
  state[BPAWN as usize] = STARTING_BPAWNS;
  state[WROOK as usize] = STARTING_WROOKS;
  state[BROOK as usize] = STARTING_BROOKS;
  state[WKNIGHT as usize] = STARTING_WKNIGHTS;
  state[BKNIGHT as usize] = STARTING_BKNIGHTS;
  state[WBISHOP as usize] = STARTING_WBISHOPS;
  state[BBISHOP as usize] = STARTING_BBISHOPS;
  state[WQUEEN as usize] = STARTING_WQUEEN;
  state[BQUEEN as usize] = STARTING_BQUEEN;  
  state[WKING as usize] = STARTING_WKING;  
  state[BKING as usize] = STARTING_BKING;  
  state
}

fn enter_your_move_message(turn_number: u8) {
  if turn_number % 2 == 0 {
    println!("Enter your move, lowercase.");
  } else {
    println!("Enter your move, uppercase.");
  }
}

fn send_message_for_turn(state: [u64; 13], turn_number: u8) {
  print_board_pieces(state);
  enter_your_move_message(turn_number);
}

fn play_player_turn(state: &mut [u64; 13], turn_number: &mut u8) {
  send_message_for_turn(*state, *turn_number);
  let states: Vec<[u64; 13]> = states_for_turn(*state, *turn_number);
  let new_state: [u64; 13] = get_legal_input_state(*state, states);
  *state = new_state;
  *turn_number += 1;
}