use crate::constants::WHOLE_BOARD;

pub fn get_slice_occupation(board: [u64; 13], slice_index: u8) -> u64 {
  board[slice_index as usize]
}

pub fn get_black_occupation(board: [u64; 13]) -> u64 {
  let mut occ_board: u64 = 0;
  for slice_index in 6..12 {
    occ_board |= board[slice_index as usize];
  }
  occ_board
}

pub fn get_white_occupation(board: [u64; 13]) -> u64 {
  let mut occ_board: u64 = 0;
  for slice_index in 0..6 {
    occ_board |= board[slice_index as usize];
  }
  occ_board
}

pub fn get_white_occupation_except_king(board: [u64; 13]) -> u64 {
  let mut occ_board: u64 = 0;
  for slice_index in 0..5 {
    occ_board |= board[slice_index as usize];
  }
  occ_board
}

pub fn get_black_occupation_except_king(board: [u64; 13]) -> u64 {
  let mut occ_board: u64 = 0;
  for slice_index in 6..11 {
    occ_board |= board[slice_index as usize];
  }
  occ_board
}

pub fn get_all_occupation(board: [u64; 13]) -> u64 {
  let mut mut_occ_board: u64 = 0;
  for slice_index in 0..12 {
    mut_occ_board |= board[slice_index as usize];
  }

  let mut mut_occ_board_duplicate: u64 = 0;
  for slice_index in 0..12 {
    mut_occ_board_duplicate ^= board[slice_index as usize];
  }
  assert!(mut_occ_board == mut_occ_board_duplicate);
  mut_occ_board
}

pub fn get_all_not_occupation(board: [u64; 13]) -> u64 {
  get_all_occupation(board) ^ WHOLE_BOARD
}

pub fn get_ally_occupation(board: [u64; 13], turn: u8) -> u64 {
  if turn % 2 == 0 {
    get_black_occupation(board)
  } else {
    get_white_occupation(board)
  }
}

pub fn get_enemy_occupation(board: [u64; 13], turn: u8) -> u64 {
  if turn % 2 == 1 {
    get_black_occupation(board)
  } else {
    get_white_occupation(board)
  }
}

pub fn get_not_ally_occupation(board: [u64; 13], turn: u8) -> u64 {
  if turn % 2 == 0 {
    get_black_occupation(board) ^ WHOLE_BOARD
  } else {
    get_white_occupation(board) ^ WHOLE_BOARD
  }
}

#[cfg(test)]
mod test {
  use super::*;
  mod get_all_occupation_tests {
    use super::*;
    #[test]
    fn gets_whole_occupied_board() {
      let mut board: [u64; 13] = [0; 13];
      board[0] = 0xFF;
      board[1] = 0xFF00;
      board[2] = 0xFFEF0000;
      assert!(get_all_occupation(board) == 0xFFEFFFFF);
    }
  }
}