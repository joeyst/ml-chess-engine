use crate::open_squares;

use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
  pub static ref CROSS_MOVE_MAP: Mutex<CrossMoveMap> = Mutex::new(make_cross_move_map());
  pub static ref DIAGONAL_MOVE_MAP: Mutex<DiagonalMoveMap> = Mutex::new(make_diagonal_move_map());
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
    fn can_get_cross_value() {
      let value: u64 = CROSS_MOVE_MAP.lock().unwrap().get_value(1, 0x102);
      assert!(CROSS_MOVE_MAP.lock().unwrap().map.get(&1).expect("").contains_key(&0x102));
      assert!(value == 0x102);
    }
  }
}