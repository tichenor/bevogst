use bevy::prelude::*;

use crate::{board::components::Position, point::Point, state::MainState};

use self::components::{Actor, Piece, Walker, Fighter, TileOccupier};

pub mod components;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_npcs);
    }
}

pub fn spawn_npcs(mut commands: Commands) {
    commands.spawn((
        Actor::default(),
        Piece { kind: "NPC".to_string() },
        Position { p: Point::new(3, 5) },
        Walker,
        Fighter { strength: 2 },
        TileOccupier {},
    ));
    commands.spawn((
        Actor::default(),
        Piece { kind: "NPC".to_string() },
        Position { p: Point::new(5, 5) },
        Walker,
        Fighter { strength: 3 },
        TileOccupier {},
    ));
}


