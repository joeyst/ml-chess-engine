use crate::user::get_legal_input_state;
use crate::r#move::states_for_turn;
use crate::constants::*;
use crate::utility::print_board_pieces;
use crate::bot::{make_bot, Bot, basic_eval, center_squares_worth, random_eval};
use crate::network::train::learn_bot_eval;

lazy_static! {
  pub static ref RANDOM_BOT: Bot = make_bot(random_eval, 0);
  pub static ref LEARN_BOT: Bot = make_bot(learn_bot_eval, 3);
}

pub fn two_console_game() {
  let mut state: [u64; 13] = setup_board();
  let mut turn_number: u8 = 1;
  loop {
    play_player_turn(&mut state, &mut turn_number);
  }
}

pub fn one_bot_game() {
  let mut state: [u64; 13] = setup_board();
  let mut turn_number: u8 = 1;

  let bot: Bot = make_bot(basic_eval, 3);
  loop {
    play_player_turn(&mut state, &mut turn_number);
    play_engine_turn(&bot, &mut state, &mut turn_number);
  }
}

pub fn two_bot_game_learn_bot() {
  let mut state: [u64; 13] = setup_board();
  let mut turn_number: u8 = 1;

  let bot1: Bot = make_bot(learn_bot_eval, 0);
  let bot2: Bot = make_bot(learn_bot_eval, 0);

  loop {
    println!("Gotten here.");
    play_engine_turn(&bot2, &mut state, &mut turn_number);
    play_engine_turn(&bot1, &mut state, &mut turn_number);
  }
}

pub fn two_bot_game() {
  let mut state: [u64; 13] = setup_board();
  let mut turn_number: u8 = 1;

  let bot1: Bot = make_bot(basic_eval, 3);
  let bot2: Bot = make_bot(center_squares_worth, 3);

  loop {
    play_engine_turn(&bot2, &mut state, &mut turn_number);
    play_engine_turn(&bot1, &mut state, &mut turn_number);
  }
}

pub fn setup_board() -> [u64; 13] {
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

pub fn play_engine_turn(engine: &Bot, state: &mut [u64; 13], turn_number: &mut u8) {
  *state = engine.get_state(*state, *turn_number);
  print_board_pieces(*state);
  println!("The engine has played.");
  println!("Move #{}", *turn_number);
  *turn_number += 1;
}


pub fn play_engine_turn_quiet(engine: &Bot, state: &mut [u64; 13], turn_number: &mut u8) {
  *state = engine.get_state_quiet(*state, *turn_number);
  *turn_number += 1;
}