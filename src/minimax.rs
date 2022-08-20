use crate::r#move::{wstate, bstate, states_for_turn};

pub struct Bot {
  eval_fn: fn([u64; 13]) -> u8,
  depth: u8,
  team: u8
}

fn make_bot(eval_function: fn([u64; 13]) -> u8, d: u8, t: u8) -> Bot {
  Bot {
    eval_fn: eval_function,
    depth: d,
    team: t
  }
}

impl Bot {
  pub fn get_state(&self, state: [u64; 13]) -> [u64; 13] {
    *states_for_turn(state, self.team).iter().max_by_key(|s| self.minimax(vec![**s], self.team, self.depth)).unwrap()
  }
  
  fn minimax(&self, states: Vec<[u64; 13]>, turn_number: u8, depth_left: u8) -> u8 {
    if depth_left == 0 {
      self.get_extrema_for_turn(states, turn_number)
    }
    else if turn_number % 2 == 0 {
      states.iter().map(|&s| self.minimax(wstate(s), turn_number + 1, depth_left - 1)).min().unwrap()
    }
    else {
      states.iter().map(|&s| self.minimax(bstate(s), turn_number + 1, depth_left - 1)).max().unwrap()
    }
  }

  fn get_extrema_for_turn(&self, states: Vec<[u64; 13]>, turn: u8) -> u8 {
    if turn % 2 == 0 {
      self.get_lowest(states)
    } else {
      self.get_highest(states)
    }
  }

  #[inline(always)]
  fn get_lowest(&self, states: Vec<[u64; 13]>) -> u8 {
    states.iter().map(|&s| (self.eval_fn)(s)).min().unwrap()
  }

  #[inline(always)]
  fn get_highest(&self, states: Vec<[u64; 13]>) -> u8 {
    states.iter().map(|&s| (self.eval_fn)(s)).max().unwrap()
  }
}