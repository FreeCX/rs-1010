use std::ops::{Add, Mul, Shr, Sub};
use std::time::{Duration, SystemTimeError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

#[macro_export]
macro_rules! figure {
    ($i:expr, $c:expr; $( ($x:expr, $y:expr) ),*) => {
        {
            let mut slice = Vec::new();
            $(
                slice.push(coord!($x, $y));
            )*
            game::Figure::from_slice($i, &slice, $c)
        }
    };
}

#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr) => {
        crate::extra::Coord { x: $x, y: $y }
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

// convert Duration to HH:MM:SS
pub fn as_time_str(duration: &Result<Duration, SystemTimeError>) -> String {
    let secs = duration.clone().unwrap_or(Duration::from_secs(0)).as_secs();
    let (hours, minutes, seconds) = (secs / (60 * 60), (secs / 60) % 60, secs % 60);
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
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
