#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, non_snake_case))]

mod network;

mod mask_for_square;
mod constants;
mod safe_shift;
mod utility;
mod safe_next_bit;
mod find_occ_bit;
mod fill_beyond_bit;
mod find_block;
mod map;
mod open_squares;
mod r#move;
mod split_state;
mod board;
mod crop;
mod pawn_move;
mod user;
mod game;
mod bot;

extern crate rand;

use crate::map::CROSS_MOVE_MAP;
use crate::mask_for_square::for_rank;
use crate::utility::{print_board, print_board_pieces};
use std::io;
use crate::game::{two_console_game, one_bot_game, two_bot_game};

#[macro_use]
extern crate lazy_static;

fn main() {
  two_bot_game();
}