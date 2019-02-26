use crate::extra::Coord;
use palette::{LinSrgb, Shade};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

fn light_up(c: Color, l: f32) -> Color {
    let color = LinSrgb::new(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0).lighten(l);
    let to_u8 = |x: f32| (x * 255.0).round() as u8;
    Color::RGB(to_u8(color.red), to_u8(color.green), to_u8(color.blue))
}

pub fn fill_rounded_rect(canvas: &mut Canvas<Window>, c1: Coord, c2: Coord, r: i16, c: Color) -> Result<(), String> {
    let mut tmp: Vec<Vec<(i16, i16)>> = vec![Vec::new(); 8];
    let (mut x, mut y) = (r - 1, 0);
    let (mut dx, mut dy) = (1, 1);
    let mut err = dx - (r << 1);
    while x >= y {
        tmp[0].push((x - r, y - r)); // 8
        tmp[1].push((y - r, x - r)); // 7
        tmp[2].push((r - y, x - r)); // 6
        tmp[3].push((r - x, y - r)); // 5
        tmp[4].push((r - x, r - y)); // 4
        tmp[5].push((r - y, r - x)); // 3
        tmp[6].push((y - r, r - x)); // 2
        tmp[7].push((x - r, r - y)); // 1
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
    let data = [(c2.x, c2.y), (c1.x, c2.y), (c1.x, c1.y), (c2.x, c1.y)];
    let (mut vx, mut vy) = (Vec::new(), Vec::new());
    for (index, (x, y)) in data.iter().enumerate() {
        for (tx, ty) in &tmp[index * 2 + 0] {
            vx.push(x + tx);
            vy.push(y + ty);
        }
        for (tx, ty) in tmp[index * 2 + 1].iter().rev() {
            vx.push(x + tx);
            vy.push(y + ty);
        }
    }
    canvas.filled_polygon(&vx, &vy, c)?;
    let nc = light_up(c, 0.02);
    for (x, y) in vx.into_iter().zip(vy) {
        canvas.pixel(x, y, nc)?;
    }
    Ok(())
}
