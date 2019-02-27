extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::fs::File;
use std::io::prelude::{Read, Write};
use std::time::Duration;

#[macro_use]
mod extra;
mod game;
mod random;
mod render;

fn load_highscore(filename: &str) -> u32 {
    if let Ok(mut file) = File::open(filename) {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer.parse().unwrap_or(0)
    } else {
        0
    }
}

fn save_highscore(filename: &str, score: u32) {
    if let Ok(mut file) = File::create(filename) {
        let _ = write!(&mut file, "{}", score);
    }
}

fn main() {
    // game consts
    const HIGHSCORE_FILE: &'static str = "gamescore.txt";
    const LINE_MULTIPLIER: u32 = 10;
    const BASKET_COUNT: u8 = 3;
    const BASKET_SIZE: u8 = 5;
    const FIELD_SIZE: u8 = 10;
    const FIELD_SHIFT: i16 = 10;
    const TILE_SIZE_1: u8 = 32;
    const TILE_SIZE_2: u8 = TILE_SIZE_1 / 2;
    const TILE_SEP_1: u8 = 3;
    const TILE_SEP_2: u8 = 2;
    const ROUND_RADIUS: i16 = 4;
    const FIELD_WIDTH: u32 = (TILE_SIZE_1 as u32 + TILE_SEP_1 as u32) * FIELD_SIZE as u32 + 2 * FIELD_SHIFT as u32;
    const BASKET_WIDTH: u32 = (TILE_SIZE_2 as u32 + TILE_SEP_2 as u32) * BASKET_SIZE as u32 + FIELD_SHIFT as u32;
    const FONT_SIZE: u16 = 18;
    const FONT_HEIGHT: i16 = FONT_SIZE as i16 + 2;
    // TODO: check field size
    const W_WIDTH: u32 = FIELD_WIDTH + BASKET_WIDTH;
    const W_HEIGHT: u32 = FIELD_WIDTH;
    const FPS: u32 = 60;

    // game params
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
    let field_bg_color = Color::RGB(170, 170, 170);
    let font_color = Color::RGB(200, 200, 200);
    let field_pos = coord!(FIELD_SHIFT, FIELD_SHIFT);
    let basket_pos = coord!(370, 69);
    let basket_shift = coord!(0, 100);
    let text_pos = coord!(FIELD_WIDTH as i16, FIELD_SHIFT - 5);
    let score_pos = text_pos + coord!(0, FONT_HEIGHT);
    let highscore_pos = score_pos + coord!(0, FONT_HEIGHT);
    let mut mouse_last_pos = coord!(0, 0);
    let mut highscore: u32 = load_highscore(HIGHSCORE_FILE);
    let mut score: u32 = 0;

    // SDL2
    let sdl_context = sdl2::init().expect("Can't init sdl2 context");
    let video_subsystem = sdl_context.video().expect("Can't create video subsystem");
    let window =
        video_subsystem.window("1010", W_WIDTH, W_HEIGHT).position_centered().build().expect("Can't create window");
    let mut canvas = window.into_canvas().build().expect("Can't get canvas");
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).expect("Can't create ttf context");
    let font = ttf_context.load_font("./resource/FiraMono-Regular.ttf", FONT_SIZE).expect("Can't load font");

    // game objects
    let mut current_figure: Option<game::Figure> = None;
    let mut field = game::Field::init_square(FIELD_SIZE, TILE_SIZE_1, TILE_SEP_1, field_pos);
    let mut basket =
        game::BasketSystem::new(BASKET_COUNT, BASKET_SIZE, TILE_SIZE_2, TILE_SEP_2, basket_pos, basket_shift);
    // fill basket by random figures
    basket.fill(&figures);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // render
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();
        render::render_font(&mut canvas, &font, text_pos, font_color, "score").expect("Can't render font");
        render::render_font(&mut canvas, &font, score_pos, font_color, &format!("{:08}", score))
            .expect("Can't render font");
        render::render_font(&mut canvas, &font, highscore_pos, font_color, &format!("{:08}", highscore))
            .expect("Can't render font");
        field.render(&mut canvas, field_bg_color, ROUND_RADIUS).expect("Can't draw field");
        basket.render(&mut canvas, field_bg_color, ROUND_RADIUS).expect("Can't draw basket");
        if let Some(figure) = &current_figure {
            figure
                .render(
                    &mut canvas,
                    mouse_last_pos,
                    coord!(TILE_SIZE_1 as i16, TILE_SIZE_1 as i16),
                    coord!(TILE_SEP_1 as i16, TILE_SEP_1 as i16),
                    ROUND_RADIUS,
                )
                .expect("Can't draw figure");
        }
        canvas.present();

        // events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    mouse_last_pos.x = x as i16 - TILE_SIZE_2 as i16;
                    mouse_last_pos.y = y as i16 - TILE_SIZE_2 as i16;
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    current_figure = match current_figure {
                        Some(ref figure) => {
                            if !field.set_figure(x, y, &figure) {
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

        // game state
        if let Some(lines) = field.next_state() {
            score += (lines.x + lines.y + lines.x * lines.y) * LINE_MULTIPLIER;
        }
        if current_figure == None {
            basket.check_and_refill(&figures);
        }

        // update highscore
        highscore = highscore.max(score);

        // sleep
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    save_highscore(HIGHSCORE_FILE, highscore);
}
