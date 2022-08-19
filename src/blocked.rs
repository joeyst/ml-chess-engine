use crate::utility::*;


// given bit, will find bit one in a specific direction, returning an empty board if crossing a border. 
mod next_bit {
  use super::*;
  fn next(mut bit: u64, shift: i8, border_check: fn(u64) -> bool) -> u64 {
    if border_check(bit) {
      return 0;
    }
    bit <<= shift;
    bit
  }
  pub fn right(bit: u64) -> u64 {
    next(bit, 1, right_bit)
  }

  pub fn left(bit: u64) -> u64 {
    next(bit, -1, left_bit)
  }

  pub fn up(bit: u64) -> u64 {
    next(bit, 8, top_bit)
  }

  pub fn down(bit: u64) -> u64 {
    next(bit, -8, bottom_bit)
  }

  pub fn up_left(bit: u64) -> u64 {
    next(bit, 7, up_left_bit)
  }

  pub fn up_right(bit: u64) -> u64 {
    next(bit, 9, up_right_bit)
  }

  pub fn down_left(bit: u64) -> u64 {
    next(bit, -9, bottom_left_bit)
  }

  pub fn down_right(bit: u64) -> u64 {
    next(bit, -7, bottom_right_bit)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn finds_next_bit_right() {
    let bit: u64 = 0x8;
    assert!(next_bit::right(bit) == 0x10);
  }

  #[test]
  fn finds_bit_up_to_end_of_right() {
    let bit: u64 = 0x40;
    assert!(next_bit::right(bit) == 0x80);
  }

  #[test]
  fn finds_no_bit_to_right() {
    let bit: u64 = 0x80;
    assert!(next_bit::right(bit) == 0);
  }

  #[test]
  fn finds_bit_up_right() {
    let bit: u64 = 0x400000000000;
    assert!(next_bit::up_right(bit) == 0x80000000000000);
  }

  #[test]
  fn gets_empty_board_when_no_up_right_bit() {
    let bit: u64 = 0x4000000000000000;
    assert!(next_bit::up_right(bit) == 0);
  }

}