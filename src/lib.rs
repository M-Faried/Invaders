pub mod display;
pub mod frame;
pub mod invaders;
pub mod keyboard;
pub mod player;
pub mod shot;
pub mod traits;

pub const FRAME_REFRESH_INTERVAL: u64 = 1; //ms
pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;
pub const INVADERS_WIDTH: usize = NUM_COLS - 1;
pub const INVADERS_HIGHT: usize = 9;
pub const INVADERS_MOVE_INIT_INTERVAL: u64 = 2000; //ms
pub const INVADERS_MOVE_INTERVAL_DECREMENT: u128 = 250; //ms
pub const PLAYER_SHOTS_MAX_COUNT: usize = 3;
pub const SHOT_UPDATE_INTERVAL: u64 = 10;
pub const SHOT_EXPLODING_INTERVAL: u64 = 250;
