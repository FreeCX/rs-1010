use crate::extra::{Coord, RectData};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::Window;

type SDL2Result = Result<(), String>;

fn s_ellipse(a: f32, b: f32, n: f32, m: f32, t: f32) -> (f32, f32) {
    let x = a + t.cos().abs().powf(2.0 / n) * a.copysign(t.cos());
    let y = b + t.sin().abs().powf(2.0 / m) * b.copysign(t.sin());
    (x, y)
}

pub fn build_rounded_rect(c1: Coord, c2: Coord, r: i16) -> RectData {
    // --- render super ellipse ---
    let steps: i32 = 20;
    let a = 0.5 * (c2.x - c1.x) as f32;
    let b = 0.5 * (c2.y - c1.y) as f32;
    let r = r as f32;
    let mut v: Vec<(i16, i16)> = (0_i32..steps)
        .filter(|dt| dt % 5 != 0)
        .map(|dt| {
            let t = dt as f32 * std::f32::consts::TAU / steps as f32;
            let (x, y) = s_ellipse(a, b, r, r, t);
            (x.round() as i16, y.round() as i16)
        })
        .collect();

    // --- reorder data  ---
    v.sort_by(|a, b| a.1.cmp(&b.1));
    // min and max of x and y
    let (min_y, max_y) = (v[0].1, v[v.len() - 1].1);
    let (mut min_x, mut max_x) = (0, 0);
    let mut rects = vec![];

    for y in min_y..=max_y {
        let mut iterator = v.iter().filter(|item| item.1 == y);
        if let Some(start) = iterator.next() {
            min_x = start.0;
            max_x = start.0;
            for item in iterator {
                min_x = min_x.min(item.0);
                max_x = max_x.max(item.0);
            }
            rects.push(Rect::new(min_x as i32, y as i32, (max_x - min_x) as u32, 0));
        } else {
            let mut curr = Rect::new(min_x as i32, y as i32, (max_x - min_x) as u32, 0);
            if let Some(last) = rects.pop() {
                if last.w == curr.w {
                    curr.y = last.y;
                    curr.h = last.h + 1;
                }
            }
            rects.push(curr);
        }
    }

    RectData::new(rects)
}

pub fn fill_rounded_rect_new(canvas: &mut Canvas<Window>, c1: Coord, c2: Coord, r: i16, c: Color) -> SDL2Result {
    // --- build rounded rect ---
    let data = build_rounded_rect(c1, c2, r);
    // --- draw ---
    fill_rounded_rect_from(canvas, &data, c)
}

pub fn fill_rounded_rect_from(canvas: &mut Canvas<Window>, data: &RectData, c: Color) -> SDL2Result {
    let last_color = canvas.draw_color();
    canvas.set_draw_color(c);
    canvas.fill_rects(data.data())?;
    canvas.set_draw_color(last_color);
    Ok(())
}

pub fn font(surface: &mut Surface, font: &Font, pos: Coord, fg: Color, bg: Color, text: &str) -> SDL2Result {
    let font_size = font.size_of(text).map_err(|e| e.to_string())?;
    let font_surface = font.render(text).blended(fg).map_err(|e| e.to_string())?;
    let dst_rect = Rect::new(pos.x as i32, pos.y as i32, font_size.0, font_size.1);
    surface.fill_rect(dst_rect, bg)?;
    font_surface.blit(None, surface, dst_rect)?;
    Ok(())
}

pub fn surface_copy(canvas: &mut Canvas<Window>, surface: &Surface) -> SDL2Result {
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(surface).map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(0, 0, width, height);
    canvas.copy(&texture, None, target)?;
    Ok(())
}
