use crate::constants::*;
use crate::split_state::*;
use crate::board::{get_all_occupation, get_slice_occupation, get_not_ally_occupation};
use crate::map::{CROSS_MOVE_MAP, DIAGONAL_MOVE_MAP, L_MOVE_MAP, SQUARE_MOVE_MAP};
use crate::{print_board, print_board_pieces};
use crate::map::MoveMap;
use std::sync::Mutex;
use crate::pawn_move::{wpawn_all, bpawn_all};

#[inline(always)]
fn gstate(board: [u64; 13], state_functions: Vec<fn([u64; 13]) -> Vec<[u64; 13]>>) -> Vec<[u64; 13]> {
  let mut states: Vec<[u64; 13]> = Vec::new();

  for state_function in state_functions.iter() {
    for state in (state_function)(board).iter() {
      states.push(*state);
    }
  }

  states
}

pub fn wstate(board: [u64; 13]) -> Vec<[u64; 13]> {
  gstate(board, vec![wrook, wbishop, wqueen, wpawn, wking, wknight])
}

pub fn bstate(board: [u64; 13]) -> Vec<[u64; 13]> {
  gstate(board, vec![brook, bbishop, bqueen, bpawn, bking, bknight])
}

pub fn states_for_turn(board: [u64; 13], turn: u8) -> Vec<[u64; 13]> {
  if turn % 2 == 0 {
    bstate(board)
  } else {
    wstate(board)
  }
}

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

pub fn wking(board: [u64; 13]) -> Vec<[u64; 13]> {
  sliding_move_general(board, WKING, &SQUARE_MOVE_MAP, WHITE_TEAM)
}

pub fn bking(board: [u64; 13]) -> Vec<[u64; 13]> {
  sliding_move_general(board, BKING, &SQUARE_MOVE_MAP, BLACK_TEAM)
}

pub fn wpawn(board: [u64; 13]) -> Vec<[u64; 13]> {
  wpawn_all(board)
}

pub fn bpawn(board: [u64; 13]) -> Vec<[u64; 13]> {
  bpawn_all(board)
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

  mod pawn_tests {
    use super::*;

    #[test]
    fn finds_pawn_move_one_and_two_up_and_left_and_right() {
      let mut board: [u64; 13] = [0; 13];
      board[WPAWN as usize] = 1 << 11;
      board[BPAWN as usize] = 1 << 18;
      board[BPAWN as usize] |= 1 << 20;
      let states: Vec<[u64; 13]> = wpawn(board);
      assert!(states.len() == 4);
    }

    #[test] 
    fn finds_pawn_move_one_up_and_left_and_right() {
      let mut board: [u64; 13] = [0; 13];
      board[WPAWN as usize] = 1 << 19;
      board[BPAWN as usize] = 1 << 26;
      board[BPAWN as usize] |= 1 << 28;
      let states: Vec<[u64; 13]> = wpawn(board);
      assert!(states.len() == 3);
    }

    #[test]
    fn finds_pawn_move_left_and_right_and_not_up_because_blocked_by_same_team() {
      let mut board: [u64; 13] = [0; 13];
      board[WPAWN as usize] = 1 << 11;
      board[WKNIGHT as usize] = 1 << 19;
      board[BPAWN as usize] = 1 << 18;
      board[BPAWN as usize] |= 1 << 20;
      let states: Vec<[u64; 13]> = wpawn(board);
      assert!(states.len() == 2);
    }

    #[test]
    fn finds_pawn_move_left_and_right_and_not_up_because_blocked_by_different_team() {
      let mut board: [u64; 13] = [0; 13];
      board[WPAWN as usize] = 1 << 11;
      board[BKNIGHT as usize] = 1 << 19;
      board[BPAWN as usize] = 1 << 18;
      board[BPAWN as usize] |= 1 << 20;
      let states: Vec<[u64; 13]> = wpawn(board);
      assert!(states.len() == 2);
    }

    #[test] 
    fn finds_pawn_move_up_only_one_and_not_left_or_right() {
      let mut board: [u64; 13] = [0; 13];
      board[WPAWN as usize] = 1 << 19;
      board[WKNIGHT as usize] = 1 << 26;
      board[WKNIGHT as usize] |= 1 << 28;
      let states: Vec<[u64; 13]> = wpawn(board);
      assert!(states.len() == 1);
    }

    #[test]
    fn finds_pawn_move_up_only_one_and_not_left_or_right_because_left_and_right_are_empty() {
      let mut board: [u64; 13] = [0; 13];
      board[WPAWN as usize] = 1 << 19;
      let states: Vec<[u64; 13]> = wpawn(board);
      assert!(states.len() == 1);
    }

    #[test]
    fn finds_multiple_pawns_can_move_forward() {
      let mut board: [u64; 13] = [0; 13];
      board[WPAWN as usize] = 1 << 19;
      board[WPAWN as usize] |= 1 << 20;
      let states: Vec<[u64; 13]> = wpawn(board);
      assert!(states.len() == 2);
    }
  }

  mod all_piece_tests {
    use super::*;

    #[test]
    fn finds_rook_and_pawn_and_knight_moves() {
      let mut board: [u64; 13] = [0; 13];
      board[WPAWN as usize] = 1 << 19;
      board[WPAWN as usize] |= 1 << 20;
      board[WKNIGHT as usize] = 1;
      board[WROOK as usize] = 1 << 9;
      board[WBISHOP as usize] = 1 << 22;
      let states: Vec<[u64; 13]> = wstate(board);
      assert!(states.len() == 27);
    }
  }
}