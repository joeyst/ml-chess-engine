use crate::open_squares;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  pub static ref CROSS_MOVE_MAP: Mutex<MoveMap> = Mutex::new(make_move_map(crate::open_squares::cross));
  pub static ref DIAGONAL_MOVE_MAP: Mutex<DiagonalMoveMap> = Mutex::new(make_diagonal_move_map());
}

pub struct MoveMap {
  open_squares_function: fn(u64, u64) -> u64,
  pub map: HashMap<u64, HashMap<u64, u64>>
}

pub fn make_move_map(open_squares_fn: fn(u64, u64) -> u64) -> MoveMap {
  MoveMap {
    open_squares_function: open_squares_fn,
    map: HashMap::<u64, HashMap<u64, u64>>::new()
  }
}

impl MoveMap {
  pub fn get_value(&mut self, square: u64, board: u64) -> u64 {
    if !self.map.contains_key(&square) {
      self.map.insert(square, HashMap::<u64, u64>::new());
    }
    if !self.map.get_mut(&square).expect("").contains_key(&board) {
      self.map.get_mut(&square).expect("").insert(board, (self.open_squares_function)(board, square));
    }
    *self.map.get_mut(&square).expect("").get(&board).expect("")
  }
}

pub struct CrossMoveMap {
  pub map: HashMap<u64, HashMap<u64, u64>>
}

impl CrossMoveMap {
  pub fn get_value(&mut self, square: u64, board: u64) -> u64 {
    if !self.map.contains_key(&square) {
      self.map.insert(square, HashMap::<u64, u64>::new());
    }
    if !self.map.get_mut(&square).expect("").contains_key(&board) {
      self.map.get_mut(&square).expect("").insert(board, crate::open_squares::cross(board, square));
    }
    *self.map.get_mut(&square).expect("").get(&board).expect("")
  }
}

pub fn make_cross_move_map() -> CrossMoveMap {
  CrossMoveMap {
    map: HashMap::<u64, HashMap<u64, u64>>::new()
  }
}

pub struct DiagonalMoveMap {
  pub map: HashMap<u64, HashMap<u64, u64>>
}

impl DiagonalMoveMap {
  pub fn get_value(&mut self, square: u64, board: u64) -> u64 {
    if !self.map.contains_key(&square) {
      self.map.insert(square, HashMap::<u64, u64>::new());
    }
    if !self.map.get_mut(&square).expect("").contains_key(&board) {
      self.map.get_mut(&square).expect("").insert(board, crate::open_squares::diagonal(board, square));
    }
    *self.map.get_mut(&square).expect("").get(&board).expect("")
  }
}

pub fn make_diagonal_move_map() -> DiagonalMoveMap {
  DiagonalMoveMap {
    map: HashMap::<u64, HashMap<u64, u64>>::new()
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
  }

  mod diagonal_move_map_tests {
    use super::*;
    #[test]
    fn can_get_and_store_diagonal_value() {
      let value: u64 = DIAGONAL_MOVE_MAP.lock().unwrap().get_value(1, 0x102);
      assert!(CROSS_MOVE_MAP.lock().unwrap().map.get(&1).expect("").contains_key(&0x102));
      let second_value: u64 = DIAGONAL_MOVE_MAP.lock().unwrap().get_value(1, 0x102);
      assert!(value == 0x8040201008040200);
      assert!(value == second_value);
    }
  }
}