use crate::utility::{bottom_bit, top_bit, right_bit, left_bit};
use crate::constants::{ALL_BUT_FIRST_RANK, ALL_BUT_LAST_RANK, ALL_BUT_FIRST_FILE, ALL_BUT_LAST_FILE};

pub fn crop_board(board: &mut u64, square: u64) {
  if !bottom_bit(square) {
    *board &= ALL_BUT_FIRST_RANK;
  }
  if !top_bit(square) {
    *board &= ALL_BUT_LAST_RANK;
  }
  if !right_bit(square) {
    *board &= ALL_BUT_LAST_FILE;
  }
  if !left_bit(square) {
    *board &= ALL_BUT_FIRST_FILE;
  }
}

pub fn crop_board_return(mut board: u64, square: u64) -> u64 {
  if !bottom_bit(square) {
    board &= ALL_BUT_FIRST_RANK;
  }
  if !top_bit(square) {
    board &= ALL_BUT_LAST_RANK;
  }
  if !right_bit(square) {
    board &= ALL_BUT_LAST_FILE;
  }
  if !left_bit(square) {
    board &= ALL_BUT_FIRST_FILE;
  }
  board
}