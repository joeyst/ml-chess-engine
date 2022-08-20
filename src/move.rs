use crate::constants::*;
use crate::split_state::*;
use crate::board::{get_all_occupation, get_slice_occupation, get_not_ally_occupation};
use crate::map::{CROSS_MOVE_MAP, DIAGONAL_MOVE_MAP, L_MOVE_MAP};
use crate::print_board;
use crate::map::MoveMap;
use std::sync::Mutex;

/*
pub fn wstate(board: [u64; 13]) -> Vec<[u64; 13]> {

}
*/

pub fn sliding_move_general(board: [u64; 13], slice_index: u8, map: &Mutex<MoveMap>, team: u8) -> Vec<[u64; 13]> {
  let mut possible_states: Vec<[u64; 13]> = Vec::new();
  let occ: u64 = get_all_occupation(board);
  let mut open_squares: u64;

  for piece in split_slice_into_slices(get_slice_occupation(board, slice_index)).iter() {
    open_squares = map.lock().unwrap().get_value(*piece, occ);
    open_squares &= get_not_ally_occupation(board, team);
    for state in split_board_into_updated_states(*piece, open_squares, board, slice_index).iter() {
      possible_states.push(*state);
    }
  }
  possible_states
}

pub fn wrook(board: [u64; 13]) -> Vec<[u64; 13]> {
  sliding_move_general(board, WROOK, &CROSS_MOVE_MAP, WHITE_TEAM)
}

pub fn brook(board: [u64; 13]) -> Vec<[u64; 13]> {
  sliding_move_general(board, BROOK, &CROSS_MOVE_MAP, BLACK_TEAM)
}

pub fn wbishop(board: [u64; 13]) -> Vec<[u64; 13]> {
  sliding_move_general(board, WBISHOP, &DIAGONAL_MOVE_MAP, WHITE_TEAM)
}

pub fn bbishop(board: [u64; 13]) -> Vec<[u64; 13]> {
  sliding_move_general(board, BBISHOP, &DIAGONAL_MOVE_MAP, BLACK_TEAM)
}

pub fn wqueen(board: [u64; 13]) -> Vec<[u64; 13]> {
  let mut states: Vec<[u64; 13]> = Vec::new();
  for state in sliding_move_general(board, WQUEEN, &CROSS_MOVE_MAP, WHITE_TEAM) {states.push(state)}
  for state in sliding_move_general(board, WQUEEN, &DIAGONAL_MOVE_MAP, WHITE_TEAM) {states.push(state)}
  states
}

pub fn bqueen(board: [u64; 13]) -> Vec<[u64; 13]> {
  let mut states: Vec<[u64; 13]> = Vec::new();
  for state in sliding_move_general(board, BQUEEN, &CROSS_MOVE_MAP, BLACK_TEAM) {states.push(state)}
  for state in sliding_move_general(board, BQUEEN, &DIAGONAL_MOVE_MAP, BLACK_TEAM) {states.push(state)}
  states
}

pub fn wknight(board: [u64; 13]) -> Vec<[u64; 13]> {
  sliding_move_general(board, WKNIGHT, &L_MOVE_MAP, WHITE_TEAM)
}

pub fn bknight(board: [u64; 13]) -> Vec<[u64; 13]> {
  sliding_move_general(board, BKNIGHT, &L_MOVE_MAP, BLACK_TEAM)
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