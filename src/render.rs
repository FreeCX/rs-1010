use crate::extra::{Coord, RectData, RectPart};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureQuery};
use sdl2::surface::Surface;
use sdl2::ttf::Font;
use sdl2::video::Window;

type SDL2Result = Result<(), String>;

pub fn build_rounded_rect(c1: Coord, c2: Coord, r: i16) -> RectData {
    // because 8 * r memory is too big
    let approx_memory = (5.7 * r as f32).round() as usize + 7;
    // prepare points for polygon
    let mut v: Vec<(i16, i16)> = Vec::with_capacity(approx_memory);
    let (mut x, mut y) = (r - 1, 0);
    let (mut dx, mut dy) = (1, 1);
    let mut err = dx - (r << 1);
    while x >= y {
        v.push((c2.x + x - r, c1.y + r - y)); // 1
        v.push((c2.x + y - r, c1.y + r - x)); // 2
        v.push((c1.x + r - y, c1.y + r - x)); // 3
        v.push((c1.x + r - x, c1.y + r - y)); // 4
        v.push((c1.x + r - x, c2.y + y - r)); // 5
        v.push((c1.x + r - y, c2.y + x - r)); // 6
        v.push((c2.x + y - r, c2.y + x - r)); // 7
        v.push((c2.x + x - r, c2.y + y - r)); // 8
        if err <= 0 {
            y += 1;
            err += dy;
            dy += 2;
        }
        if err > 0 {
            x -= 1;
            dx += 2;
            err += dx - (r << 1);
        }
    }

    // --- reorder data for drawing ---

    v.sort_by(|a, b| a.1.cmp(&b.1));
    // min and max of x and y
    let (min_y, max_y) = (v[0].1, v[v.len() - 1].1);
    let (mut min_x, mut max_x) = (0, 0);
    // allocated buffer for Rect's: (top + bottom) * r + center
    let mut rects = Vec::with_capacity(2 * r as usize + 1);
    // rectangle in center of rounded rect
    let mut rect = (coord!(), coord!());
    let mut part = RectPart::Top;

    for y in min_y..=max_y {
        let mut iterator = v.iter().filter(|x| x.1 == y);
        if let Some(start) = iterator.next() {
            min_x = start.0;
            max_x = start.0;
            for item in iterator {
                min_x = min_x.min(item.0);
                max_x = max_x.max(item.0);
            }

            rects.push(Rect::new(min_x as i32, y as i32, (max_x - min_x) as u32, 0));

            // find bottom part of rectangle
            if part == RectPart::Bottom {
                rect.1.y = y - rect.0.y;
                part = RectPart::Top;
            }
        } else if part == RectPart::Top {
            // top part founded!
            rect.0.x = min_x;
            rect.0.y = y;
            rect.1.x = max_x - min_x;
            part = RectPart::Bottom;
        }
    }

    rects.push(Rect::new(rect.0.x as i32, rect.0.y as i32, rect.1.x as u32, rect.1.y as u32));
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
