#![windows_subsystem = "windows"]
use std::panic;
use std::time::SystemTime;

use sdl2::controller::{Axis, Button};
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};
use sdl2::mouse::MouseButton;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::BlendMode;
use sdl2::surface::Surface;

use tini::Ini;

use crate::consts::*;

#[macro_use]
mod extra;
mod audio;
mod build;
mod consts;
mod game;
mod handler;
mod random;
mod render;
mod save;
mod score;

fn v_as_color(config: &Ini, section: &str, param: &str, default: &[u8; 3]) -> Color {
    let color = match config.get_vec::<u8>(section, param) {
        Some(value) => {
            match value[..] {
                // suport only RGB24
                [r, g, b] => &[r, g, b],
                _ => default,
            }
        }
        None => default,
    };
    Color::RGBA(color[0], color[1], color[2], 255)
}

fn main() {
    // handle panics
    panic::set_hook(Box::new(handler::panic_handler));

    // load game config
    let mut config = match Ini::from_file(CONFIG_FILE) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("[warning] problem with config: {}", e);
            default_config()
        }
    };
    let magnetization = config.get("game", "magnetization").unwrap_or(DEFAULT_MAGNET_PARAM);
    let alpha_value = config.get("game", "alpha").unwrap_or(DEFAULT_ALPHA_PARAM);
    let cfg_user_name = config.get("game", "username").unwrap_or_else(|| DEFAULT_USER_NAME.to_string());
    let ask_username = config.get("game", "ask_username").unwrap_or_else(|| cfg_user_name == DEFAULT_USER_NAME);
    let show_fps = config.get("game", "show_fps").unwrap_or(DEFAULT_SHOW_FPS);
    let mut score_table = score::ScoreTable::from_config(&config);

    // objects positions
    let basket_pos = coord!(FIELD_WIDTH as i16, FIELD_SHIFT_HEIGHT + 4 * FONT_HEIGHT);
    let basket_shift = coord!(0, BASKET_HEIGHT as i16);
    let field_pos = coord!(FIELD_SHIFT_WIDTH, FIELD_SHIFT_HEIGHT);
    let score_pos = coord!(FIELD_WIDTH as i16 + 3, FIELD_SHIFT_HEIGHT - 3);
    let highscore_pos = score_pos + coord!(0, FONT_HEIGHT - 1);
    let timer_pos = highscore_pos + coord!(0, FONT_HEIGHT - 1);
    let separator_pos = timer_pos + coord!(0, FONT_HEIGHT - 1);
    let mut mouse_pos = coord!();
    let mut figure_pos = coord!();

    // SDL2
    let sdl_context = sdl2::init().expect(INIT_SDL_ERROR);
    let video_subsystem = sdl_context.video().unwrap_or_else(|_| subsystem_panic!(create; "video"));
    // init audio subsystem
    let _audio_subsystem = sdl_context.audio().unwrap_or_else(|_| subsystem_panic!(create; "audio"));
    sdl2::mixer::open_audio(44100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1024)
        .unwrap_or_else(|_| subsystem_panic!(open; "audio device"));
    let _mixer_context = sdl2::mixer::init(InitFlag::all()).unwrap_or_else(|_| subsystem_panic!(create; "mixer"));
    sdl2::mixer::allocate_channels(4);

    // configure controller
    let game_controller_subsystem =
        sdl_context.game_controller().unwrap_or_else(|_| subsystem_panic!(create; "controller"));
    let available =
        game_controller_subsystem.num_joysticks().unwrap_or_else(|_| subsystem_panic!(create; "controller"));
    let _controller = (0..available).find_map(|id| {
        if !game_controller_subsystem.is_game_controller(id) {
            return None;
        }
        match game_controller_subsystem.open(id) {
            Ok(c) => Some(c),
            Err(_) => None,
        }
    });

    let window = video_subsystem.window(GT, W_WIDTH, W_HEIGHT).position_centered().build().expect(INIT_WINDOW_ERROR);
    let mut canvas = window.into_canvas().build().expect(GET_CANVAS_ERROR);
    let timer = msg!(sdl_context.timer(); canvas.window(), GT);
    let ttf_context = msg!(sdl2::ttf::init().map_err(|e| e.to_string()); canvas.window(), GT);

    // TODO: rewrite to Font struct
    let font = msg!(ttf_context.load_font(FONT_FILE, FONT_DEF_SIZE); canvas.window(), GT);
    let font_big = msg!(ttf_context.load_font(FONT_FILE, FONT_BIG_SIZE); canvas.window(), GT);
    let font_min = msg!(ttf_context.load_font(FONT_FILE, FONT_MIN_SIZE); canvas.window(), GT);

    // game pixel format
    let pixel_fmt = PixelFormatEnum::RGBA32;

    // configure audio system
    let mut audio = audio::AudioSystem::new();
    audio.set_sfx_status(config.get("audio", "enable_sfx").unwrap_or(DEFAULT_SFX_ENABLE));
    audio.set_music_status(config.get("audio", "enable_music").unwrap_or(DEFAULT_MUSIC_ENABLE));
    audio.set_sfx_volume(config.get("audio", "volume_sound").unwrap_or(DEFAULT_SFX_VOLUME));
    audio.set_music_volume(config.get("audio", "volume_music").unwrap_or(DEFAULT_MUSIC_VOLUME));
    // and load all audio
    audio.batch_load_sfx(SFX_TRACKS);
    audio.batch_load_music(MUSIC_TRACKS);
    // start playing bg music
    audio.play_music(MUSIC_BG_ID, audio::MusicLoop::Repeat);

    // game palette
    let palette = [
        // 00
        v_as_color(&config, "color", "fig1", FIG_COLOR_01),
        // 01
        v_as_color(&config, "color", "fig2", FIG_COLOR_02),
        // 02
        v_as_color(&config, "color", "fig3", FIG_COLOR_03),
        // 03
        v_as_color(&config, "color", "fig4", FIG_COLOR_04),
        // 04
        v_as_color(&config, "color", "fig5", FIG_COLOR_05),
        // 05
        v_as_color(&config, "color", "fig6", FIG_COLOR_06),
        // 06
        v_as_color(&config, "color", "fig7", FIG_COLOR_07),
        // 07
        v_as_color(&config, "color", "fig8", FIG_COLOR_08),
        // 08: bg_color
        v_as_color(&config, "color", "game_background", GAME_BACKGROUND_COLOR),
        // 09: field_bg_color
        v_as_color(&config, "color", "field_background", FIELD_BACKGROUND_COLOR),
        // 10: font_color
        v_as_color(&config, "color", "font", FONT_ACOLOR),
        // 11: light_font_color
        v_as_color(&config, "color", "light", FONT_BCOLOR),
        // 12: border_color
        v_as_color(&config, "color", "border", BORDER_COLOR),
    ];

    // available game figures
    let figures = &[
        // ###
        // ###
        // ###
        figure!(1, palette[0]; (0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2), (2, 2)),
        // ##
        // ##
        figure!(2, palette[0]; (0, 0), (1, 0), (0, 1), (1, 1)),
        // #
        figure!(3, palette[0]; (0, 0)),
        // #####
        figure!(4, palette[1]; (0, 0), (0, 1), (0, 2), (0, 3), (0, 4)),
        // #
        // #
        // #
        // #
        // #
        figure!(5, palette[1]; (0, 0), (1, 0), (2, 0), (3, 0), (4, 0)),
        // #
        // #
        // #
        // #
        figure!(6, palette[2]; (0, 0), (0, 1), (0, 2), (0, 3)),
        // ####
        figure!(7, palette[2]; (0, 0), (1, 0), (2, 0), (3, 0)),
        // #
        // #
        // #
        figure!(8, palette[3]; (0, 0), (0, 1), (0, 2)),
        // ###
        figure!(9, palette[3]; (0, 0), (1, 0), (2, 0)),
        // #
        // #
        figure!(10, palette[4]; (0, 0), (0, 1)),
        // ##
        figure!(11, palette[4]; (0, 0), (1, 0)),
        // ###
        //   #
        //   #
        figure!(12, palette[5]; (0, 0), (1, 0), (2, 0), (2, 1), (2, 2)),
        //   #
        //   #
        // ###
        figure!(13, palette[5]; (2, 0), (2, 1), (0, 2), (1, 2), (2, 2)),
        // #
        // #
        // ###
        figure!(14, palette[5]; (0, 0), (0, 1), (0, 2), (1, 2), (2, 2)),
        // ###
        // #
        // #
        figure!(15, palette[5]; (0, 0), (1, 0), (2, 0), (0, 1), (0, 2)),
        // ###
        //   #
        figure!(16, palette[6]; (0, 0), (1, 0), (2, 0), (2, 1)),
        //   #
        // ###
        figure!(17, palette[6]; (1, 0), (1, 1), (0, 2), (1, 2)),
        // #
        // ###
        figure!(18, palette[6]; (0, 0), (0, 1), (1, 1), (2, 1)),
        // ##
        // #
        // #
        figure!(19, palette[6]; (0, 0), (1, 0), (0, 1), (0, 2)),
        // ##
        //  #
        figure!(20, palette[7]; (0, 0), (1, 0), (1, 1)),
        //  #
        // ##
        figure!(21, palette[7]; (1, 0), (0, 1), (1, 1)),
        // #
        // ##
        figure!(22, palette[7]; (0, 0), (0, 1), (1, 1)),
        // ##
        // #
        figure!(23, palette[7]; (0, 0), (1, 0), (0, 1)),
    ];

    // game scores
    let mut highscore = score_table.get_highscore();
    let mut score: u32 = 0;
    // game over params
    let mut gameover_flag = config.get("game", "show_highscore_at_start").unwrap_or(DEFAULT_HIGHSCORE_AT_START);
    let mut user_name = String::new();
    // rendering params
    let (fsx, fsy) = msg!(font_big.size_of(GAME_OVER); canvas.window(), GT);
    let mut name_input_flag = false;

    // turn on alpha channel
    if config.get("game", "blend").unwrap_or(DEFAULT_BLEND) {
        canvas.set_blend_mode(BlendMode::Blend);
    }

    // game objects
    let mut current_figure: Option<game::Figure> = None;
    let mut field = game::Field::init_square(FIELD_LEN, TILE_SIZE_1, TILE_SEP_1, ROUND_STEPS, ROUND_RADIUS, field_pos);
    let mut basket = game::BasketSystem::new(
        BASKET_COUNT,
        BASKET_SIZE,
        TILE_SIZE_2,
        TILE_SEP_2,
        BASKET_ROUND_STEPS,
        ROUND_RADIUS,
        basket_pos,
        basket_shift,
    );

    // font rendering surface
    let surface_size = Rect::new(0, 0, W_WIDTH, W_HEIGHT);
    let mut surface = msg!(Surface::new(W_WIDTH, W_HEIGHT, pixel_fmt); canvas.window(), GT);
    msg!(surface.set_blend_mode(BlendMode::Blend); canvas.window(), GT);
    let surface_bg = Color::RGBA(palette[8].r, palette[8].g, palette[8].b, 0);

    // fill basket by random figures
    basket.rnd_fill(figures);

    // fps block
    let fps = config.get("game", "fps").unwrap_or(DEFAULT_FPS_PARAM);
    let target_delta = MILLISECOND / fps;
    let mut last_tick = timer.ticks();
    let mut current_delta = 0;

    // game timer
    let mut game_start = SystemTime::now();
    let mut game_stop = game_start.elapsed();

    // restore game state
    if let Some(state) = config.get::<String>("game", "state") {
        // remove state from config (ignore errors)
        config = config.section("game").erase("state");
        let _ = config.to_file(CONFIG_FILE);
        // deserialize
        save::deserialize(state, &palette[0], figures, &mut field, &mut basket, &mut score, &mut game_start);
        // update timer
        game_stop = game_start.elapsed();
    }

    let mut event_pump = msg!(sdl_context.event_pump(); canvas.window(), GT);
    'running: loop {
        // block fps at target
        let current_tick = timer.ticks();
        current_delta += current_tick - last_tick;
        last_tick = current_tick;

        if current_delta < target_delta {
            // this is not very precise delay
            timer.delay(target_delta - current_delta);
            continue;
        }

        canvas.set_draw_color(palette[8]);
        canvas.clear();

        // field and basket
        msg!(field.render(&mut canvas, palette[9], palette[8]); canvas.window(), GT);
        msg!(basket.render(&mut canvas, palette[9], palette[8]); canvas.window(), GT);

        // score, highscore and timer
        msg!(surface.fill_rect(surface_size, surface_bg); canvas.window(), GT);
        msg!(render::font(&mut surface, &font, score_pos, palette[10], palette[8], &format!("{:08}", score)); canvas.window(), GT);
        msg!(render::font(&mut surface, &font, highscore_pos, palette[10], palette[8], &format!("{:08}", highscore)); canvas.window(), GT);
        msg!(render::font(&mut surface, &font, timer_pos, palette[10], palette[8], &extra::as_time_str(&game_stop)); canvas.window(), GT);
        msg!(render::font(&mut surface, &font, separator_pos, palette[10], palette[8], "————————"); canvas.window(), GT);

        if show_fps {
            msg!(render::font(&mut surface, &font, coord!(10), palette[10], palette[8], &format!("{fps}")); canvas.window(), GT);
        }

        if gameover_flag && !name_input_flag {
            // highscore table
            let mut scores = Vec::new();
            let mut ss = coord!();
            let mut curr_score = None;
            let mut max_score_width = fsx;

            for (index, score::Score { name, score, time, last }) in
                score_table.iter().take(GAMESCORE_COUNT).enumerate()
            {
                let name = if name.chars().count() > MAX_NAME_SIZE {
                    format!("{}...", &name[..MAX_NAME_SIZE - 3])
                } else {
                    name.to_string()
                };
                if *last {
                    curr_score = Some(index);
                }
                let score = format!("{}. {: <4$} {:08} ({})", index + 1, name, score, time, MAX_NAME_SIZE);
                let (ssx, ssy) = msg!(font_min.size_of(&score); canvas.window(), GT);
                ss.y += ssy as i16;
                ss.x = ss.x.max(ssx as i16);
                max_score_width = max_score_width.max(font_min.size_of(&score).unwrap_or((0, 0)).0);
                scores.push(score);
            }

            let fp1 =
                coord!((W_WIDTH as i16 - max_score_width as i16) >> 1, (W_HEIGHT as i16 - fsy as i16 - ss.y) >> 1);
            let p1 = fp1 - 2 * BORDER;
            let p2 = fp1 + coord!(max_score_width as i16, ss.y + fsy as i16 - BORDER) + 2 * BORDER;
            let p3 = p1 + BORDER;
            let p4 = p2 - BORDER;

            msg!(render::fill_rect(&mut canvas, p1, p2, palette[12]); canvas.window(), GT);
            msg!(render::fill_rect(&mut canvas, p3, p4, palette[8]); canvas.window(), GT);
            msg!(render::font(&mut surface, &font_big, fp1 - coord!(-10, 5), palette[10], palette[8], GAME_OVER); canvas.window(), GT);
            for (index, text) in scores.iter().enumerate() {
                let fp2 = fp1 + coord!(0, fsy as i16 + index as i16 * (ss.y / scores.len() as i16)) - coord!(0, BORDER);
                let fcolor = if Some(index) == curr_score { palette[11] } else { palette[10] };
                msg!(render::font(&mut surface, &font_min, fp2, fcolor, palette[8], text); canvas.window(), GT);
            }
        } else if gameover_flag && name_input_flag {
            // gameover input name
            let input_name = format!("{}{}", GAME_OVER_TEXT, user_name);

            // prepare textures for input name form
            let inf_ssy = (3 * FONT_MIN_SIZE) as i16;
            let inf_fp1 = coord!((W_WIDTH as i16 - fsx as i16) >> 1, (W_HEIGHT as i16 - fsy as i16 - inf_ssy) >> 1);
            let inf_fp2 = inf_fp1 + coord!(0, fsy as i16 - BORDER);
            let p1 = inf_fp1 - 2 * BORDER;
            let p2 = inf_fp1 + coord!(fsx as i16, inf_ssy + fsy as i16 - BORDER) + 2 * BORDER;
            let p3 = p1 + BORDER;
            let p4 = p2 - BORDER;

            msg!(render::fill_rect(&mut canvas, p1, p2, palette[12]); canvas.window(), GT);
            msg!(render::fill_rect(&mut canvas, p3, p4, palette[8]); canvas.window(), GT);
            msg!(render::font(&mut surface, &font_big, inf_fp1, palette[10], palette[8], GAME_OVER); canvas.window(), GT);
            msg!(render::font(&mut surface, &font, inf_fp2, palette[10], palette[8], &input_name); canvas.window(), GT);
        } else {
            // stop game timer
            game_stop = game_start.elapsed();
        }

        // events
        for event in event_pump.poll_iter() {
            match event {
                // exit the game
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. }
                | Event::ControllerButtonDown { button: Button::Back, .. } => break 'running,

                // add user name to score table
                Event::TextInput { text, .. } => {
                    if name_input_flag && user_name.chars().count() < MAX_NAME_SIZE {
                        user_name.push_str(&text);
                    }
                }

                // input user name
                Event::KeyDown { scancode: Some(key), .. } => {
                    if name_input_flag {
                        match key {
                            Scancode::Return | Scancode::KpEnter => {
                                let fixed_user_name = user_name.replace(',', " ").trim().to_string();
                                // ignore empty user name
                                if fixed_user_name.chars().count() == 0 {
                                    continue;
                                }
                                score_table.push(fixed_user_name, score, extra::as_time_str(&game_stop));
                                user_name.clear();
                                name_input_flag = false;
                                field.clear();
                                basket.clear();
                            }
                            Scancode::Backspace => {
                                user_name.pop();
                            }
                            _ => (),
                        }
                    }
                }

                // store current mouse position
                Event::MouseMotion { x, y, .. } => mouse_pos = coord!(x as i16, y as i16),

                // figure set/return to basket
                Event::ControllerAxisMotion { axis: Axis::TriggerRight, value: AXIS_MAX, .. }
                | Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    if gameover_flag && !name_input_flag {
                        // restart game
                        game_start = SystemTime::now();
                        gameover_flag = false;
                        score = 0;
                        // start playing bg music
                        audio.play_music(MUSIC_BG_ID, audio::MusicLoop::Repeat);
                        continue;
                    }
                    if gameover_flag {
                        continue;
                    }
                    current_figure = match current_figure {
                        Some(ref figure) => {
                            audio.play_sfx(SFX_CLACK_ID);
                            let sel_pos = if magnetization { figure_pos } else { mouse_pos };
                            if !field.set_figure(&sel_pos, figure) {
                                basket.ret(figure.clone());
                            } else {
                                score += figure.blocks() * BLOCK_COST_MULTIPLIER;
                            }
                            None
                        }
                        None => {
                            let item = basket.get(mouse_pos);
                            if item.is_some() {
                                audio.play_sfx(SFX_CLICK_ID);
                            }
                            item
                        }
                    };
                }

                // return figure to basket
                Event::ControllerAxisMotion { axis: Axis::TriggerLeft, value: AXIS_MAX, .. } => {
                    current_figure = match current_figure {
                        Some(figure) => {
                            audio.play_sfx(SFX_CLACK_ID);
                            basket.ret(figure);
                            None
                        }
                        other => other,
                    };
                }

                _ => {}
            }
        }

        // calculate score
        if let Some(lines) = field.next_state() {
            audio.play_sfx(SFX_CLEAR_ID);
            score += (lines.x + lines.y + lines.x * lines.y) * LINE_MULTIPLIER;
        }

        // refill baskets
        if current_figure.is_none() && !gameover_flag {
            basket.check_and_refill(figures);
        }

        // update highscore
        highscore = highscore.max(score);

        // check gameover
        if !field.can_set(basket.figures()) && current_figure.is_none() {
            if !gameover_flag && !name_input_flag {
                audio.stop_music();
                audio.play_music(MUSIC_GAMEOVER_ID, audio::MusicLoop::Once);
                name_input_flag = true;
            }
            // autoset username to score table
            if !ask_username && name_input_flag {
                score_table.push(cfg_user_name.clone(), score, extra::as_time_str(&game_stop));
                name_input_flag = false;
                field.clear();
                basket.clear();
            }
            gameover_flag = true;
        }

        // draw last frame font
        msg!(render::surface_copy(&mut canvas, &surface); canvas.window(), GT);

        // render selected figure (if they catched)
        if let Some(figure) = &current_figure {
            let size_1 = coord!(TILE_SIZE_1 as i16);
            let size_2 = coord!(TILE_SIZE_2 as i16);
            let sep = coord!(TILE_SEP_1 as i16);
            figure_pos = if field.is_point_in(&mouse_pos) && magnetization {
                field.get_point_in(&mouse_pos, figure)
            } else {
                mouse_pos - size_2
            };
            // field already have this texture
            let block_texture = &field.textures[&(TILE_SIZE_1 as i16)];
            msg!(figure.render(&mut canvas, block_texture, figure_pos, size_1, sep, alpha_value); canvas.window(), GT);
        }

        canvas.present();

        // for new render cycle
        current_delta = 0;
    }

    // save game state
    if score > 0 && !gameover_flag && !name_input_flag {
        let state = save::serialize(field, basket, score, game_start);
        config = config.section("game").item("state", state);
    }

    // update highscore results
    msg!(score_table.update_config(GAMESCORE_COUNT, config).to_file(CONFIG_FILE); canvas.window(), GT);
}
