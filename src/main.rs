#![windows_subsystem = "windows"]
#![allow(unused_assignments)]
extern crate backtrace;
extern crate sdl2;
extern crate tini;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::time::SystemTime;
use std::panic;
use std::{fs::File, io::Write};
use tini::Ini;

#[macro_use]
mod extra;
mod game;
mod random;
mod render;

// handle panic and write crash repot to file
fn panic_handler(panic_info: &panic::PanicInfo) {
    let mut buffer = String::new();
    buffer.push_str(&format!(
        "The application had a problem and crashed.\n\
         To help us diagnose the problem you can send us a crash report.\n\n\
         Authors: {}\n\n\
         We take privacy seriously, and do not perform any automated error collection.\n\
         In order to improve the software, we rely on people to submit reports.\n\n\
         Thank you!\n\n\
         --- crash report start ---\n\
         name: {}\n\
         version: {}\n\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    ));
    match panic_info.location() {
        Some(location) => {
            let info = format!("panic occurred in file '{}' at line {}\n", location.file(), location.line());
            buffer.push_str(&info);
        }
        None => buffer.push_str("panic occurred but can't get location information...\n"),
    }
    buffer.push_str("stack backtrace:\n");
    let mut index = 0;
    backtrace::trace(|frame| {
        let ip = frame.ip();
        let symbol_address = frame.symbol_address();
        backtrace::resolve(ip, |symbol| {
            if let Some(name) = symbol.name() {
                let symbol_info = format!("\t{}: {} @ {:?}\n", index, name, symbol_address);
                buffer.push_str(&symbol_info);
                index += 1;
            }
            match (symbol.filename(), symbol.lineno()) {
                (Some(filename), Some(line)) => {
                    let file_info = format!("\t\t\tat {}:{}\n", filename.display(), line);
                    buffer.push_str(&file_info);
                }
                _ => {}
            }
        });
        true
    });
    buffer.push_str("--- crash report end ---");
    File::create("crash.log")
        .and_then(|mut file| write!(file, "{}", buffer))
        .unwrap_or_else(|_| println!("{}", buffer));
}

fn main() {
    // handle panics
    panic::set_hook(Box::new(panic_handler));

    // default config
    const DEFAULT_CONFIG: &'static str = r#"
        [config]
        magnetization = true
        blend = true
        alpha = 150
        fps = 30
        [score]
        value = 0
        time = 00:00:00
    "#;
    // resource & config
    const FONT_FILE: &'static str = "./resources/FiraMono-Regular.ttf";
    const CONFIG_FILE: &'static str = "./resources/config.ini";
    // game title
    const GT: &'static str = "1010";
    const MILLISECOND: u32 = 1000;
    // game score multiplier
    const LINE_MULTIPLIER: u32 = 10;
    const BASKET_COUNT: u8 = 3;
    const BASKET_SIZE: u8 = 5;
    const FIELD_SIZE: u8 = 10;
    const FIELD_SHIFT: i16 = 10;
    // field tile size & separator
    const TILE_SIZE_1: u8 = 32;
    const TILE_SEP_1: u8 = 3;
    // basket tile size & separator
    const TILE_SIZE_2: u8 = TILE_SIZE_1 / 2;
    const TILE_SEP_2: u8 = 2;
    // game block round rect
    const ROUND_RADIUS: i16 = 4;
    // gameover round rect radius
    const BIG_ROUND_RADIUS: i16 = 8;
    const FIELD_WIDTH: u32 = (TILE_SIZE_1 as u32 + TILE_SEP_1 as u32) * FIELD_SIZE as u32 + 2 * FIELD_SHIFT as u32;
    const BASKET_LEN: u32 = (TILE_SIZE_2 as u32 + TILE_SEP_2 as u32) * BASKET_SIZE as u32 + FIELD_SHIFT as u32;
    // game window size
    const W_WIDTH: u32 = FIELD_WIDTH + BASKET_LEN;
    const W_HEIGHT: u32 = FIELD_WIDTH;
    // font consts
    const FONT_DEF_SIZE: u16 = 18;
    const FONT_BIG_SIZE: u16 = 48;
    const FONT_HEIGHT: i16 = FONT_DEF_SIZE as i16 + 2;

    // load game config
    let config = match Ini::from_file(CONFIG_FILE) {
        Ok(value) => value,
        Err(_) => Ini::from_buffer(DEFAULT_CONFIG),
    };
    let magnetization = config.get("config", "magnetization").unwrap_or(true);
    let alpha_value = config.get("config", "alpha").unwrap_or(150);

    // available game figures
    let figures = vec![
        figure!(Color::RGB(230, 100, 100); (0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)),
        figure!(Color::RGB(230, 100, 100); (0, 0), (1, 0), (0, 1), (1, 1)),
        figure!(Color::RGB(230, 100, 100); (0, 0)),
        figure!(Color::RGB(230, 210, 100); (0, 0), (0, 1), (0, 2), (0, 3), (0, 4)),
        figure!(Color::RGB(230, 210, 100); (0, 0), (1, 0), (2, 0), (3, 0), (4, 0)),
        figure!(Color::RGB(100, 230, 100); (0, 0), (0, 1), (0, 2), (0, 3)),
        figure!(Color::RGB(100, 230, 100); (0, 0), (1, 0), (2, 0), (3, 0)),
        figure!(Color::RGB(230, 100, 200); (0, 0), (0, 1), (0, 2)),
        figure!(Color::RGB(230, 100, 200); (0, 0), (1, 0), (2, 0)),
        figure!(Color::RGB(100, 230, 200); (0, 0), (0, 1)),
        figure!(Color::RGB(100, 230, 200); (0, 0), (1, 0)),
        figure!(Color::RGB(100, 200, 230); (0, 0), (1, 0), (2, 0), (2, 1), (2, 2)),
        figure!(Color::RGB(100, 200, 230); (2, 0), (2, 1), (0, 2), (1, 2), (2, 2)),
        figure!(Color::RGB(100, 200, 230); (0, 0), (0, 1), (0, 2), (1, 2), (2, 2)),
        figure!(Color::RGB(100, 200, 230); (0, 0), (1, 0), (2, 0), (0, 1), (0, 2)),
        figure!(Color::RGB(100, 100, 230); (0, 0), (1, 0), (2, 0), (2, 1)),
        figure!(Color::RGB(100, 100, 230); (1, 0), (1, 1), (0, 2), (1, 2)),
        figure!(Color::RGB(100, 100, 230); (0, 0), (0, 1), (1, 1), (2, 1)),
        figure!(Color::RGB(100, 100, 230); (0, 0), (1, 0), (0, 1), (0, 2)),
        figure!(Color::RGB(210, 100, 230); (0, 0), (1, 0), (1, 1)),
        figure!(Color::RGB(210, 100, 230); (1, 0), (0, 1), (1, 1)),
        figure!(Color::RGB(210, 100, 230); (0, 0), (0, 1), (1, 1)),
        figure!(Color::RGB(210, 100, 230); (0, 0), (1, 0), (0, 1)),
    ];
    // default colors
    let bg_color = Color::RGB(100, 100, 100);
    let field_bg_color = Color::RGB(170, 170, 170);
    let font_color = Color::RGB(200, 200, 200);
    let border_color = Color::RGB(210, 210, 210);

    // objects positions
    let basket_pos = coord!(FIELD_WIDTH as i16, 69);
    let basket_shift = coord!(0, BASKET_LEN as i16);
    let field_pos = coord!(FIELD_SHIFT, FIELD_SHIFT);
    let score_pos = coord!(FIELD_WIDTH as i16, FIELD_SHIFT - 3);
    let highscore_pos = score_pos + coord!(0, FONT_HEIGHT - 1);
    let timer_pos = highscore_pos + coord!(0, FONT_HEIGHT - 1);
    let mut mouse_pos = coord!(0, 0);
    let mut figure_pos = coord!(0, 0);

    // game scores
    let mut highscore = config.get("score", "value").unwrap_or(0);
    let mut score: u32 = 0;
    // game over params
    let mut gameover_flag = false;
    let gameover_pos = coord!(100, (W_HEIGHT as i16 - FONT_BIG_SIZE as i16 - 5) / 2);
    let bp3 = gameover_pos - coord!(10, 0);
    let bp4 = gameover_pos + coord!(269, FONT_BIG_SIZE as i16 + 8);
    let border_size = coord!(8, 8);
    let bp1 = bp3 - border_size;
    let bp2 = bp4 + border_size;

    // SDL2
    let sdl_context = sdl2::init().expect("Can't init sdl2 context");
    let video_subsystem = sdl_context.video().expect("Can't create video subsystem");
    let window =
        video_subsystem.window(GT, W_WIDTH, W_HEIGHT).position_centered().build().expect("Can't create window");
    let mut canvas = window.into_canvas().build().expect("Can't get canvas");
    let mut timer = msg!(sdl_context.timer(); canvas.window(), GT);
    let ttf_context = msg!(sdl2::ttf::init().map_err(|e| e.to_string()); canvas.window(), GT);
    // TODO: rewrite to Font struct
    let font = msg!(ttf_context.load_font(FONT_FILE, FONT_DEF_SIZE); canvas.window(), GT);
    let font_big = msg!(ttf_context.load_font(FONT_FILE, FONT_BIG_SIZE); canvas.window(), GT);

    // turn on alpha channel
    if config.get("config", "blend").unwrap_or(true) {
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    }

    // game objects
    let mut current_figure: Option<game::Figure> = None;
    let mut field = game::Field::init_square(FIELD_SIZE, TILE_SIZE_1, TILE_SEP_1, field_pos);
    let mut basket =
        game::BasketSystem::new(BASKET_COUNT, BASKET_SIZE, TILE_SIZE_2, TILE_SEP_2, basket_pos, basket_shift);

    // fill basket by random figures
    basket.fill(&figures);

    // fps block
    let fps = config.get("config", "fps").unwrap_or(30);
    let mut elapsed = 0;
    let mut last_time = 0;

    // game timer
    let mut game_start = SystemTime::now();
    let mut game_stop = game_start.elapsed();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        last_time = timer.ticks();

        // render text, field & basket
        canvas.set_draw_color(bg_color);
        canvas.clear();
        msg!(render::font(&mut canvas, &font, score_pos, font_color, &format!("{:08}", score)); canvas.window(), GT);
        msg!(render::font(&mut canvas, &font, highscore_pos, font_color, &format!("{:08}", highscore));
                 canvas.window(), GT);
        msg!(render::font(&mut canvas, &font, timer_pos, font_color, &extra::as_time_str(&game_stop));
                 canvas.window(), GT);
        msg!(field.render(&mut canvas, field_bg_color, ROUND_RADIUS); canvas.window(), GT);
        msg!(basket.render(&mut canvas, field_bg_color, ROUND_RADIUS); canvas.window(), GT);

        if gameover_flag {
            // render gameover message
            msg!(render::fill_rounded_rect(&mut canvas, bp1, bp2, BIG_ROUND_RADIUS, border_color); canvas.window(), GT);
            msg!(render::fill_rounded_rect(&mut canvas, bp3, bp4, BIG_ROUND_RADIUS, bg_color); canvas.window(), GT);
            msg!(render::font(&mut canvas, &font_big, gameover_pos, font_color, "GAME OVER"); canvas.window(), GT);
        } else {
            // or count game timer
            game_stop = game_start.elapsed();
        }

        // render selected figure (if they catched)
        if let Some(figure) = &current_figure {
            let size_1 = coord!(TILE_SIZE_1 as i16, TILE_SIZE_1 as i16);
            let size_2 = coord!(TILE_SIZE_2 as i16, TILE_SIZE_2 as i16);
            let sep = coord!(TILE_SEP_1 as i16, TILE_SEP_1 as i16);
            figure_pos = if field.is_point_in(&mouse_pos) && magnetization {
                field.get_point_in(&mouse_pos, &figure)
            } else {
                mouse_pos - size_2
            };
            msg!(figure.render(&mut canvas, figure_pos, size_1, sep, alpha_value, ROUND_RADIUS); canvas.window(), GT);
        }
        canvas.present();

        // events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos.x = x as i16;
                    mouse_pos.y = y as i16;
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if gameover_flag {
                        // restart game
                        gameover_flag = false;
                        basket.fill(&figures);
                        field.clear();
                        score = 0;
                        game_start = SystemTime::now();
                        continue;
                    }
                    // figure set | return
                    current_figure = match current_figure {
                        Some(ref figure) => {
                            let sel_pos = if magnetization { figure_pos } else { mouse_pos };
                            if !field.set_figure(&sel_pos, &figure) {
                                basket.ret(figure.clone());
                            } else {
                                score += figure.blocks();
                            }
                            None
                        }
                        None => basket.get(coord!(x as i16, y as i16)),
                    };
                }
                _ => {}
            }
        }

        // calculate score
        if let Some(lines) = field.next_state() {
            score += (lines.x + lines.y + lines.x * lines.y) * LINE_MULTIPLIER;
        }
        // refill baskets
        if current_figure == None {
            basket.check_and_refill(&figures);
        }
        // update highscore
        highscore = highscore.max(score);
        // check gameover
        if !field.can_set(basket.figures()) && current_figure == None {
            gameover_flag = true;
            current_figure = None;
        }

        // fps counter
        let current_time = timer.ticks();
        elapsed = current_time - last_time;
        last_time = current_time;

        // sleep
        let sleep_time = if elapsed < MILLISECOND / fps { MILLISECOND / fps - elapsed } else { MILLISECOND / fps };
        if sleep_time > 0 {
            timer.delay(sleep_time);
        }
    }

    if config.get("score", "value") < Some(highscore) {
        // update game highscore
        msg!(config.section("score")
                       .item("value", &format!("{}", highscore))
                       .item("time", &extra::as_time_str(&game_stop))
                       .to_file(CONFIG_FILE); canvas.window(), GT);
    }
}
