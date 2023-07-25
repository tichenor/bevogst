use bevy::prelude::*;

use crate::actions::Action;

#[derive(Component)]
pub struct Piece {
    pub kind: String,
}

/// Component that allows an [Entity] to perform [Action]s. Apart
/// from being a marker, this component can hold an entity's action
/// if any is planned.
#[derive(Component, Default)]
pub struct Actor(pub Option<Box<dyn Action>>);
