// default config
pub const DEFAULT_CONFIG: &'static str = r#"
[config]
show_highscore_at_start = false
magnetization = true
blend = true
alpha = 150
fps = 60
[score]
users =
scores =
times ="#;
pub const GAME_OVER: &'static str = "GAME OVER";
// resource & config
pub const FONT_FILE: &'static str = "./resources/FiraMono-Regular.ttf";
pub const CONFIG_FILE: &'static str = "./resources/config.ini";
pub const GAMESCORE_COUNT: usize = 5;
// game title
pub const GT: &'static str = "1010";
pub const MILLISECOND: u32 = 1000;
// game score multiplier
pub const LINE_MULTIPLIER: u32 = 10;
pub const BASKET_COUNT: u8 = 3;
pub const BASKET_SIZE: u8 = 5;
pub const FIELD_SIZE: u8 = 10;
pub const FIELD_SHIFT: i16 = 10;
// field tile size & separator
pub const TILE_SIZE_1: u8 = 32;
pub const TILE_SEP_1: u8 = 3;
// basket tile size & separator
pub const TILE_SIZE_2: u8 = TILE_SIZE_1 / 2;
pub const TILE_SEP_2: u8 = 2;
// game block round rect
pub const ROUND_RADIUS: i16 = 4;
// gameover round rect radius
pub const BIG_ROUND_RADIUS: i16 = 8;
pub const FIELD_WIDTH: u32 = (TILE_SIZE_1 as u32 + TILE_SEP_1 as u32) * FIELD_SIZE as u32 + 2 * FIELD_SHIFT as u32;
pub const BASKET_LEN: u32 = (TILE_SIZE_2 as u32 + TILE_SEP_2 as u32) * BASKET_SIZE as u32 + FIELD_SHIFT as u32;
// game window size
pub const W_WIDTH: u32 = FIELD_WIDTH + BASKET_LEN;
pub const W_HEIGHT: u32 = FIELD_WIDTH;
// font consts
pub const FONT_MIN_SIZE: u16 = 12;
pub const FONT_DEF_SIZE: u16 = 18;
pub const FONT_BIG_SIZE: u16 = 48;
pub const FONT_HEIGHT: i16 = FONT_DEF_SIZE as i16 + 2;
