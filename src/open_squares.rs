use crate::find_block;
use crate::mask_for_square::for_square;
use crate::utility::{bit_to_index, print_board, find_squares_in_list_on_board};
use crate::utility::square_bounds::{find_squares_within_given_distance, reduce_square_indices_to_slice, find_offsets_on_board_within_distance};

pub fn cross(board: u64, bit: u64) -> u64 {
  let block: u64 = crate::find_block::cross(board, bit);
  // this assumes that there is a bit on the board. If zero, underflow. Leaving unsafe because it would take
  // overhead to make it safe. 
  let cross_mask: u64 = crate::mask_for_square::for_square(crate::utility::bit_to_index(bit) - 1);
  (block & cross_mask) ^ cross_mask ^ bit
}

pub fn diagonal(board: u64, bit: u64) -> u64 {
  let block: u64 = crate::find_block::diagonal(board, bit);
  let diagonal_mask: u64 = crate::mask_for_square::for_square_diagonal(crate::utility::bit_to_index(bit) - 1);
  (block & diagonal_mask) ^ diagonal_mask ^ bit
}

pub fn L_shape(board: u64, square: u64) -> u64 {
  find_offsets_on_board_within_distance(board, square, vec![-15, -17, -10, -6, 6, 10, 15, 17], 2)
}

pub fn square_shape(board: u64, square: u64) -> u64 {
  find_offsets_on_board_within_distance(board, square, vec![9, 8, 7, 1, -1, -7, -8, -9], 1)
}

#[cfg(test)]
mod test {
  use super::*;
  const BOARD: u64 = 0x5555555555555555;

  #[test]
  fn finds_open_on_cross() {
    assert!(cross(BOARD, 1 << 11) == 0x0808080808081408);
  }

  #[test] 
  fn finds_open_on_diagonal() {
    assert!(diagonal(BOARD, 1 << 11) == 0x140014);
  }

  #[test]
  fn finds_L_shape_from_bottom_left() {
    assert!(L_shape(0, 1) == 0x20400);
  }

  #[test]
  fn finds_L_shape_from_bottom_middle() {
    assert!(L_shape(0, 1 << 4) == ((1 << 10) | (1 << 14) | (1 << 19) | (1 << 21)));
  }

  #[test] 
  fn finds_L_shape_from_middle_of_board() {
    assert!(L_shape(0, 1 << 20) == 0x2844004428);
  }

  #[test]
  fn finds_L_shape_from_top_right_of_board() {
    assert!(L_shape(0, 1 << 63) == 0x20400000000000);
  }

  
}