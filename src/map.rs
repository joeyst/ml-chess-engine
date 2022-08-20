use crate::open_squares;
use crate::crop::{crop_board, crop_board_return};
use crate::constants::EVERY_OTHER_VERTICAL_STARTING_FILE_0;

use std::collections::HashMap;
use std::sync::Mutex;
use crate::utility::print_board;
use crate::mask_for_square::{cross_mask_from_bit, diagonal_mask_from_bit, empty_mask_from_bit};

lazy_static! {
  pub static ref CROSS_MOVE_MAP: Mutex<MoveMap> = Mutex::new(make_move_map(crate::open_squares::cross, cross_mask_from_bit));
  pub static ref DIAGONAL_MOVE_MAP: Mutex<MoveMap> = Mutex::new(make_move_map(crate::open_squares::diagonal, diagonal_mask_from_bit));
  pub static ref L_MOVE_MAP: Mutex<MoveMap> = Mutex::new(make_move_map(crate::open_squares::L_shape, empty_mask_from_bit));
  pub static ref SQUARE_MOVE_MAP: Mutex<MoveMap> = Mutex::new(make_move_map(crate::open_squares::square_shape, empty_mask_from_bit));
}

pub struct MoveMap {
  open_squares_function: fn(u64, u64) -> u64,
  mask_of_square_function: fn(u64) -> u64,
  pub map: HashMap<u64, HashMap<u64, u64>>
}

pub fn make_move_map(open_squares_fn: fn(u64, u64) -> u64, mask_of_square_fn: fn(u64) -> u64) -> MoveMap {
  MoveMap {
    open_squares_function: open_squares_fn,
    mask_of_square_function: mask_of_square_fn,
    map: HashMap::<u64, HashMap<u64, u64>>::new()
  }
}

impl MoveMap {
  pub fn get_value(&mut self, square: u64, mut board: u64) -> u64 {
    board &= (self.mask_of_square_function)(square);
    crop_board(&mut board, square);
    if !self.map.contains_key(&square) {
      self.map.insert(square, HashMap::<u64, u64>::new());
    }
    if !self.map.get_mut(&square).expect("").contains_key(&board) {
      self.map.get_mut(&square).expect("").insert(board, (self.open_squares_function)(board, square));
    }
    *self.map.get_mut(&square).expect("").get(&board).expect("")
  }
}

#[cfg(test)]
mod test {
  use super::*;
  mod cross_move_map_tests {
    use super::*;
    #[test]
    fn can_get_and_store_cross_value() {
      let value: u64 = CROSS_MOVE_MAP.lock().unwrap().get_value(1, 0x102);
      assert!(CROSS_MOVE_MAP.lock().unwrap().map.get(&1).expect("").contains_key(&0x102));
      let second_value: u64 = CROSS_MOVE_MAP.lock().unwrap().get_value(1, 0x102);
      assert!(value == 0x102);
      assert!(value == second_value);
    }

    #[test]
    fn crops_outer_edges_when_on_bottom_left_square() {
      let value: u64 = CROSS_MOVE_MAP.lock().unwrap().get_value(1, 0x1010101010101FE);
      assert!(!CROSS_MOVE_MAP.lock().unwrap().map.get(&1).expect("").contains_key(&0x1010101010101FE));
      assert!(CROSS_MOVE_MAP.lock().unwrap().map.get(&1).expect("").contains_key(&0x101010101017E));
    }

    #[test]
    fn crops_outer_edges_when_on_middle_square() {
      let value: u64 = CROSS_MOVE_MAP.lock().unwrap().get_value(1 << 28, EVERY_OTHER_VERTICAL_STARTING_FILE_0);
      assert!(!CROSS_MOVE_MAP.lock().unwrap().map.get(&(1 << 28)).expect("").contains_key(&EVERY_OTHER_VERTICAL_STARTING_FILE_0));
      print_board(crop_board_return(EVERY_OTHER_VERTICAL_STARTING_FILE_0, 1 << 28));
      print_board(crop_board_return(EVERY_OTHER_VERTICAL_STARTING_FILE_0, 1 << 28) & cross_mask_from_bit(1 << 28));
      assert!(CROSS_MOVE_MAP.lock().unwrap().map.get(&(1 << 28)).expect("").contains_key(&0x10101054101000));
    }
  }

  mod diagonal_move_map_tests {
    use super::*;
    #[test]
    fn can_get_and_store_diagonal_value() {
      let value: u64 = DIAGONAL_MOVE_MAP.lock().unwrap().get_value(1, 0x103);
      assert!(DIAGONAL_MOVE_MAP.lock().unwrap().map.get(&1).expect("").contains_key(&0x1));
      let second_value: u64 = DIAGONAL_MOVE_MAP.lock().unwrap().get_value(1, 0x101); 
      assert!(value == 0x8040201008040200);
      assert!(value == second_value);
    }
  }
}