use std::collections::HashMap;

use bevy::prelude::*;

use crate::point::Point;

use super::{CurrentBoard, components::{Position, Tile}};



pub fn spawn_board(
    mut commands: Commands,
    mut current: ResMut<CurrentBoard>,
) {
    current.tiles = HashMap::new();
    for x in 1..8 {
        for y in 1..8 {
            let p = Point::new(x, y);
            let tile = commands.spawn((
                Position { p },
                Tile::Floor,
            )).id();
            current.tiles.insert(p, tile);
        }
    }

    for i in 0..8 {
        let p1 = Point::new(i, 0);
        let p2 = Point::new(i, 8);
        let q1 = Point::new(0, i);
        let q2 = Point::new(8, i);
        
        for p in [p1, p2, q1, q2] {
            let tile = commands.spawn((
                Position { p },
                Tile::Wall,
            )).id();
            current.tiles.insert(p, tile);
        }
    }
}
