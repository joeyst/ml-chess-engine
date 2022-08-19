

fn general(mut bit: u64, direction: fn(u64) -> u64) -> u64 {
  let mut board: u64 = 0;
  bit = direction(bit);
  while bit != 0 {
    board |= bit;
    bit = direction(bit);
  }
  board
}

pub fn right(mut bit: u64) -> u64 {
  general(bit, crate::blocked::next_bit::right)
}

pub fn left(mut bit: u64) -> u64 {
  general(bit, crate::blocked::next_bit::left)
}

pub fn up(mut bit: u64) -> u64 {
  general(bit, crate::blocked::next_bit::up)
}

pub fn down(mut bit: u64) -> u64 {
  general(bit, crate::blocked::next_bit::down)
}

pub fn up_left(mut bit: u64) -> u64 {
  general(bit, crate::blocked::next_bit::up_left)
}

pub fn up_right(mut bit: u64) -> u64 {
  general(bit, crate::blocked::next_bit::up_right)
}

pub fn down_left(mut bit: u64) -> u64 {
  general(bit, crate::blocked::next_bit::down_left)
}

pub fn down_right(mut bit: u64) -> u64 {
  general(bit, crate::blocked::next_bit::down_right)
}


use crate::utility::print_board;

#[cfg(test)]
mod test {
  use super::*;
  mod fill_beyond_right {
    use super::*;
    #[test]
    fn fills_right_of_first_file() {
      assert!(right(1) == 0xFE);
    }

    #[test]
    fn fills_right_of_last_file() {
      assert!(right(0x80) == 0);
    }

    #[test]
    fn fills_right_in_middle_of_board() {
      assert!(right(0x80000) == 0xF00000);
    }

    #[test]
    fn fills_left_of_zero() {
      assert!(left(1) == 0);
    }

    #[test]
    fn fills_right_of_zero() {
      assert!(right(1) == 0xFE);
    }

    #[test]
    fn fills_up_of_zero() {
      assert!(up(1) == 0x101010101010100);
    }

    #[test]
    fn fills_down_of_zero() {
      assert!(down(1) == 0);
    }

    #[test]
    fn fills_up_left_of_zero() {
      assert!(up_left(1) == 0);
    }

    #[test]
    fn fills_up_right_of_zero() {
      assert!(up_right(1) == 0x8040201008040200);
    }

    #[test]
    fn fills_down_left_of_zero() {
      assert!(down_left(1) == 0);
    }

    #[test] 
    fn fills_down_right_of_zero() {
      assert!(down_right(1) == 0);
    }
  }
}