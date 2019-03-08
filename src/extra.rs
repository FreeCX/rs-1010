use std::ops::{Add, Mul, Sub};
use std::time::{Duration, SystemTimeError};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i16,
    pub y: i16,
}

#[macro_export]
macro_rules! figure {
    ($c:expr; $( ($x:expr, $y:expr) ),*) => {
        {
            let mut slice = Vec::new();
            $(
                slice.push(coord!($x, $y));
            )*
            game::Figure::from_slice(&slice, $c)
        }
    };
}

#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr) => {
        crate::extra::Coord { x: $x, y: $y }
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
                panic!(err);
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

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self::Output) -> Self::Output {
        coord!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Add<T> for Coord
where
    T: Into<i16>,
{
    type Output = Self;

    fn add(self, k: T) -> Self::Output {
        let v = k.into();
        coord!(self.x + v, self.y + v)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self::Output) -> Self::Output {
        coord!(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Sub<T> for Coord
where
    T: Into<i16>,
{
    type Output = Self;

    fn sub(self, k: T) -> Self::Output {
        let v = k.into();
        coord!(self.x - v, self.y - v)
    }
}

impl Mul for Coord {
    type Output = Self;

    fn mul(self, k: Self) -> Self::Output {
        coord!(self.x * k.x, self.y * k.y)
    }
}

impl<T> Mul<T> for Coord
where
    T: Into<i16>,
{
    type Output = Self;

    fn mul(self, k: T) -> Self::Output {
        let v = k.into();
        coord!(self.x * v, self.y * v)
    }
}
