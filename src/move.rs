use crate::constants::*;
use crate::board::get_all_occupation;
use crate::split_state::*;
use crate::board::get_slice_occupation;
use crate::map::CROSS_MOVE_MAP;

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
    for state in split_board_into_updated_states(*rook, open_squares, board, WROOK).iter() {
      possible_states.push(*state);
    }
  }
  possible_states
}

