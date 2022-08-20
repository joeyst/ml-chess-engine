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

pub fn L_shape(square: u64) -> u64 {
  let square_index: u8 = bit_to_index(square) - 1;

  let temp_signed: i8 = square_index as i8;
  let offsets: Vec<i8> = vec![-15, -17, -10, -6, 6, 10, 15, 17];
  let mut squares: Vec<u8> = Vec::new();
  for offset in offsets.iter() {
    if (temp_signed + offset >= 0) && (temp_signed + offset <= 63) {
      squares.push((temp_signed + offset) as u8);
    }
  }

  let file: i16 = (square_index % 8) as i16;
  let rank: i16 = (square_index / 8) as i16;

  let mut temp_file: i16;
  let mut temp_rank: i16;

  let mut dif_in_file: i16;
  let mut dif_in_rank: i16;

  let mut new_squares: Vec<u8> = Vec::new();
  for s in squares.iter() {
    temp_file = (s % 8) as i16;
    temp_rank = (s / 8) as i16;

    dif_in_file = temp_file - file;
    dif_in_rank = temp_rank - rank;

    if dif_in_file < 0 {
      dif_in_file *= -1;
    }
    if dif_in_rank < 0 {
      dif_in_rank *= 1;
    }

    if (dif_in_file <= 2) && (dif_in_rank <= 2) {
      new_squares.push(*s);
    }
  }

  let mut new_board: u64 = 0;
  let mut temp_board: u64;
  for s in new_squares.iter() {
    temp_board = 1;
    new_board |= temp_board << s;
  }
  new_board
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
    assert!(L_shape(1) == 0x20400);
  }

  #[test]
  fn finds_L_shape_from_bottom_middle() {
    assert!(L_shape(1 << 4) == ((1 << 10) | (1 << 14) | (1 << 19) | (1 << 21)));
  }

  #[test] 
  fn finds_L_shape_from_middle_of_board() {
    assert!(L_shape(1 << 20) == 0x2844004428);
  }

  #[test]
  fn finds_L_shape_from_top_right_of_board() {
    assert!(L_shape(1 << 63) == 0x20400000000000);
  }
}