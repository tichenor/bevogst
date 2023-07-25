use bevy::prelude::*;

use crate::{point::Point, board::{CurrentBoard, components::Position}};
use super::Action;


/// When executed, attempts to move the [Entity] to the specified [Point]. The [Action] 
/// is invalid if there is no [CurrentBoard], the [Point] is not a valid coordinate for
/// the [CurrentBoard], or if the [Entity] does not have a [Position] coordinate.
pub struct MoveToAction(pub Entity, pub Point);

impl Action for MoveToAction {
    fn execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<CurrentBoard>() else { return false };
        if !board.tiles.contains_key(&self.1) { return false };
        
        let Some(mut pos) = world.get_mut::<Position>(self.0) else { return false };
        pos.p = self.1;
        true
    }
}
