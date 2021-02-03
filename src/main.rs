#![windows_subsystem = "windows"]
extern crate backtrace;
extern crate sdl2;
extern crate tini;

use std::convert::TryFrom;
use std::panic;
use std::time::SystemTime;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::MouseButton;
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use tini::Ini;

use crate::consts::*;

#[macro_use]
mod extra;
mod build;
mod consts;
mod game;
mod handler;
mod random;
mod render;
mod save;
mod score;

fn v_as_color(pixel_fmt: &PixelFormat, config: &Ini, section: &str, param: &str, default: u32) -> Color {
    let color = match config.get_vec::<u8>(section, param) {
        Some(value) => {
            match value[..] {
                // suport only RGB24
                [r, g, b] => ((r as u32) << 16) + ((g as u32) << 8) + b as u32,
                _ => default,
            }
        }
        None => default,
    };
    Color::from_u32(&pixel_fmt, color)
}

fn main() {
    // handle panics
    panic::set_hook(Box::new(handler::panic_handler));

    // load game config
    let mut config = match Ini::from_file(CONFIG_FILE) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("[warning] config: {}", e);
            Ini::from_string(DEFAULT_CONFIG).expect("cannot load default config")
        }
    };
    let magnetization = config.get("game", "magnetization").unwrap_or(DEFAULT_MAGNET_PARAM);
    let alpha_value = config.get("game", "alpha").unwrap_or(DEFAULT_ALPHA_PARAM);
    let cfg_user_name = config.get("game", "username").unwrap_or(DEFAULT_USER_NAME.to_string());
    let ask_username = config.get("game", "ask_username").unwrap_or(cfg_user_name == DEFAULT_USER_NAME);
    let mut score_table = score::ScoreTable::from_config(&config);

    // objects positions
    let basket_pos = coord!(FIELD_WIDTH as i16, 69);
    let basket_shift = coord!(0, BASKET_LEN as i16);
    let field_pos = coord!(FIELD_SHIFT, FIELD_SHIFT);
    let score_pos = coord!(FIELD_WIDTH as i16, FIELD_SHIFT - 3);
    let highscore_pos = score_pos + coord!(0, FONT_HEIGHT - 1);
    let timer_pos = highscore_pos + coord!(0, FONT_HEIGHT - 1);
    let mut mouse_pos = coord!();
    let mut figure_pos = coord!();

    // SDL2
    let sdl_context = sdl2::init().expect(INIT_SDL_ERROR);
    let video_subsystem = sdl_context.video().expect(INIT_SDL_SUBSYSTEM_ERROR);
    let window = video_subsystem.window(GT, W_WIDTH, W_HEIGHT).position_centered().build().expect(INIT_WINDOW_ERROR);
    let mut canvas = window.into_canvas().build().expect(GET_CANVAS_ERROR);
    let mut timer = msg!(sdl_context.timer(); canvas.window(), GT);
    let ttf_context = msg!(sdl2::ttf::init().map_err(|e| e.to_string()); canvas.window(), GT);

    // TODO: rewrite to Font struct
    let font = msg!(ttf_context.load_font(FONT_FILE, FONT_DEF_SIZE); canvas.window(), GT);
    let font_big = msg!(ttf_context.load_font(FONT_FILE, FONT_BIG_SIZE); canvas.window(), GT);
    let font_min = msg!(ttf_context.load_font(FONT_FILE, FONT_MIN_SIZE); canvas.window(), GT);

    // game pixel format
    let pixel_fmt: PixelFormat = msg!(PixelFormat::try_from(PixelFormatEnum::RGB24); canvas.window(), GT);

    let palette = [
        // 00
        v_as_color(&pixel_fmt, &config, "color", "fig1", FIG_COLOR_01),
        // 01
        v_as_color(&pixel_fmt, &config, "color", "fig2", FIG_COLOR_02),
        // 02
        v_as_color(&pixel_fmt, &config, "color", "fig3", FIG_COLOR_03),
        // 03
        v_as_color(&pixel_fmt, &config, "color", "fig4", FIG_COLOR_04),
        // 04
        v_as_color(&pixel_fmt, &config, "color", "fig5", FIG_COLOR_05),
        // 05
        v_as_color(&pixel_fmt, &config, "color", "fig6", FIG_COLOR_06),
        // 06
        v_as_color(&pixel_fmt, &config, "color", "fig7", FIG_COLOR_07),
        // 07
        v_as_color(&pixel_fmt, &config, "color", "fig8", FIG_COLOR_08),
        // 08: bg_color
        v_as_color(&pixel_fmt, &config, "color", "game_background", GAME_BACKGROUND_COLOR),
        // 09: field_bg_color
        v_as_color(&pixel_fmt, &config, "color", "field_background", FIELD_BACKGROUND_COLOR),
        // 10: font_color
        v_as_color(&pixel_fmt, &config, "color", "font", FONT_ACOLOR),
        // 11: light_font_color
        v_as_color(&pixel_fmt, &config, "color", "light", FONT_BCOLOR),
        // 12: border_color
        v_as_color(&pixel_fmt, &config, "color", "border", BORDER_COLOR),
    ];

    // available game figures
    let figures = &[
        figure!(1, palette[0]; (0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)),
        figure!(2, palette[0]; (0, 0), (1, 0), (0, 1), (1, 1)),
        figure!(3, palette[0]; (0, 0)),
        figure!(4, palette[1]; (0, 0), (0, 1), (0, 2), (0, 3), (0, 4)),
        figure!(5, palette[1]; (0, 0), (1, 0), (2, 0), (3, 0), (4, 0)),
        figure!(6, palette[2]; (0, 0), (0, 1), (0, 2), (0, 3)),
        figure!(7, palette[2]; (0, 0), (1, 0), (2, 0), (3, 0)),
        figure!(8, palette[3]; (0, 0), (0, 1), (0, 2)),
        figure!(9, palette[3]; (0, 0), (1, 0), (2, 0)),
        figure!(10, palette[4]; (0, 0), (0, 1)),
        figure!(11, palette[4]; (0, 0), (1, 0)),
        figure!(12, palette[5]; (0, 0), (1, 0), (2, 0), (2, 1), (2, 2)),
        figure!(13, palette[5]; (2, 0), (2, 1), (0, 2), (1, 2), (2, 2)),
        figure!(14, palette[5]; (0, 0), (0, 1), (0, 2), (1, 2), (2, 2)),
        figure!(15, palette[5]; (0, 0), (1, 0), (2, 0), (0, 1), (0, 2)),
        figure!(16, palette[6]; (0, 0), (1, 0), (2, 0), (2, 1)),
        figure!(17, palette[6]; (1, 0), (1, 1), (0, 2), (1, 2)),
        figure!(18, palette[6]; (0, 0), (0, 1), (1, 1), (2, 1)),
        figure!(19, palette[7]; (0, 0), (1, 0), (0, 1), (0, 2)),
        figure!(20, palette[7]; (0, 0), (1, 0), (1, 1)),
        figure!(21, palette[7]; (1, 0), (0, 1), (1, 1)),
        figure!(22, palette[7]; (0, 0), (0, 1), (1, 1)),
        figure!(23, palette[7]; (0, 0), (1, 0), (0, 1)),
    ];

    // game scores
    let mut highscore = score_table.get_highscore();
    let mut score: u32 = 0;
    // game over params
    let mut gameover_flag = config.get("game", "show_highscore_at_start").unwrap_or(DEFAULT_HIGHSCORE_AT_START);
    let mut user_name = String::new();
    // rendering params
    let (fsx, fsy) = font_big.size_of(GAME_OVER).unwrap();
    let mut name_input_flag = false;

    // turn on alpha channel
    if config.get("game", "blend").unwrap_or(DEFAULT_BLEND) {
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    }

    // game objects
    let mut current_figure: Option<game::Figure> = None;
    let mut field = game::Field::init_square(FIELD_SIZE, TILE_SIZE_1, TILE_SEP_1, field_pos);
    let mut basket =
        game::BasketSystem::new(BASKET_COUNT, BASKET_SIZE, TILE_SIZE_2, TILE_SEP_2, basket_pos, basket_shift);

    // fill basket by random figures
    basket.rnd_fill(figures);

    // fps block
    let fps = config.get("game", "fps").unwrap_or(DEFAULT_FPS_PARAM);
    let mut last_time = timer.ticks();

    // game timer
    let mut game_start = SystemTime::now();
    let mut game_stop = game_start.elapsed();

    // restore game state
    if let Some(state) = config.get::<String>("game", "state") {
        save::deserialize(state, &palette[6], figures, &mut field, &mut basket, &mut score, &mut game_start);
        game_stop = game_start.elapsed();
    }

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // render cycle: text, field & basket
        canvas.set_draw_color(palette[8]);
        canvas.clear();
        msg!(render::font(&mut canvas, &font, score_pos, palette[10], &format!("{:08}", score)); canvas.window(), GT);
        msg!(render::font(&mut canvas, &font, highscore_pos, palette[10], &format!("{:08}", highscore));
                 canvas.window(), GT);
        msg!(render::font(&mut canvas, &font, timer_pos, palette[10], &extra::as_time_str(&game_stop));
                 canvas.window(), GT);
        msg!(field.render(&mut canvas, palette[9], ROUND_RADIUS); canvas.window(), GT);
        msg!(basket.render(&mut canvas, palette[9], ROUND_RADIUS); canvas.window(), GT);

        if gameover_flag && !name_input_flag {
            // highscore table
            let mut scores = Vec::new();
            let mut ss = coord!();
            let mut curr_score = None;
            for (index, score::Score { name, score, time, last }) in
                score_table.iter().take(GAMESCORE_COUNT).enumerate()
            {
                let name = if name.chars().count() > MAX_NAME_SIZE {
                    format!("{}...", &name[..MAX_NAME_SIZE - 3])
                } else {
                    format!("{}", name)
                };
                if *last {
                    curr_score = Some(index);
                }
                let score = format!("{}. {: <4$} {:08} ({})", index + 1, name, score, time, MAX_NAME_SIZE);
                let (ssx, ssy) = font_min.size_of(&score).unwrap();
                ss.y += ssy as i16;
                ss.x = ss.x.max(ssx as i16);
                scores.push(score);
            }
            let fp1 = coord!((W_WIDTH as i16 - fsx as i16) >> 1, (W_HEIGHT as i16 - fsy as i16 - ss.y) >> 1);
            let p1 = fp1 - 2 * BORDER;
            let p2 = fp1 + coord!(fsx as i16, ss.y + fsy as i16 - BORDER) + 2 * BORDER;
            let p3 = p1 + BORDER;
            let p4 = p2 - BORDER;
            msg!(render::fill_rounded_rect(&mut canvas, p1, p2, BIG_ROUND_RADIUS, palette[12]); canvas.window(), GT);
            msg!(render::fill_rounded_rect(&mut canvas, p3, p4, BIG_ROUND_RADIUS, palette[8]); canvas.window(), GT);
            msg!(render::font(&mut canvas, &font_big, fp1, palette[10], GAME_OVER); canvas.window(), GT);
            for (index, text) in scores.iter().enumerate() {
                let fp2 = fp1 + coord!(0, fsy as i16 + index as i16 * (ss.y / scores.len() as i16)) - coord!(0, BORDER);
                let fcolor = if Some(index) == curr_score { palette[11] } else { palette[10] };
                msg!(render::font(&mut canvas, &font_min, fp2, fcolor, text); canvas.window(), GT);
            }
        } else if gameover_flag && name_input_flag {
            // gameover input name
            let ssy = (2 * FONT_MIN_SIZE) as i16;
            let fp1 = coord!((W_WIDTH as i16 - fsx as i16) >> 1, (W_HEIGHT as i16 - fsy as i16 - ssy) >> 1);
            let p1 = fp1 - 2 * BORDER;
            let p2 = fp1 + coord!(fsx as i16, ssy + fsy as i16 - BORDER) + 2 * BORDER;
            let p3 = p1 + BORDER;
            let p4 = p2 - BORDER;
            let fp2 = fp1 + coord!(0, fsy as i16 - BORDER);
            let input_name = format!("{}{}", GAME_OVER_TEXT, user_name);
            msg!(render::fill_rounded_rect(&mut canvas, p1, p2, BIG_ROUND_RADIUS, palette[12]); canvas.window(), GT);
            msg!(render::fill_rounded_rect(&mut canvas, p3, p4, BIG_ROUND_RADIUS, palette[8]); canvas.window(), GT);
            msg!(render::font(&mut canvas, &font_big, fp1, palette[10], GAME_OVER); canvas.window(), GT);
            msg!(render::font(&mut canvas, &font, fp2, palette[10], &input_name); canvas.window(), GT);
        } else {
            // stop game timer
            game_stop = game_start.elapsed();
        }

        // render selected figure (if they catched)
        if let Some(figure) = &current_figure {
            let size_1 = coord!(TILE_SIZE_1 as i16);
            let size_2 = coord!(TILE_SIZE_2 as i16);
            let sep = coord!(TILE_SEP_1 as i16);
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
                Event::TextInput { text, .. } => {
                    if name_input_flag && user_name.chars().count() < MAX_NAME_SIZE {
                        user_name.push_str(&text);
                    }
                }
                Event::KeyDown { scancode: Some(key), .. } => {
                    if name_input_flag {
                        match key {
                            Scancode::Return | Scancode::KpEnter => {
                                let fixed_user_name = user_name.replace(",", " ").trim().to_string();
                                // ignore empty user name
                                if fixed_user_name.chars().count() == 0 {
                                    continue;
                                }
                                score_table.push(fixed_user_name, score, extra::as_time_str(&game_stop));
                                user_name.clear();
                                name_input_flag = false;
                            }
                            Scancode::Backspace => {
                                user_name.pop();
                            }
                            _ => (),
                        }
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    mouse_pos = coord!(x as i16, y as i16);
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    if gameover_flag && !name_input_flag {
                        // restart game
                        gameover_flag = false;
                        basket.rnd_fill(figures);
                        field.clear();
                        score = 0;
                        game_start = SystemTime::now();
                        continue;
                    }
                    if gameover_flag {
                        continue;
                    }
                    // figure set | return
                    current_figure = match current_figure {
                        Some(ref figure) => {
                            let sel_pos = if magnetization { figure_pos } else { mouse_pos };
                            if !field.set_figure(&sel_pos, &figure) {
                                basket.ret(figure.clone());
                            } else {
                                score += figure.blocks() * BLOCK_COST_MULTIPLIER;
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
            basket.check_and_refill(figures);
        }
        // update highscore
        highscore = highscore.max(score);
        // check gameover
        if !field.can_set(basket.figures()) && current_figure == None {
            if !gameover_flag && !name_input_flag {
                name_input_flag = true;
            }
            // autoset username to score table
            if !ask_username && name_input_flag {
                score_table.push(cfg_user_name.clone(), score, extra::as_time_str(&game_stop));
                name_input_flag = false;
            }
            gameover_flag = true;
        }

        // fps counter
        let current_time = timer.ticks();
        let elapsed = current_time - last_time;
        last_time = current_time;

        // sleep
        let sleep_time = if elapsed < MILLISECOND / fps { MILLISECOND / fps - elapsed } else { MILLISECOND / fps };
        if sleep_time > 0 {
            timer.delay(sleep_time);
        }
    }

    // save game state
    if score > 0 && !gameover_flag {
        let state = save::serialize(field, basket, score, game_start);
        config = config.section("game").item("state", state);
    }

    // update highscore results
    msg!(score_table.to_config(GAMESCORE_COUNT, config).to_file(CONFIG_FILE); canvas.window(), GT);
}
