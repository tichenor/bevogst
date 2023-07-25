use std::collections::HashMap;

use bevy::prelude::*;

use crate::point::Point;

use super::{CurrentBoard, components::{Position, Tile}};



pub fn spawn_board(
    mut commands: Commands,
    mut current: ResMut<CurrentBoard>,
) {
    current.tiles = HashMap::new();
    for x in 0..8 {
        for y in 0..8 {
            let p = Point::new(x, y);
            let tile = commands.spawn((
                Position { p },
                Tile,
            )).id();
            current.tiles.insert(p, tile);
        }
    }
}
