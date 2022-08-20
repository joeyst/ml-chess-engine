use crate::constants::*;
use crate::split_state::*;
use crate::board::{get_all_occupation, get_slice_occupation, get_not_ally_occupation};
use crate::map::CROSS_MOVE_MAP;
use crate::print_board;

/*
pub fn wstate(board: [u64; 13]) -> Vec<[u64; 13]> {

}
*/



pub fn wrook(board: [u64; 13]) -> Vec<[u64; 13]> {
  let slice_index: u8 = WROOK;
  let mut possible_states: Vec<[u64; 13]> = Vec::new();
  let occ: u64 = get_all_occupation(board);
  let mut open_squares: u64;

  for rook in split_slice_into_slices(get_slice_occupation(board, slice_index)).iter() {
    open_squares = CROSS_MOVE_MAP.lock().unwrap().get_value(*rook, occ);
    open_squares &= get_not_ally_occupation(board, 1);
    for state in split_board_into_updated_states(*rook, open_squares, board, WROOK).iter() {
      possible_states.push(*state);
    }
  }
  possible_states
}

#[cfg(test)]
mod test {
  use super::*;
  mod wrook_tests {
    use super::*;

    #[test]
    fn finds_moves_for_one_rook() {
      let mut board: [u64; 13] = [0; 13];
      board[0] = 1 << 10;
      board[1] = 0xFFF000;
      let moves: Vec<[u64; 13]> = wrook(board);
      assert!(moves.len() == 4);
    }

    #[test]
    fn finds_that_theres_no_moves_for_one_rook() {
      let mut board: [u64; 13] = [0; 13];
      board[0] = 1 << 10;
      board[1] = 0xFFFBFF;
      let moves: Vec<[u64; 13]> = wrook(board);
      assert!(moves.len() == 0);
    }

    #[test]
    fn finds_moves_for_two_rooks() {
      let mut board: [u64; 13] = [0; 13];
      board[0] = 1 << 10 | 1 << 30;
      board[1] = 0xFFF000;
      let moves: Vec<[u64; 13]> = wrook(board);
      assert!(moves.len() == 15);
    }
  }
}