use std::ops::{Add, Mul, Shr, Sub};

use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use tini::Ini;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlendColor {
    pub main: Color,
    pub blend: Option<Color>,
}

#[derive(Clone)]
pub struct RectData(Vec<Rect>, Vec<Point>);

#[macro_export]
macro_rules! figure {
    ($i:expr, $c:expr; $( ($x:expr, $y:expr) ),*) => {
        {
            let slice = vec![$( coord!($x, $y), )* ];
            game::Figure::from_slice($i, &slice, $c)
        }
    };
}

#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr) => {
        $crate::extra::Coord { x: $x, y: $y }
    };
    ($xy:expr) => {
        coord!($xy, $xy)
    };
    () => {
        coord!(0, 0)
    };
}

#[macro_export]
macro_rules! normalize {
    ($param:expr; $lower:expr, $upper:expr) => {
        if $param < $lower {
            $param = $lower;
        }
        if $param > $upper {
            $param = $upper;
        }
    };
}

#[macro_export]
macro_rules! msg {
    ($data:expr; $wnd:expr, $title:expr) => {
        match $data {
            Ok(value) => value,
            Err(err) => {
                // show error dialog box and panic
                sdl2::messagebox::show_simple_message_box(
                    sdl2::messagebox::MessageBoxFlag::ERROR,
                    $title,
                    &format!("{}", err),
                    $wnd,
                )
                .unwrap_or(());
                panic!("{}", err);
            }
        }
    };
}

impl Coord {
    pub fn floor_frac(self, rhs: Coord) -> Self {
        let xi = (self.x as f32 / rhs.x as f32).floor() as i16;
        let yi = (self.y as f32 / rhs.y as f32).floor() as i16;
        coord!(xi, yi)
    }

    pub fn normalize(mut self, lower: Coord, upper: Coord) -> Self {
        normalize!(self.x; lower.x, upper.x);
        normalize!(self.y; lower.y, upper.y);
        self
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(mut self, rhs: Self::Output) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}

impl<T> Add<T> for Coord
where
    T: Into<i16>,
{
    type Output = Self;

    fn add(mut self, k: T) -> Self::Output {
        let v = k.into();
        self.x += v;
        self.y += v;
        self
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(mut self, rhs: Self::Output) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl<T> Sub<T> for Coord
where
    T: Into<i16>,
{
    type Output = Self;

    fn sub(mut self, k: T) -> Self::Output {
        let v = k.into();
        self.x -= v;
        self.y -= v;
        self
    }
}

impl Mul for Coord {
    type Output = Self;

    fn mul(mut self, k: Self) -> Self::Output {
        self.x *= k.x;
        self.y *= k.y;
        self
    }
}

impl<T> Mul<T> for Coord
where
    T: Into<i16>,
{
    type Output = Self;

    fn mul(mut self, k: T) -> Self::Output {
        let v = k.into();
        self.x *= v;
        self.y *= v;
        self
    }
}

impl<T> Shr<T> for Coord
where
    T: Into<i16>,
{
    type Output = Self;

    fn shr(mut self, k: T) -> Self::Output {
        let v = k.into();
        self.x >>= v;
        self.y >>= v;
        self
    }
}

impl RectData {
    pub fn new(lines: Vec<Rect>, points: Vec<Point>) -> Self {
        RectData(lines, points)
    }

    pub fn shift(&self, size: Coord) -> Self {
        let mut shifted = self.clone();

        // shift lines
        for point in shifted.0.iter_mut() {
            point.x += size.x as i32;
            point.y += size.y as i32;
        }
        // shift points
        for point in shifted.1.iter_mut() {
            point.x += size.x as i32;
            point.y += size.y as i32;
        }

        shifted
    }

    pub fn rects(&self) -> &'_ Vec<Rect> {
        &self.0
    }

    pub fn points(&self) -> &'_ Vec<Point> {
        &self.1
    }
}

impl From<Color> for BlendColor {
    fn from(color: Color) -> Self {
        BlendColor { main: color, blend: None }
    }
}

impl BlendColor {
    pub fn blend(main: Color, bg: Color) -> Self {
        let r = main.r / 2 + bg.r / 2;
        let g = main.g / 2 + bg.g / 2;
        let b = main.b / 2 + bg.b / 2;
        BlendColor { main, blend: Some(Color::RGB(r, g, b)) }
    }
}

pub fn fake_contrast(a: Color, k: f32) -> Color {
    let r = ((a.r as u16 + (255.0 * k).round() as u16) >> 2) as u8;
    let g = ((a.g as u16 + (255.0 * k).round() as u16) >> 2) as u8;
    let b = ((a.b as u16 + (255.0 * k).round() as u16) >> 2) as u8;
    Color::RGBA(r, g, b, a.a)
}

pub fn v_as_color(config: &Ini, section: &str, param: &str, default: &[u8; 3]) -> Color {
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
