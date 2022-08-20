use crate::constants::FIRST_FILE;
use crate::constants::FIRST_RANK;
use crate::constants::FORWARD_DIAGONAL;
use crate::constants::BACKWARD_DIAGONAL;
use crate::safe_shift;

pub fn cross_mask_from_bit(bit: u64) -> u64 {
  if bit == 0 {
    return 0;
  }
  let square: u8 = crate::utility::bit_to_index(bit) - 1;
  for_square(square)
}

pub fn diagonal_mask_from_bit(bit: u64) -> u64 {
  if bit == 0 {
    return 0;
  }
  let square: u8 = crate::utility::bit_to_index(bit) - 1;
  for_square_diagonal(square)
}

pub fn for_file(file: u8) -> u64 {
  FIRST_FILE << file
}

pub fn for_rank(rank: u8) -> u64 {
  FIRST_RANK << (8 * rank)
}

pub fn for_square(square: u8) -> u64 {
  for_file(square % 8) | for_rank(square / 8)
}

pub fn for_forward_diagonal(square: u8) -> u64 {
  let rank: i8 = (square / 8).try_into().unwrap();
  let file: i8 = (square % 8).try_into().unwrap();
  let shift: i8 = file - rank;
  crate::safe_shift::horizontal(FORWARD_DIAGONAL, shift)
}

pub fn for_backward_diagonal(square: u8) -> u64 {
  let rank: i8 = (square / 8).try_into().unwrap();
  let file: i8 = (square % 8).try_into().unwrap();
  let shift: i8 = (rank + file) - 7;
  crate::safe_shift::horizontal(BACKWARD_DIAGONAL, shift)
}


pub fn for_square_diagonal(square: u8) -> u64 {
  for_forward_diagonal(square) | for_backward_diagonal(square)
}

#[cfg(test)]
mod test {
  use super::*;
  mod for_file {
    use super::*;
    #[test]
    fn calculates_last_file() {
      const FILE: u8 = 7;
      assert!(for_file(FILE) == 0x8080808080808080);
    }

    #[test]
    fn calculates_second_to_last_file() {
      const FILE: u8 = 6;
      assert!(for_file(FILE) == 0x4040404040404040);
    }

    #[test]
    fn calculates_first_file() {
      const FILE: u8 = 0;
      assert!(for_file(FILE) == 0x101010101010101);
    }
  }

  mod for_rank {
    use super::*;
    #[test]
    fn calculates_last_rank() {
      const RANK: u8 = 7;
      assert!(for_rank(RANK) == 0xFF00000000000000);
    }

    #[test]
    fn calculates_second_to_last_rank() {
      const RANK: u8 = 6;
      assert!(for_rank(RANK) == 0xFF000000000000);
    }

    #[test]
    fn calculates_first_rank() {
      const RANK: u8 = 0;
      assert!(for_rank(RANK) == 0xFF);
    }
  }

  mod for_square {
    use super::*;

    #[test]
    fn calculates_last_square() {
      const SQUARE: u8 = 63;
      assert!(for_square(SQUARE) == 0xFF80808080808080);
    }

    #[test]
    fn calculates_first_square() {
      const SQUARE: u8 = 0;
      assert!(for_square(SQUARE) == 0x1010101010101FF);
    }

    #[test]
    fn calculates_square_43() {
      const SQUARE: u8 = 43;
      assert!(for_square(SQUARE) == 0x0808FF0808080808);
    }
  }
}
