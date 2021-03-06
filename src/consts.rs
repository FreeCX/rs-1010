// default config
pub const DEFAULT_CONFIG: &'static str = r#"
[score]
users =
scores =
times =

[game]
show_highscore_at_start = false
magnetization = false
blend = true
alpha = 150
fps = 60
username = user
ask_username = true

[color]
game_background = 100, 100, 100
field_background = 170, 170, 170
font = 200, 200, 200
light = 255, 255, 255
border = 210, 210, 210
fig1 = 230, 100, 100
fig2 = 230, 210, 100
fig3 = 100, 230, 100
fig4 = 230, 100, 200
fig5 = 100, 230, 200
fig6 = 100, 200, 230
fig7 = 100, 100, 230
fig8 = 210, 100, 230
"#;

// game strings
pub const GAME_OVER_TEXT: &'static str = "your name:";
pub const GAME_OVER: &'static str = "GAME OVER";
pub const GT: &'static str = "1010";

// errors
pub const INIT_SDL_ERROR: &'static str = "Cannot init sdl2 context";
pub const INIT_SDL_SUBSYSTEM_ERROR: &'static str = "Cannot create video subsystem";
pub const INIT_WINDOW_ERROR: &'static str = "Cannot create window";
pub const GET_CANVAS_ERROR: &'static str = "Cannot get canvas";
pub const GET_COLOR_ERROR: &'static str = "Cannot get color";

// resource & config
pub const FONT_FILE: &'static str = "./resources/FiraMono-Regular.ttf";
pub const CONFIG_FILE: &'static str = "./resources/config.ini";
pub const GAMESCORE_COUNT: usize = 5;

// game fps param
pub const MILLISECOND: u32 = 1000;

// game score multiplier
pub const LINE_MULTIPLIER: u32 = 30;
pub const BLOCK_COST_MULTIPLIER: u32 = 5;

// basket params
pub const BASKET_COUNT: u8 = 3;
pub const BASKET_SIZE: u8 = 5;

// filed params
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

// defaul game params
pub const DEFAULT_HIGHSCORE_AT_START: bool = false;
pub const DEFAULT_USER_NAME: &'static str = "unknown";
pub const DEFAULT_MAGNET_PARAM: bool = true;
pub const DEFAULT_BLEND: bool = true;
pub const DEFAULT_ALPHA_PARAM: u8 = 150;
pub const DEFAULT_FPS_PARAM: u32 = 60;

// other
pub const MAX_NAME_SIZE: usize = 14;
pub const BORDER: i16 = 6;
pub const SQR_SIZE: u8 = 12;

// default game colors
pub const GAME_BACKGROUND_COLOR: u32 = (100 << 16) + (100 << 8) + 100;
pub const FIELD_BACKGROUND_COLOR: u32 = (170 << 16) + (170 << 8) + 170;
pub const FONT_ACOLOR: u32 = (200 << 16) + (200 << 8) + 200;
pub const FONT_BCOLOR: u32 = (255 << 16) + (255 << 8) + 255;
pub const BORDER_COLOR: u32 = (210 << 16) + (210 << 8) + 210;
pub const FIG_COLOR_01: u32 = (230 << 16) + (100 << 8) + 100;
pub const FIG_COLOR_02: u32 = (230 << 16) + (210 << 8) + 100;
pub const FIG_COLOR_03: u32 = (100 << 16) + (230 << 8) + 100;
pub const FIG_COLOR_04: u32 = (230 << 16) + (100 << 8) + 200;
pub const FIG_COLOR_05: u32 = (100 << 16) + (230 << 8) + 200;
pub const FIG_COLOR_06: u32 = (100 << 16) + (200 << 8) + 230;
pub const FIG_COLOR_07: u32 = (100 << 16) + (100 << 8) + 230;
pub const FIG_COLOR_08: u32 = (210 << 16) + (100 << 8) + 230;
