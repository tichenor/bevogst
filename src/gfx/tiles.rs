use bevy::prelude::*;

use crate::board::{components::{Position, Tile}, Board};

use super::{GraphicsAssets, TILE_SIZE, TILE_Z};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    board: Res<Board>,
    assets: Res<GraphicsAssets>,
) {
    for y in 0..board.height {
        for x in 0..board.width {
            let tile = board.get_tile_xy(x, y);

            let idx = match tile {
                Tile::Floor => 177,
                Tile::Wall => 219,
            };
            let mut sprite = TextureAtlasSprite::new(idx);
            sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
            sprite.color = Color::OLIVE;
            let v = super::get_world_position(&Position { p: (x, y).into() }, TILE_Z);

            commands.spawn_empty()
                .insert(
                    SpriteSheetBundle {
                        sprite,
                        texture_atlas: assets.sprite_texture.clone(),
                        transform: Transform::from_translation(v),
                        ..Default::default()
                    }
                );
        }
    }
}
