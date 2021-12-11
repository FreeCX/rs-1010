use crate::extra::{Coord, RectData, RectPart};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::Window;

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
    // allocated buffer for SDL2 lines:
    // 2 (points) * 2 (top and bottom part) * r
    let mut lines = Vec::with_capacity(2 * 2 * r as usize);
    // rectangle in center of polygon
    let mut rect = Rect::new(0, 0, 0, 0);
    let mut part = RectPart::Top;
    let mut is_odd = true;

    for y in min_y..=max_y {
        let mut iterator = v.iter().filter(|x| x.1 == y);
        if let Some(start) = iterator.next() {
            min_x = start.0;
            max_x = start.0;
            for item in iterator {
                min_x = min_x.min(item.0);
                max_x = max_x.max(item.0);
            }
            if is_odd {
                lines.push(Point::new(min_x as i32, y as i32));
                lines.push(Point::new(max_x as i32, y as i32));
            } else {
                lines.push(Point::new(max_x as i32, y as i32));
                lines.push(Point::new(min_x as i32, y as i32));
            }
            is_odd = !is_odd;

            // find bottom part of rectangle
            if part == RectPart::Bottom {
                rect.set_height((y - rect.y as i16) as u32);
                part = RectPart::Top;
            }
        } else if part == RectPart::Top {
            // top part founded!
            rect.x = min_x as i32;
            rect.y = y as i32;
            rect.set_width((max_x - min_x) as u32 + 1);
            part = RectPart::Bottom;
        }
    }

    RectData::new(lines, rect)
}

pub fn fill_rounded_rect_new(
    canvas: &mut Canvas<Window>, c1: Coord, c2: Coord, r: i16, c: Color,
) -> Result<(), String> {
    // --- build rounded rect ---
    let data = build_rounded_rect(c1, c2, r);

    // --- draw ---
    fill_rounded_rect_from(canvas, &data, c)
}

pub fn fill_rounded_rect_from(canvas: &mut Canvas<Window>, data: &RectData, c: Color) -> Result<(), String> {
    let last_color = canvas.draw_color();
    canvas.set_draw_color(c);
    canvas.draw_lines(data.lines.as_slice())?;
    canvas.fill_rect(data.rect)?;
    canvas.set_draw_color(last_color);

    Ok(())
}

pub fn font(canvas: &mut Canvas<Window>, font: &Font, pos: Coord, c: Color, text: &str) -> Result<(), String> {
    let texture_creator = canvas.texture_creator();
    let surface = font.render(text).blended(c).map_err(|e| e.to_string())?;
    let texture = texture_creator.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(pos.x as i32, pos.y as i32, width, height);
    canvas.copy(&texture, None, Some(target))?;
    Ok(())
}
