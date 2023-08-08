use bevy::prelude::*;

use crate::actions::Action;

#[derive(Component)]
pub struct Piece {
    pub kind: String,
}

/// Component that holds an [Entity]s [Action]s it wants to perform. 
/// Apart from being a marker, this component can hold any number of 
/// actions planned for the entity. Each action will be given a score
/// to determine what action gets performed each turn.
#[derive(Component, Default)]
pub struct Actor(pub Vec<(Box<dyn Action>, i32)>);

#[derive(Component)]
pub struct Health {
    pub value: u32,
}

#[derive(Component)]
pub struct Fighter {
    pub strength: u32,
}

/// Marker component indicating whether a [Tile][crate::board::components::Tile]
/// is occupied whenever an [Entity] is at the same 
/// [Position][crate::board::components::Position] as that tile. There can only 
/// be a single occupier on a given tile.
#[derive(Component)]
pub struct TileOccupier;

/// Movement behaviour for non-player pieces
#[derive(Component)]
pub struct Walker;
