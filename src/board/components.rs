use bevy::prelude::{Component, Entity, Resource};

use crate::point::Point;



#[derive(Component, Clone)]
pub struct Position {
    pub p: Point
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
}

