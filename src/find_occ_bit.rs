fn general(board: u64, mut bit: u64, direction: fn(u64) -> u64) -> u64 {
  while bit != 0 {
    bit = direction(bit);
    if bit & board != 0 {
      return bit;
    }
  }
  return 0;
}

pub fn up(board: u64, mut bit: u64) -> u64 {
  general(board, bit, crate::safe_next_bit::next_bit::up)
}

pub fn down(board: u64, mut bit: u64) -> u64 {
  general(board, bit, crate::safe_next_bit::next_bit::down)
}

pub fn right(board: u64, mut bit: u64) -> u64 {
  general(board, bit, crate::safe_next_bit::next_bit::right)
}

pub fn left(board: u64, mut bit: u64) -> u64 {
  general(board, bit, crate::safe_next_bit::next_bit::left)
}

pub fn up_right(board: u64, mut bit: u64) -> u64 {
  general(board, bit, crate::safe_next_bit::next_bit::up_right)
}

pub fn up_left(board: u64, mut bit: u64) -> u64 {
  general(board, bit, crate::safe_next_bit::next_bit::up_left)
}

pub fn down_right(board: u64, mut bit: u64) -> u64 {
  general(board, bit, crate::safe_next_bit::next_bit::down_right)
}

pub fn down_left(board: u64, mut bit: u64) -> u64 {
  general(board, bit, crate::safe_next_bit::next_bit::down_left)
}

#[cfg(test)]
mod test {
  use super::*;
  mod find_occ_bit_up {
    use super::*;
    #[test]
    fn find_one_up() {
      let BOARD: u64 = 0xFF000000000000;
      assert!(up(BOARD, 0x10000000000) == 0x1000000000000);
    }

    #[test]
    fn find_two_up() {
      let BOARD: u64 = 0xFF000000000000;
      assert!(up(BOARD, 0x100000000) == 0x1000000000000);
    }

    #[test]
    fn find_none_up() {
      assert!(up(0, 0x10) == 0);
    }
  }
}