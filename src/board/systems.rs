use bevy::prelude::*;

use crate::{GameSeed, mapgen::{self, BuildData}};

use super::Board;

pub fn spawn_board(
    mut board: ResMut<Board>,
    mut build_data: ResMut<BuildData>,
    game_seed: Res<GameSeed>,
) {
    *build_data = mapgen::random_builder(game_seed.0, 0);
    debug!("build data resource initialized");
    *board = build_data.board.clone();
}
