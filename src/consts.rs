use tini::Ini;

// game strings
pub const GAME_OVER_TEXT: &str = "your name: ";
pub const GAME_OVER: &str = "GAME OVER";
pub const GAME_PAUSE: &str = "PAUSED";
pub const GT: &str = "1010";

// errors
pub const INIT_SDL_ERROR: &str = "Cannot init sdl2 context";
pub const INIT_WINDOW_ERROR: &str = "Cannot create window";
pub const GET_CANVAS_ERROR: &str = "Cannot get canvas";
pub const GET_COLOR_ERROR: &str = "Cannot get color";

// resource & config
pub const FONT_FILE: &str = "./resources/FiraMono-Regular.ttf";
pub const CONFIG_FILE: &str = "./resources/config.ini";
pub const GAMESCORE_COUNT: usize = 5;

// game fps param
pub const MILLISECOND: u32 = 1000;

// game score multiplier
pub const LINE_MULTIPLIER: u32 = 30;
pub const BLOCK_COST_MULTIPLIER: u32 = 5;

// filed params
pub const FIELD_LEN: u8 = 10;
pub const FIELD_SHIFT_WIDTH: i16 = 210;
pub const FIELD_SHIFT_HEIGHT: i16 = 60;
pub const FIELD_BASKET_SEP: u32 = 10;
// game block round rect
pub const ROUND_RADIUS: i16 = 8;
pub const ROUND_STEPS: i16 = 20;
// field tile size & separator
pub const TILE_SIZE_1: u8 = 64;
pub const TILE_SEP_1: u8 = 4;

// basket params
pub const BASKET_COUNT: u8 = 3;
pub const BASKET_SIZE: u8 = 5;
pub const BASKET_SHIFT: u8 = 7;
pub const BASKET_ROUND_STEPS: i16 = 8;
// basket tile size & separator
pub const TILE_SIZE_2: u8 = 32;
pub const TILE_SEP_2: u8 = 2;

// gameover round rect radius
pub const FIELD_WIDTH: u32 =
    FIELD_SHIFT_WIDTH as u32 + (TILE_SIZE_1 as u32 + TILE_SEP_1 as u32) * FIELD_LEN as u32 + FIELD_BASKET_SEP;
pub const BASKET_WIDTH: u32 = (TILE_SIZE_2 as u32 + TILE_SEP_2 as u32) * BASKET_SIZE as u32;
pub const BASKET_HEIGHT: u32 = BASKET_WIDTH + BASKET_SHIFT as u32;

// game window size
pub const W_WIDTH: u32 = 1280;
pub const W_HEIGHT: u32 = 800;

// font consts
pub const FONT_MIN_SIZE: u16 = 16;
pub const FONT_DEF_SIZE: u16 = 34;
pub const FONT_BIG_SIZE: u16 = 64;
pub const FONT_HEIGHT: i16 = FONT_DEF_SIZE as i16 + 2;

// defaul game params
pub const DEFAULT_HIGHSCORE_AT_START: bool = false;
pub const DEFAULT_USER_NAME: &str = "user";
pub const DEFAULT_MAGNET_PARAM: bool = true;
pub const DEFAULT_BLEND: bool = true;
pub const DEFAULT_ALPHA_PARAM: u8 = 150;
pub const DEFAULT_FPS_PARAM: u32 = 60;
pub const DEFAULT_SHOW_FPS: bool = false;

// other
pub const MAX_NAME_SIZE: usize = 14;
pub const BORDER: i16 = 6;
pub const MINIMAL_TILE_SIZE: u8 = 4;
pub const TILE_CLEAN_ANIMATION_SIZE: u8 = (TILE_SIZE_1 / 2) - MINIMAL_TILE_SIZE;

// serde bits
pub const SERDE_FIELD_SIZE: u8 = 1;
pub const SERDE_FIGURE_SIZE: u8 = 5;
pub const SERDE_SCORE_SIZE: u8 = 32;
pub const SERDE_TIME_SIZE: u8 = 64;
pub const SERDE_PADDING_SIZE: u8 = 5;
pub const SERDE_V2_SUPPORT: u8 = 0b10101;
pub const SERDE_COLOR: u8 = 8;
pub const SERDE_POS: u8 = 4;

