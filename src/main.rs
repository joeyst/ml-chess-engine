#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

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
use crate::map::CROSS_MOVE_MAP;
use crate::mask_for_square::for_rank;
use crate::utility::print_board;

#[macro_use]
extern crate lazy_static;

fn main() {
  print_board(0x5555555555555555);
  let open_squares: u64 = CROSS_MOVE_MAP.lock().unwrap().get_value(1 << 14, 0x5555555555555555);
  print_board(open_squares);
}