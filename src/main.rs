use bevy::prelude::*;

mod config;
mod state;
mod assets;
mod point;
mod board;
mod gfx;
mod camera;
mod pieces;
mod player;
mod input;
mod actions;
mod manager;
pub mod pathfind;
mod mapgen;
mod random;
mod bitgrid;
mod rect;
mod saveload;

#[derive(Resource)]
pub struct GameSeed(u64);

impl Default for GameSeed {
    fn default() -> Self {
        let game_seed: u64 = std::env::args()
            .nth(1)
            .and_then(|arg| arg.as_str().parse().ok())
            .unwrap_or_else(rand::random);
        Self(game_seed)
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (
                            config::WINDOW_WIDTH,
                            config::WINDOW_HEIGHT,
                        ).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ).set(
                ImagePlugin::default_nearest()
            )
        )
        .insert_resource(Msaa::Off)
        .add_state::<state::MainState>()
        .add_state::<state::GameState>()
        .add_plugins(
            (
                actions::ActionsPlugin,
                assets::AssetPlugin,
                board::BoardPlugin,
                pieces::PiecesPlugin,
                gfx::GraphicsPlugin,
                player::PlayerPlugin,
                input::InputPlugin,
                manager::ManagerPlugin,
                mapgen::MapGenPlugin,
                camera::CameraPlugin,
            )
        )
        .init_resource::<GameSeed>()
        .run()
}
