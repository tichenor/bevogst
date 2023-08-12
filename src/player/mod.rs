use bevy::prelude::*;

use crate::{state::MainState, pieces::components::{Piece, Actor, Health, TileOccupier, Fighter}, board::components::Position, mapgen::{MapGenSet, BuildData}, camera};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MainState::Game), 
            spawn_player.in_set(MapGenSet::Spawning)
        );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    build_data: Res<BuildData>,
) {
    commands.spawn(
        (
            Actor::default(),
            Player,
            Piece { kind: "Player".to_string() },
            // starting position should always be generated when the spawn_player system is run
            Position { p: build_data.starting_position.unwrap() },             
            Health { value: 10 },
            TileOccupier {},
            Fighter { strength: 5 },
        )
    );
}
