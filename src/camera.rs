use bevy::prelude::*;

use crate::{gfx::TILE_SIZE, board::components::Position, player::Player, state::MainState, mapgen::{MapGenSet, BuildData}};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), setup.in_set(MapGenSet::Spawning));
    }
}

pub fn setup(
    mut commands: Commands,
    build_data: Res<BuildData>,
) {
    debug!("requesting build_data starting position...");
    let pos = build_data.starting_position.unwrap();
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(
        pos.x as f32 * TILE_SIZE,
        pos.y as f32 * TILE_SIZE,
        camera.transform.translation.z,
    );
    commands.spawn(camera);
}
