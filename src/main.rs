extern crate palette;
extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use std::time::Duration;

#[macro_use]
mod extra;
mod game;
mod random;
mod render;

pub fn main() {
    const BASKET_COUNT: u8 = 3;
    const BASKET_SIZE: u8 = 5;
    const FIELD_SIZE: u8 = 10;
    const TILE_SIZE_1: u8 = 32;
    const TILE_SIZE_2: u8 = TILE_SIZE_1 / 2;
    const TILE_SEP_1: u8 = 3;
    const TILE_SEP_2: u8 = 2;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("1010", 500, 500).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let figures = vec![
        figure!(Color::RGB(210, 230, 100); (0, 0), (0, 1), (1, 0), (1, 1)),
        figure!(Color::RGB(230, 100, 100); (0, 0), (0, 1), (1, 1)),
        figure!(Color::RGB(230, 210, 100); (0, 0), (0, 1)),
    ];
    let mut mouse_last_pos = coord!(0, 0);
    let mut current_figure: Option<game::Figure> = None;
    let mut field = game::Field::init_square(FIELD_SIZE, TILE_SIZE_1, TILE_SEP_1, coord!(10, 10));
    let mut basket =
        game::BasketSystem::new(BASKET_COUNT, BASKET_SIZE, TILE_SIZE_2, TILE_SEP_2, coord!(370, 10), coord!(0, 100));
    basket.fill(&figures);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // render
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();
        field.render(&mut canvas, Color::RGB(170, 170, 170)).expect("Can't draw field");
        basket.render(&mut canvas, Color::RGB(170, 170, 170)).expect("Can't draw basket");
        if let Some(figure) = &current_figure {
            figure
                .render(&mut canvas, mouse_last_pos, coord!(TILE_SIZE_1 as i16, TILE_SIZE_1 as i16), coord!(3, 3))
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
        field.next_state();
        if current_figure == None {
            basket.check_and_refill(&figures);
        }

        // The rest of the game loop goes here...
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
