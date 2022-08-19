pub fn up(board: u64, mut bit: u64) -> u64 {
  while bit != 0 {
    bit = crate::blocked::next_bit::up(bit);
    if bit & board != 0 {
      return bit;
    }
  }
  return 0;
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

    fn find_two_up() {
      let BOARD: u64 = 0xFF000000000000;
      assert!(up(BOARD, 0x100000000) == 0x1000000000000);
    }

    fn find_none_up() {
      assert!(up(0, 0x10) == 0);
    }
  }
}