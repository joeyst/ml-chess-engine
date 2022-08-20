use crate::find_block;
use crate::mask_for_square::for_square;
use crate::utility::bit_to_index;
use crate::utility::print_board;

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
}