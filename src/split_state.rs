use crate::utility::isolate_lsb;
use crate::constants::WHOLE_BOARD;
use crate::print_board;

fn split_slice_into_slices(mut slice: u64) -> Vec<u64> {
  let mut slices: Vec<u64> = Vec::new();
  while (slice != 0) {
    slices.push(isolate_lsb(slice));
    slice ^= isolate_lsb(slice);
  }
  return slices;
}

fn remove_start_square(board: &mut [u64; 13], starting_square: u64) {
  let all_but_starting_square: u64 = WHOLE_BOARD ^ starting_square;
  for slice_index in 0..12 {
    board[slice_index] &= all_but_starting_square;
  }
}

fn add_ending_square(board: &mut [u64; 13], ending_square: u64, slice_index: u8) {
  board[slice_index as usize] |= ending_square;
}

fn remove_other_pieces_from_ending_square(board: &mut [u64; 13], ending_square: u64, slice_index: u8) {
  let all_but_ending_square = WHOLE_BOARD ^ ending_square;
  for slice_number in 0..12 {
    if slice_number != slice_index {
      board[slice_number as usize] &= all_but_ending_square;
    }
  }
}

pub fn split_board_into_updated_states(starting_square: u64, slice_of_splits: u64, mut board: [u64; 13], slice_index: u8) -> Vec<[u64; 13]> {
  let mut states: Vec<[u64; 13]> = Vec::new();
  let potential_ending_squares: Vec<u64> = split_slice_into_slices(slice_of_splits);
  let mut current_whole_board: [u64; 13];
  remove_start_square(&mut board, starting_square);

  for ending_square in potential_ending_squares.iter() {
    current_whole_board = board;
    add_ending_square(&mut current_whole_board, *ending_square, slice_index);
    remove_other_pieces_from_ending_square(&mut current_whole_board, *ending_square, slice_index);
    states.push(current_whole_board);
  }
  states
}

#[cfg(test)]
mod test {
  use super::*;
  mod split_slice_into_slices_tests {
    use super::*;
    #[test]
    fn splits_into_zero_slices() {
      let slice: u64 = 0;
      assert!(split_slice_into_slices(slice).len() == 0);
    }

    #[test]
    fn splits_into_one_slice() {
      let slice: u64 = 0x8000;
      assert!(split_slice_into_slices(slice).len() == 1);
    }

    #[test] 
    fn splits_into_multiple_slices() {
      let slice: u64 = 0xFF00;
      assert!(split_slice_into_slices(slice).len() == 8);
    }
  }

  mod split_board_into_updated_states_tests {
    use super::*;
    #[test]
    fn splits_into_no_updated_states() {
      let starting_square: u64 = 0x8;
      let slice_of_splits: u64 = 0;
      let mut board: [u64; 13] = [0; 13];
      let slice_index: u8 = 0;
      let states: Vec<[u64; 13]> = split_board_into_updated_states(starting_square, slice_of_splits, board, slice_index);
      assert!(states.len() == 0);
    }

    #[test]
    fn splits_into_one_updated_state() {
      let starting_square: u64 = 0x8;
      let slice_of_splits: u64 = 1;
      let mut board: [u64; 13] = [0; 13];
      let slice_index: u8 = 0;
      let states: Vec<[u64; 13]> = split_board_into_updated_states(starting_square, slice_of_splits, board, slice_index);
      assert!(states.len() == 1);
    }

    #[test]
    fn splits_into_three_updated_states() {
      let starting_square: u64 = 0x8;
      let slice_of_splits: u64 = 7;
      let mut board: [u64; 13] = [0; 13];
      let slice_index: u8 = 0;
      let states: Vec<[u64; 13]> = split_board_into_updated_states(starting_square, slice_of_splits, board, slice_index);
      assert!(states.len() == 3);
    }
  }
}