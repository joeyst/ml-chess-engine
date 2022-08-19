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
use crate::mask_for_square::for_rank;
use crate::utility::print_board;

fn main() {
  let mut hmap = crate::map::make_cross_move_map();
  print_board(0x5555555555555555);
  let mut open_squares: u64 = hmap.get_value(1 << 14, 0x5555555555555555);
  print_board(open_squares);
}