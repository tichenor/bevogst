use bevy::prelude::*;
use gfx::GraphicsWaitEvent;
use input::PlayerInputReadyEvent;

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
                gfx::GraphicsPlugin,
                player::PlayerPlugin,
                input::InputPlugin,
                manager::ManagerPlugin,
            )
        )
        .add_systems(Startup, camera::setup)
        .run()
}
