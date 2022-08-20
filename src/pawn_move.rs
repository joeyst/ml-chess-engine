use crate::constants::*;
use crate::utility::{two_way_shift};
use crate::board::*;
use crate::split_state::split_slice_into_slices;

fn remove_square_from_board_by_index_away(mut board: [u64; 13], bit: u64, shift: i8) -> [u64; 13] {
  let starting_square: u64 = two_way_shift(bit, -shift);
  let all_but_starting_square = starting_square ^ WHOLE_BOARD;
  for slice_index in 0..12 {
    board[slice_index] &= all_but_starting_square;
  }
  board
}

fn get_updated_board_from_endings(board: [u64; 13], pawns: u64, shift: i8, slice_index: u8) -> Vec<[u64; 13]> {
  let mut states: Vec<[u64; 13]> = Vec::new();
  let mut temp_board: [u64; 13];
  let mut all_but_ending_square;

  for new_pawn_bit in split_slice_into_slices(pawns).iter() {
    all_but_ending_square = new_pawn_bit ^ WHOLE_BOARD;
    temp_board = remove_square_from_board_by_index_away(board, *new_pawn_bit, shift);
    temp_board[slice_index as usize] |= *new_pawn_bit;
    for slice_number in 0..12 {
      if slice_number != slice_index {
        temp_board[slice_number as usize] &= all_but_ending_square;
      }
    }

    states.push(temp_board);
  }
  states
}

#[inline(always)]
pub fn general_pawn_all(board: [u64; 13], team: u8, slice_index: u8, end_slice_fn: Vec<fn(u64, u64) -> u64>, shifts: Vec<i8>) -> Vec<[u64; 13]> {
  let mut states: Vec<[u64; 13]> = Vec::new();
  let not_board_occ: u64 = get_all_not_occupation(board);
  let enemy_occ: u64 = get_enemy_occupation(board, team);
  let pawns: u64 = board[slice_index as usize];
  let all_slices_of_moves: Vec<u64> = vec![end_slice_fn[0](pawns, not_board_occ), end_slice_fn[1](pawns, not_board_occ), end_slice_fn[2](pawns, enemy_occ), end_slice_fn[3](pawns, enemy_occ)];

  for single_slice_of_moves in all_slices_of_moves.iter().zip(shifts.iter()) {
    for ending in get_updated_board_from_endings(board, *single_slice_of_moves.0, *single_slice_of_moves.1, slice_index).iter() {
      states.push(*ending);
    }
  }

  states
}

#[inline(always)]
pub fn wpawn_all(board: [u64; 13]) -> Vec<[u64; 13]> {
  general_pawn_all(board, WHITE_TEAM, WPAWN, vec![wpawn_one, wpawn_two, wpawn_right, wpawn_left], vec![8, 16, 9, 7])
}

#[inline(always)]
pub fn bpawn_all(board: [u64; 13]) -> Vec<[u64; 13]> {
  general_pawn_all(board, BLACK_TEAM, BPAWN, vec![bpawn_one, bpawn_two, bpawn_right, bpawn_left], vec![-8, -16, -7, -9])
}


fn general_pawn_move(mut pawns: u64, shift: i8, occupation: u64, mask: u64) -> u64 {
  pawns &= mask;
  pawns = two_way_shift(pawns, shift);
  pawns & occupation
}

pub fn wpawn_one(pawns: u64, occupation: u64) -> u64 {
  general_pawn_move(pawns, 8, occupation, ALL_BUT_LAST_RANK)
}

pub fn wpawn_right(pawns: u64, occupation: u64) -> u64 {
  general_pawn_move(pawns, 9, occupation, ALL_BUT_LAST_RANK & ALL_BUT_LAST_FILE)
}

pub fn wpawn_left(pawns: u64, occupation: u64) -> u64 {
  general_pawn_move(pawns, 7, occupation, ALL_BUT_LAST_RANK & ALL_BUT_FIRST_FILE)
}

pub fn wpawn_two(mut pawns: u64, occupation: u64) -> u64 {
  pawns = wpawn_one(pawns, occupation);
  wpawn_one(pawns, occupation) & FOURTH_RANK
}

pub fn bpawn_one(pawns: u64, occupation: u64) -> u64 {
  general_pawn_move(pawns, -8, occupation, ALL_BUT_FIRST_RANK)
}

pub fn bpawn_right(pawns: u64, occupation: u64) -> u64 {
  general_pawn_move(pawns, -7, occupation, ALL_BUT_FIRST_RANK & ALL_BUT_LAST_FILE)
}

pub fn bpawn_left(pawns: u64, occupation: u64) -> u64 {
  general_pawn_move(pawns, -9, occupation, ALL_BUT_FIRST_RANK & ALL_BUT_FIRST_FILE)
}

pub fn bpawn_two(mut pawns: u64, occupation: u64) -> u64 {
  pawns = bpawn_one(pawns, occupation);
  bpawn_one(pawns, occupation) & FIFTH_RANK
}