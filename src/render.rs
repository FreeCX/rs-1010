use crate::extra::Coord;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

// very simple & unstable polygon drawing
pub fn polygon(canvas: &mut Canvas<Window>, v: &mut [(i16, i16)], c: Color) -> Result<(), String> {
    v.sort_by(|a, b| a.1.cmp(&b.1));
    let (min_y, max_y) = (v[0].1, v[v.len() - 1].1);
    let (mut min_x, mut max_x) = (0, 0);
    let mut lines = Vec::new();
    for y in min_y..=max_y {
        let mut c: Vec<_> = v.iter().filter(|x| x.1 == y).collect();
        if c.len() > 0 {
            c.sort_by(|a, b| a.0.cmp(&b.0));
            min_x = c[0].0;
            max_x = c[c.len() - 1].0;
            for (a, b) in c.iter().zip(c.iter().skip(1)) {
                lines.push(Point::new(a.0 as i32, y as i32));
                lines.push(Point::new(b.0 as i32, y as i32));
            }
        } else {
            lines.push(Point::new(min_x as i32, y as i32));
            lines.push(Point::new(max_x as i32, y as i32));
        }
    }
    let last_color = canvas.draw_color();
    canvas.set_draw_color(c);
    for index in (0..lines.len() - 1).step_by(2) {
        canvas.draw_line(lines[index + 0], lines[index + 1])?;
    }
    canvas.set_draw_color(last_color);
    Ok(())
}

// TODO: optimize code for using canvas.draw_lines in polygon
pub fn fill_rounded_rect(canvas: &mut Canvas<Window>, c1: Coord, c2: Coord, r: i16, c: Color) -> Result<(), String> {
    // prepare points for polygon
    let mut tmp: Vec<(i16, i16)> = Vec::new();
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
    polygon(canvas, &mut tmp, c)?;
    Ok(())
}
