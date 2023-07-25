use bevy::prelude::*;

use crate::{pieces::components::Piece, board::components::Position};

use super::{TILE_SIZE, PIECE_Z, GraphicsAssets, POSITION_TOLERANCE, PIECE_SPEED};

pub fn spawn_piece_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Piece), Added<Piece>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, pos, piece) in query.iter() {
        let sprite_idx = match piece.kind.as_str() {
            "Player" => 1,
            _ => 63,
        };
        let mut sprite = TextureAtlasSprite::new(sprite_idx);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::WHITE;
        let v = super::get_world_position(&pos, PIECE_Z);
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

pub fn update_piece_position(
    mut query: Query<(&Position, &mut Transform), With<Piece>>,
    time: Res<Time>,
    mut ev_wait: EventWriter<super::GraphicsWaitEvent>,
) {
    let mut animating = false;
    for (pos, mut transf) in query.iter_mut() {
        let target = super::get_world_position(&pos, PIECE_Z);
        let d = (target - transf.translation).length();
        if d > POSITION_TOLERANCE {
            transf.translation = transf.translation.lerp(
                target, 
                PIECE_SPEED * time.delta_seconds(),
            );
            animating = true;
        } else {
            transf.translation = target;
        }
    }

    if animating {
        ev_wait.send(super::GraphicsWaitEvent);
    }
}
