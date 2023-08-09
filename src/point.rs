use std::ops::{Add, AddAssign, Sub, SubAssign, Div, Mul};



#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, Debug, Ord, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const NORTH: Point = Point { x: 0, y: 1 };
    pub const SOUTH: Point = Point { x: 0, y: -1 };
    pub const EAST: Point = Point { x: 1, y: 0 };
    pub const WEST: Point = Point { x: -1, y: 0 };
    pub const NORTH_EAST: Point = Point { x: 1, y: 1 };
    pub const NORTH_WEST: Point = Point { x: -1, y: 1 };
    pub const SOUTH_EAST: Point = Point { x: 1, y: -1 };
    pub const SOUTH_WEST: Point = Point { x: -1, y: -1 };

    pub const CARDINALS: [Point; 4] = [
        Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST
    ];
    pub const DIAGONALS: [Point; 4] = [
        Self::NORTH_EAST, Self::SOUTH_EAST, Self::SOUTH_WEST, Self::NORTH_WEST
    ];

    pub const OCTANT: [Point; 8] = [
        Self::NORTH, Self::EAST, Self::SOUTH, Self::WEST,
        Self::NORTH_EAST, Self::SOUTH_EAST, Self::SOUTH_WEST, Self::NORTH_WEST
    ];

    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn dist_manhattan(&self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn dist_chebyshev(&self, other: Point) -> i32 {
        i32::max((self.x - other.x).abs(), (self.y - other.y).abs())
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Div<i32> for Point {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Mul<Point> for i32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(rhs.x * self, rhs.y * self)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        (value.0 as i32, value.1 as i32).into()
    }
}
