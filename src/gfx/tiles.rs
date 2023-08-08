use bevy::prelude::*;

use crate::board::components::{Position, Tile};

use super::{GraphicsAssets, TILE_SIZE, TILE_Z};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Tile), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, pos, tile) in query.iter() {
        let idx = match tile {
            Tile::Floor => 177,
            Tile::Wall => 219,
        };
        let mut sprite = TextureAtlasSprite::new(idx);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::OLIVE;
        let v = super::get_world_position(&pos, TILE_Z);
        commands.entity(entity)
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
