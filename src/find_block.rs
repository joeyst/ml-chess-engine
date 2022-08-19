use crate::find_occ_bit;
use crate::fill_beyond_bit;

pub fn cross(board: u64, bit: u64) -> u64 {
  right(board, bit) | left(board, bit) | up(board, bit) | down(board, bit)
}

pub fn diagonal(board: u64, bit: u64) -> u64 {
  up_left(board, bit) | up_right(board, bit) | down_left(board, bit) | down_right(board, bit)
}

fn general(board: u64, mut bit: u64, find_occ_direction: fn(u64, u64) -> u64, fill_direction: fn(u64) -> u64) -> u64 {
  bit = find_occ_direction(board, bit);
  if bit == 0 {
    return 0;
  }
  fill_direction(bit)
}

pub fn right(board: u64, bit: u64) -> u64 {
  general(board, bit, crate::find_occ_bit::right, fill_beyond_bit::right)
}

pub fn left(board: u64, bit: u64) -> u64 {
  general(board, bit, crate::find_occ_bit::left, fill_beyond_bit::left)
}

pub fn up(board: u64, bit: u64) -> u64 {
  general(board, bit, crate::find_occ_bit::up, fill_beyond_bit::up)
}

pub fn down(board: u64, bit: u64) -> u64 {
  general(board, bit, crate::find_occ_bit::down, fill_beyond_bit::down)
}

pub fn up_left(board: u64, bit: u64) -> u64 {
  general(board, bit, crate::find_occ_bit::up_left, fill_beyond_bit::up_left)
}

pub fn up_right(board: u64, bit: u64) -> u64 {
  general(board, bit, crate::find_occ_bit::up_right, fill_beyond_bit::up_right)
}

pub fn down_left(board: u64, bit: u64) -> u64 {
  general(board, bit, crate::find_occ_bit::down_left, fill_beyond_bit::down_left)
}

pub fn down_right(board: u64, bit: u64) -> u64 {
  general(board, bit, crate::find_occ_bit::down_right, fill_beyond_bit::down_right)
}

#[cfg(test)]
mod test {
  use super::*;
  const BOARD: u64 = 0x5555555555555555;
  #[test]
  fn finds_blocked_right() {
    assert!(right(BOARD, 1 << 11) == 0xE000);
  }

  #[test]
  fn finds_blocked_up_right() {
    assert!(up_right(BOARD, 1 << 11) == 0x804020000000);
  }

  #[test]
  fn finds_blocked_left() {
    assert!(left(BOARD, 1 << 11) == 0x300);
  }

  #[test]
  fn finds_blocked_down() {
    assert!(down(BOARD, 1 << 11) == 0);
  }

  #[test]
  fn finds_blocked_cross() {
    assert!(cross(BOARD | 0xFF000000, 1 << 11) == 0x080808080000E300);
  }

  #[test]
  fn finds_blocked_diagonal() {
    assert!(diagonal(BOARD | 0xFF000000, 1 << 11) == 0x0000804122000000);
  }
  
}