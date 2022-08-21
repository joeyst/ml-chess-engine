pub static FIRST_FILE: u64 = 0x101010101010101;
pub static FIRST_RANK: u64 = 0xFF;
pub static FORWARD_DIAGONAL: u64 = 0x8040201008040201;
pub static BACKWARD_DIAGONAL: u64 = 0x102040810204080;

pub static ALL_BUT_LAST_FILE: u64 = 0x7F7F7F7F7F7F7F7F;
pub static ALL_BUT_FIRST_FILE: u64 = 0xFEFEFEFEFEFEFEFE;
pub static ALL_BUT_LAST_RANK: u64 = 0xFFFFFFFFFFFFFF;
pub static ALL_BUT_FIRST_RANK: u64 = 0xFFFFFFFFFFFFFF00;

pub static FOURTH_RANK: u64 = 0xFF000000;
pub static FIFTH_RANK: u64 = 0xFF00000000;

pub static WHOLE_BOARD: u64 = 0xFFFFFFFFFFFFFFFF;
pub static EVERY_OTHER_VERTICAL_STARTING_FILE_0: u64 = 0x5555555555555555;

pub static WROOK: u8 = 0;
pub static WBISHOP: u8 = 1;
pub static WQUEEN: u8 = 2;
pub static BROOK: u8 = 6;
pub static BBISHOP: u8 = 7;
pub static BQUEEN: u8 = 8;

pub static WPAWN: u8 = 3;
pub static WKNIGHT: u8 = 4;
pub static WKING: u8 = 5;
pub static BPAWN: u8 = 9;
pub static BKNIGHT: u8 = 10;
pub static BKING: u8 = 11;

pub static BLACK_TEAM: u8 = 0;
pub static WHITE_TEAM: u8 = 1;

pub const STARTING_WPAWNS: u64 = 0xFF00;
pub const STARTING_BPAWNS: u64 = 0xFF000000000000;
pub const STARTING_WROOKS: u64 = 0x81;
pub const STARTING_BROOKS: u64 = 0x8100000000000000;
pub const STARTING_WKNIGHTS: u64 = 0x42;
pub const STARTING_BKNIGHTS: u64 = 0x4200000000000000;
pub const STARTING_WBISHOPS: u64 = 0x24;
pub const STARTING_BBISHOPS: u64 = 0x2400000000000000;
pub const STARTING_WQUEEN: u64 = 0x8;
pub const STARTING_BQUEEN: u64 = 0x800000000000000;
pub const STARTING_WKING: u64 = 0x10;
pub const STARTING_BKING: u64 = 0x1000000000000000;

pub const CENTER_FOUR_SQUARES: u64 = 0x1818000000;
pub const SECOND_CENTER_SQUARES: u64 = 0x3C24243C0000;