// default game colors
pub const GAME_BACKGROUND_COLOR: &[u8; 3] = &[110, 110, 110];
pub const FIELD_BACKGROUND_COLOR: &[u8; 3] = &[170, 170, 170];
pub const FONT_ACOLOR: &[u8; 3] = &[200, 200, 200];
pub const FONT_BCOLOR: &[u8; 3] = &[255, 255, 255];
pub const BORDER_COLOR: &[u8; 3] = &[210, 210, 210];
pub const FIG_COLOR_01: &[u8; 3] = &[230, 100, 100];
pub const FIG_COLOR_02: &[u8; 3] = &[230, 210, 100];
pub const FIG_COLOR_03: &[u8; 3] = &[100, 230, 100];
pub const FIG_COLOR_04: &[u8; 3] = &[230, 100, 200];
pub const FIG_COLOR_05: &[u8; 3] = &[100, 230, 200];
pub const FIG_COLOR_06: &[u8; 3] = &[100, 200, 230];
pub const FIG_COLOR_07: &[u8; 3] = &[100, 100, 230];
pub const FIG_COLOR_08: &[u8; 3] = &[210, 100, 230];
//
pub const FAKE_K: f32 = 0.5;

// controller stuff
pub const AXIS_MAX: i16 = i16::MAX;

// id for audio effects
pub const SFX_CLICK_ID: u8 = 0;
pub const SFX_CLACK_ID: u8 = 1;
pub const SFX_CLEAR_ID: u8 = 2;
// batch effect block
pub const SFX_TRACKS: [(u8, &str); 3] = [
    (SFX_CLICK_ID, "./resources/click.ogg"),
    (SFX_CLACK_ID, "./resources/clack.ogg"),
    (SFX_CLEAR_ID, "./resources/clear.ogg"),
];
// background music info
pub const MUSIC_BG_ID: u8 = 0;
pub const MUSIC_GAMEOVER_ID: u8 = 1;
pub const MUSIC_TRACKS: [(u8, &str); 2] =
    [(MUSIC_BG_ID, "./resources/background.mp3"), (MUSIC_GAMEOVER_ID, "./resources/gameover.mp3")];
// audio system default values
pub const DEFAULT_SFX_VOLUME: u8 = 20;
pub const DEFAULT_MUSIC_VOLUME: u8 = 128;
pub const DEFAULT_SFX_ENABLE: bool = true;
pub const DEFAULT_MUSIC_ENABLE: bool = true;

// ... you know what it is
pub fn default_config() -> Ini {
    Ini::new()
        .section("score")
        .items(vec![("users", ""), ("scores", ""), ("times", "")])
        .section("game")
        .item("show_highscore_at_start", DEFAULT_HIGHSCORE_AT_START)
        .item("magnetization", DEFAULT_MAGNET_PARAM)
        .item("blend", DEFAULT_BLEND)
        .item("alpha", DEFAULT_ALPHA_PARAM)
        .item("fps", DEFAULT_FPS_PARAM)
        .item("show_fps", DEFAULT_SHOW_FPS)
        .item("username", DEFAULT_USER_NAME)
        .item("ask_username", true)
        .section("audio")
        .item("enable_sfx", DEFAULT_SFX_ENABLE)
        .item("volume_sfx", DEFAULT_SFX_VOLUME)
        .item("enable_music", DEFAULT_MUSIC_ENABLE)
        .item("volume_music", DEFAULT_MUSIC_VOLUME)
        .section("color")
        .item_vec("game_background", GAME_BACKGROUND_COLOR)
        .item_vec("field_background", FIELD_BACKGROUND_COLOR)
        .item_vec("font", FONT_ACOLOR)
        .item_vec("light", FONT_BCOLOR)
        .item_vec("border", BORDER_COLOR)
        .item_vec("fig1", FIG_COLOR_01)
        .item_vec("fig2", FIG_COLOR_02)
        .item_vec("fig3", FIG_COLOR_03)
        .item_vec("fig4", FIG_COLOR_04)
        .item_vec("fig5", FIG_COLOR_05)
        .item_vec("fig6", FIG_COLOR_06)
        .item_vec("fig7", FIG_COLOR_07)
        .item_vec("fig8", FIG_COLOR_08)
}

#[macro_export]
macro_rules! subsystem_panic {
    (create; $x:expr) => {
        panic!("Cannot create {} subsystem", $x)
    };
    (open; $x:expr) => {
        panic!("Cannot open {}", $x)
    };
}
