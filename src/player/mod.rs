use bevy::prelude::*;

use crate::{state::MainState, point::Point, pieces::components::{Piece, Actor, Health, TileOccupier, Fighter}, board::components::Position};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn(
        (
            Actor::default(),
            Player,
            Piece { kind: "Player".to_string() },
            Position { p: Point::new(0, 0) },
            Health { value: 10 },
            TileOccupier {},
            Fighter { strength: 5 },
        )
    );
}
