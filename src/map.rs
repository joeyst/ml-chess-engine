use crate::open_squares;

use std::collections::HashMap;

pub struct CrossMoveMap {
  pub map: HashMap<u64, HashMap<u64, u64>>
}

impl CrossMoveMap {
  pub fn get_value(&mut self, square: u64, board: u64) -> u64 {
    if !self.map.contains_key(&square) {
      self.map.insert(square, HashMap::<u64, u64>::new());
    }
    let mut curr_map = &mut self.map.get(&square).expect("");
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
    let mut curr_map = &mut self.map.get(&square).expect("");
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