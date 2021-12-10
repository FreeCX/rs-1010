use crate::extra::Coord;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, RenderTarget, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::Window;

// very simple polygon drawing
pub fn unfair_polygon<T>(canvas: &mut Canvas<T>, v: &mut [(i16, i16)], c: Color) -> Result<(), String>
where
    T: RenderTarget,
{
    v.sort_by(|a, b| a.1.cmp(&b.1));
    let (min_y, max_y) = (v[0].1, v[v.len() - 1].1);
    let (mut min_x, mut max_x) = (0, 0);
    let mut lines = Vec::with_capacity(2 * (max_y - min_y + 1) as usize);
    for y in min_y..=max_y {
        let mut iterator = v.iter().filter(|x| x.1 == y);
        if let Some(start) = iterator.next() {
            min_x = start.0;
            max_x = start.0;
            for item in iterator {
                min_x = min_x.min(item.0);
                max_x = max_x.max(item.0);
            }
        }
        lines.push(Point::new(min_x as i32, y as i32));
        lines.push(Point::new(max_x as i32, y as i32));
    }
    let last_color = canvas.draw_color();
    canvas.set_draw_color(c);
    canvas.draw_lines(lines.as_slice())?;
    canvas.set_draw_color(last_color);
    Ok(())
}

pub fn fill_rounded_rect<T>(canvas: &mut Canvas<T>, c1: Coord, c2: Coord, r: i16, c: Color) -> Result<(), String>
where
    T: RenderTarget,
{
    // because 8 * r memory is too big
    let approx_memory = (5.7 * r as f32).round() as usize + 7;
    // prepare points for polygon
    let mut tmp: Vec<(i16, i16)> = Vec::with_capacity(approx_memory);
    let (mut x, mut y) = (r - 1, 0);
    let (mut dx, mut dy) = (1, 1);
    let mut err = dx - (r << 1);
    while x >= y {
        tmp.push((c2.x + x - r, c1.y + r - y)); // 1
        tmp.push((c2.x + y - r, c1.y + r - x)); // 2
        tmp.push((c1.x + r - y, c1.y + r - x)); // 3
        tmp.push((c1.x + r - x, c1.y + r - y)); // 4
        tmp.push((c1.x + r - x, c2.y + y - r)); // 5
        tmp.push((c1.x + r - y, c2.y + x - r)); // 6
        tmp.push((c2.x + y - r, c2.y + x - r)); // 7
        tmp.push((c2.x + x - r, c2.y + y - r)); // 8
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
    // draw
    unfair_polygon(canvas, &mut tmp, c)?;
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
