pub fn print_board(mut board: u64) {
  let mut horizontals: Vec<String> = Vec::new();
  let mut horizontal: String = String::from("");
  
  for rank in 0..8 {
    horizontal += "\n";
    horizontal = format!("{}{}", horizontal, rank.to_string());
    horizontal += "   ";
    for file in 0..8 {
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
  println!("{}", index);
  index % 8 == 0 || index % 8 == 7 || index / 8 == 0 || index / 8 == 7
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
}