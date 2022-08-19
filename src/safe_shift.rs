use crate::constants::*;
use crate::utility::print_board;

fn general_shift(mut board: u64, shift: u8, direction: fn(u64) -> u64) -> u64 {
  for _s in 0..shift { board = direction(board); }
  board
}

pub fn horizontal(board: u64, file: i8) -> u64 {
  if file < 0 {
    return left(board, (file * -1).try_into().unwrap());
  } else {
    return right(board, file.try_into().unwrap());
  }
}

pub fn vertical(board: u64, rank: i8) -> u64 {
  if rank < 0 {
    return down(board, (rank * -1).try_into().unwrap());
  } else {
    return up(board, rank.try_into().unwrap());
  }
}

pub fn right(board: u64, file: u8) -> u64 {
  general_shift(board, file, safe_shift_right)
}

pub fn left(board: u64, file: u8) -> u64 {
  general_shift(board, file, safe_shift_left)
}

pub fn up(board: u64, rank: u8) -> u64 {
  general_shift(board, rank, safe_shift_up)
}

pub fn down(board: u64, rank: u8) -> u64 {
  general_shift(board, rank, safe_shift_down)
}

fn safe_shift_right(board: u64) -> u64 {
  (board & ALL_BUT_LAST_FILE) << 1
}

fn safe_shift_left(board: u64) -> u64 {
  (board & ALL_BUT_FIRST_FILE) >> 1
}

fn safe_shift_up(board: u64) -> u64 {
  (board & ALL_BUT_LAST_RANK) << 8
}

fn safe_shift_down(board: u64) -> u64 {
  (board & ALL_BUT_FIRST_RANK) >> 8
}

#[cfg(test)]
mod test {
  use super::*;
  mod right {
    use super::*;
    const BOARD: u64 = 0x1000101010101010;
    #[test]
    fn shifts_board_right_once() {
      assert!(right(BOARD, 1) == 0x2000202020202020);
    }

    #[test]
    fn shifts_board_right_twice() {
      assert!(right(BOARD, 2) == 0x4000404040404040);
    }

    #[test]
    fn shifts_board_off_screen() {
      assert!(right(BOARD, 9) == 0);
    }
  }

  mod left {
    use super::*;
    const BOARD: u64 = 0x1000101010101010;
    #[test]
    fn shifts_board_left_once() {
      assert!(left(BOARD, 1) == 0x800080808080808);
    }

    #[test]
    fn shifts_board_left_twice() {
      assert!(left(BOARD, 2) == 0x400040404040404);
    }

    #[test] 
    fn shifts_board_to_side_of_screen() {
      assert!(left(BOARD, 4) == 0x100010101010101);
    }

    #[test]
    fn shifts_board_left_screen() {
      assert!(left(BOARD, 9) == 0);
    }
  }

  mod up {
    use super::*;
    const BOARD: u64 = 0xF8;
    #[test]
    fn shifts_up_once() {
      assert!(up(BOARD, 1) == 0xF800);
    }

    #[test]
    fn shifts_up_twice() {
      assert!(up(BOARD, 2) == 0xF80000);
    }

    #[test]
    fn shifts_up_to_end_of_board() {
      assert!(up(BOARD, 7) == 0xF800000000000000);
    }

    #[test]
    fn shifts_up_off_the_board() {
      assert!(up(BOARD, 8) == 0);
    }
    
  }

  mod down {
    use super::*;
    const BOARD: u64 = 0xF8000000;
    #[test]
    fn shifts_down_once() {
      assert!(down(BOARD, 1) == 0xF80000);
    }

    #[test]
    fn shifts_up_twice() {
      assert!(down(BOARD, 2) == 0xF800);
    }

    #[test]
    fn shifts_down_to_end_of_board() {
      assert!(down(BOARD, 3) == 0xF8);
    }

    #[test]
    fn shifts_down_off_the_board() {
      assert!(down(BOARD, 8) == 0);
    }
  }
}
