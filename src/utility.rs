pub fn print_board(mut board: u64) {
  let mut horizontals: Vec<String> = Vec::new();
  let mut horizontal: String = String::from("");
  
  for rank in 0..8 {
    horizontal += "\n";
    horizontal = format!("{}{}", horizontal, rank.to_string());
    horizontal += "   ";
    for _file in 0..8 {
      horizontal = format!("{}{}", horizontal, (board & 1).to_string());
      horizontal += " ";
      board >>= 1;
    }
    horizontals.push(horizontal);
    horizontal = String::from("");
  }

  for rank in 0..8 {
    println!("{}", horizontals[7 - rank]);
  }
  println!("\n    0 1 2 3 4 5 6 7\n\n\n")
}

pub fn bit_to_index(mut bit: u64) -> u8 {
  let mut index: u8 = 0;
  while bit != 0 {
    bit >>= 1;
    index += 1;
  }
  index
}

pub fn border_bit(bit: u64) -> bool {
  let index: u8 = bit_to_index(bit) - 1;
  index % 8 == 0 || index % 8 == 7 || index / 8 == 0 || index / 8 == 7
}

fn border_bit_general(bit: u64, formula: fn(u8) -> bool) -> bool {
  let index: u8 = bit_to_index(bit) - 1;
  formula(index)
}

fn top_border(index: u8) -> bool {
  index / 8 == 7
}

fn bottom_border(index: u8) -> bool {
  index / 8 == 0
}

fn right_border(index: u8) -> bool {
  index % 8 == 7
}

fn left_border(index: u8) -> bool {
  index % 8 == 0
}

pub fn bottom_bit(bit: u64) -> bool {
  border_bit_general(bit, bottom_border)
}

pub fn top_bit(bit: u64) -> bool {
  border_bit_general(bit, top_border)
}

pub fn right_bit(bit: u64) -> bool {
  border_bit_general(bit, right_border)
}

pub fn left_bit(bit: u64) -> bool {
  border_bit_general(bit, left_border)
}

pub fn up_right_bit(bit: u64) -> bool {
  right_bit(bit) || top_bit(bit)
}

pub fn up_left_bit(bit: u64) -> bool {
  left_bit(bit) || top_bit(bit)
}

pub fn bottom_right_bit(bit: u64) -> bool {
  right_bit(bit) || bottom_bit(bit)
}

pub fn bottom_left_bit(bit: u64) -> bool {
  left_bit(bit) || bottom_bit(bit)
}

pub fn bit_shift_two_way(board: u64, shift: i8) -> u64 {
  if shift < 0 {
    return board >> (shift * -1);
  } 
  board << shift
}

pub fn isolate_lsb(board: u64) -> u64 {
  for i in 0..64 {
    if (board & (1 << i) != 0) {
      return (1 << i);
    }
  }
  0
}

pub fn isolate_msb(board: u64) -> u64 {
  for i in (0..64).rev() {
    if (board & (1 << i) != 0) {
      return (1 << i);
    }
  }
  0
}

#[cfg(test)]
mod test {
  use super::*;
  mod bit_to_index {
    use super::*;
    #[test] 
    fn returns_zero_for_empty_board() {
      assert!(bit_to_index(0) == 0);
    }

    #[test]
    fn can_get_index_of_first_possible_bit() {
      assert!(bit_to_index(1) == 1);
    }

    #[test] 
    fn can_get_index_of_last_possible_bit() {
      assert!(bit_to_index(0x8000000000000000) == 64);
    }

    #[test]
    fn can_get_index_of_middle_bit() {
      assert!(bit_to_index(0x80000000) == 32);
    }
  }

  mod border_bit {
    use super::*;
    #[test]
    fn finds_bit_on_left_of_board() {
      assert!(border_bit(0x10000));
    }

    #[test]
    fn finds_bit_on_right_of_board() {
      assert!(border_bit(0x800000));
    }

    #[test]
    fn finds_bit_on_bottom_of_board() {
      assert!(border_bit(0x10));
    }

    #[test]
    fn finds_bit_on_top_of_boards() {
      assert!(border_bit(0x1000000000000000));
    }

    #[test] 
    fn doesnt_find_bit_on_center_of_board() {
      assert!(border_bit(0x400) == false);
    }
  }

  mod lsb_and_msb_tests {
    use super::*;
    #[test]
    fn finds_no_lsb() {
      assert!(isolate_lsb(0) == 0);
    }

    #[test]
    fn isolates_bottom_bit() {
      assert!(isolate_lsb(1) == 1);
    }

    #[test] 
    fn isolates_bottom_bit_when_there_are_more_bits() {
      assert!(isolate_lsb(0xFF04) == 4);
    }

    #[test]
    fn isolates_bottom_bit_when_its_max_possible_value() {
      print_board(0x8000000000000000);
      assert!(isolate_lsb(0x8000000000000000) == 0x8000000000000000);
    }

    #[test]
    fn finds_no_msb() {
      assert!(isolate_msb(0) == 0);
    }

    #[test]
    fn finds_bottom_msb() {
      assert!(isolate_msb(1) == 1);
    }

    #[test]
    fn finds_top_bit_when_there_are_lower_bits() {
      assert!(isolate_msb(0x8FF00FF) == 0x8000000);
    }

    #[test]
    fn finds_top_bit_when_minimum_possible_value() {
      assert!(isolate_msb(0x1) == 1);
    }
  }
}