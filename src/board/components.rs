use std::collections::HashMap;

use bevy::prelude::{Component, Entity, Resource};

use crate::point::Point;



#[derive(Component)]
pub struct Position {
    pub p: Point
}

#[derive(Component)]
pub struct Tile;

#[derive(Default, Resource)]
pub struct Board {
    pub tiles: HashMap<Point, Entity>,
}
