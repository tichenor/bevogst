use serde::{Serialize, Deserialize};

use crate::point::Point;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Rect {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

impl Rect {
    
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        assert!(w > 0);
        assert!(h > 0);
        Rect {
            x1: x,
            y1: y,
            x2: x + w - 1,
            y2: y + h - 1,
        }
    }

    pub fn width(&self) -> u32 {
        u32::abs_diff(self.x1, self.x2) + 1
    }

    pub fn height(&self) -> u32 {
        u32::abs_diff(self.y1, self.y2) + 1
    }

    /// Return true if `other` plus `margin` overlaps with this [Rect].
    pub fn intersects(&self, other: &Rect, margin: u32) -> bool {
        other.x2 + margin >= self.x1
            && other.x1 - margin <= self.x2
            && other.y2 + margin >= self.y1
            && other.y1 - margin <= self.y2
    }

    pub fn center(&self) -> Point {
        Point::new(
            ((self.x2 + self.x1) / 2) as i32,
            ((self.y2 + self.y1) / 2) as i32,
        )
    }

    pub fn iter_xy(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        (self.y1..=self.y2)
            .flat_map(move |y| std::iter::repeat(y).zip(self.x1..=self.x2))
            .map(move |(y, x)| (x, y))
    }

}
