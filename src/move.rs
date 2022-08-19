use crate::utility::isolate_lsb;

pub fn split_slice_into_slices(mut slice: u64) -> Vec<u64> {
  let mut slices: Vec<u64> = Vec::new();
  while (slice != 0) {
    slices.push(isolate_lsb(slice));
    slice ^= isolate_lsb(slice);
  }
  return slices;
}

pub fn split_board_into_states(slice: u64, board: [u64; 13], slice_index: u8) -> Vec<[u64; 13]> {
  let mut states: Vec<[u64; 13]> = Vec::new();
  let slices: Vec<u64> = split_slice_into_slices(slice);

  states
}

/*
pub fn wstate(board: [u64; 13]) -> Vec<[u64; 13]> {

}
*/

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
}