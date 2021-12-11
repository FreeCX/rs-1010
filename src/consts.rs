use tini::Ini;

// game strings
pub const GAME_OVER_TEXT: &str = "your name: ";
pub const GAME_OVER: &str = "GAME OVER";
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
pub const DEFAULT_USER_NAME: &str = "unknown";
pub const DEFAULT_MAGNET_PARAM: bool = true;
pub const DEFAULT_BLEND: bool = true;
pub const DEFAULT_ALPHA_PARAM: u8 = 150;
pub const DEFAULT_FPS_PARAM: u32 = 60;

// other
pub const MAX_NAME_SIZE: usize = 14;
pub const BORDER: i16 = 6;
pub const SQR_SIZE: u8 = 12;

// default game colors
pub const GAME_BACKGROUND_COLOR: (u8, u8, u8) = (100, 100, 100);
pub const FIELD_BACKGROUND_COLOR: (u8, u8, u8) = (170, 170, 170);
pub const FONT_ACOLOR: (u8, u8, u8) = (200, 200, 200);
pub const FONT_BCOLOR: (u8, u8, u8) = (255, 255, 255);
pub const BORDER_COLOR: (u8, u8, u8) = (210, 210, 210);
pub const FIG_COLOR_01: (u8, u8, u8) = (230, 100, 100);
pub const FIG_COLOR_02: (u8, u8, u8) = (230, 210, 100);
pub const FIG_COLOR_03: (u8, u8, u8) = (100, 230, 100);
pub const FIG_COLOR_04: (u8, u8, u8) = (230, 100, 200);
pub const FIG_COLOR_05: (u8, u8, u8) = (100, 230, 200);
pub const FIG_COLOR_06: (u8, u8, u8) = (100, 200, 230);
pub const FIG_COLOR_07: (u8, u8, u8) = (100, 100, 230);
pub const FIG_COLOR_08: (u8, u8, u8) = (210, 100, 230);

// id for audio effects
pub const SOUND_CLICK_ID: u8 = 0;
pub const SOUND_CLACK_ID: u8 = 1;
pub const SOUND_CLEAR_ID: u8 = 2;
// batch effect block
pub const SOUND_TRACKS: [(u8, &str); 3] = [
    (SOUND_CLICK_ID, "./resources/click.ogg"),
    (SOUND_CLACK_ID, "./resources/clack.ogg"),
    (SOUND_CLEAR_ID, "./resources/clear.ogg"),
];
// background music info
pub const MUSIC_BG_ID: u8 = 0;
pub const MUSIC_GAMEOVER_ID: u8 = 1;
pub const MUSIC_TRACKS: [(u8, &str); 2] =
    [(MUSIC_BG_ID, "./resources/background.mp3"), (MUSIC_GAMEOVER_ID, "./resources/gameover.mp3")];
// audio system default values
pub const DEFAULT_SOUND_VOLUME: u8 = 128;
pub const DEFAULT_MUSIC_VOLUME: u8 = 128;
pub const DEFAULT_SOUND_ENABLE: bool = true;
pub const DEFAULT_MUSIC_ENABLE: bool = true;

// ... you know what it is
pub fn default_config() -> Ini {
    Ini::new()
        .section("score")
        .items(vec![("users", ""), ("scores", ""), ("times", "")])
        .section("game")
        .item("show_highscore_at_start", false)
        .item("magnetization", false)
        .item("blend", true)
        .item("alpha", 150)
        .item("fps", 60)
        .item("username", "user")
        .item("ask_username", true)
        .section("audio")
        .item("enable_sound", true)
        .item("enable_music", true)
        .item("volume_sound", 20)
        .item("volume_music", 128)
        .section("color")
        .item_vec("game_background", &[100, 100, 100])
        .item_vec("field_background", &[170, 170, 170])
        .item_vec("font", &[200, 200, 200])
        .item_vec("light", &[255, 255, 255])
        .item_vec("border", &[210, 210, 210])
        .item_vec("fig1", &[230, 100, 100])
        .item_vec("fig2", &[230, 210, 100])
        .item_vec("fig3", &[100, 230, 100])
        .item_vec("fig4", &[230, 100, 200])
        .item_vec("fig5", &[100, 230, 200])
        .item_vec("fig6", &[100, 200, 230])
        .item_vec("fig7", &[100, 100, 230])
        .item_vec("fig8", &[210, 100, 230])
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
