mod mask_for_square;
mod constants;
mod safe_shift;
mod utility;
use crate::mask_for_square::for_rank;
use crate::utility::print_board;


fn main() {
    println!("Hello, world!");
    print_board(0x1000101010101010);
}