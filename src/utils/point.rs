use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

pub const ORIGIN: Point = Point::new(0, 0);
pub const UP: Point = Point::new(0, -1);
pub const DOWN: Point = Point::new(0, 1);
pub const LEFT: Point = Point::new(-1, 0);
pub const RIGHT: Point = Point::new(1, 0);
pub const UP_LEFT: Point = Point::new(-1, -1);
pub const UP_RIGHT: Point = Point::new(1, -1);
pub const DOWN_LEFT: Point = Point::new(-1, 1);
pub const DOWN_RIGHT: Point = Point::new(1, 1);
pub const ORTHOGONAL: [Point; 4] = [UP, DOWN, LEFT, RIGHT];
pub const DIAGONAL: [Point; 8] = [
    UP, LEFT, RIGHT, DOWN, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
];

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    /// Return the clockwise rotation of the point. Intended for use with cardinal directions.
    #[inline]
    #[must_use]
    pub fn clockwise(self) -> Self {
        Point::new(-self.y, self.x)
    }

    /// Return the counter-clockwise rotation of the point. Intended for use with cardinal directions.
    #[inline]
    #[must_use]
    pub fn counter_clockwise(self) -> Self {
        Point::new(self.y, -self.x)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn mul(self, rhs: i32) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    #[must_use]
    fn sub(self, rhs: Self) -> Self {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.x as u32);
        state.write_u32(self.y as u32);
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
