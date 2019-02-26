use std::ops::{ Add, Sub, Mul };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self::Output) -> Self::Output {
        coord!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self::Output) -> Self::Output {
        coord!(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i16> for Coord {
    type Output = Self;

    fn mul(self, k: i16) -> Self::Output {
        coord!(self.x * k, self.y * k)
    }
}