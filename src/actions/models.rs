use std::fmt::Debug;

use bevy::prelude::*;

use crate::{point::Point, board::{components::Position, Board}, pieces::components::{TileOccupier, Health}};
use super::Action;


/// When executed, attempts to move the [Entity] to the specified [Point]. The [Action] 
/// is invalid if there is no [CurrentBoard], the [Point] is not a valid coordinate for
/// the [CurrentBoard], or if the [Entity] does not have a [Position] coordinate.
#[derive(Debug)]
pub struct MoveToAction {
    pub entity: Entity,
    pub destination: Point,
}

impl MoveToAction {
    pub fn new(entity: Entity, destination: Point) -> Self {
        Self { entity, destination }
    }
}

impl Action for MoveToAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let Some(board) = world.get_resource::<Board>() else { return Err(()) };
        if !board.in_bounds_xy(self.destination.x, self.destination.y) { return Err(()) };
        
        // If there are any entities at the target destination that already occupy that tile,
        // the action is not possible.
        if world.query_filtered::<&Position, With<TileOccupier>>().iter(world).any(|pos| pos.p == self.destination) {
            return Err(());
        }
        let Some(mut pos) = world.get_mut::<Position>(self.entity) else { return Err(()) };
        pos.p = self.destination;
        Ok(Vec::new())
    }
}

/// Uses a [Point] for the target of the attack rather than an [Entity]. This makes for
/// a more costly position lookup, but in the long run will give more flexibility.
#[derive(Debug)]
pub struct MeleeAttackAction {
    pub attacker: Entity,
    pub target_pos: Point,
    pub damage: u32,
}

impl Action for MeleeAttackAction {
   fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let Some(attacker_pos) = world.get::<Position>(self.attacker) else { 
            warn!("Position component not found for entity {:?}", self.attacker);
            return Err(()) 
        };

        // Melee means only adjacent tiles are in range
        let dist = attacker_pos.p.dist_chebyshev(self.target_pos);
        if dist > 1 { return Err(()) };

        // Valid targets are any entities with a health component at the target position
        let target_entities = world.query_filtered::<(Entity, &Position), With<Health>>()
            .iter(world)
            .filter(|(_, pos)| pos.p == self.target_pos)
            .collect::<Vec<_>>();
        if target_entities.len() == 0 { 
            warn!("No valid entities to attack at location {:?}", self.target_pos);
            return Err(()); 
        }

        let result = target_entities.iter()
            .map(|e| Box::new(DamageAction::new(e.0, self.damage)) as Box<dyn Action>)
            .collect::<Vec<_>>();

        Ok(result)
    } 
}

#[derive(Debug)]
pub struct DamageAction {
    pub entity: Entity,
    pub value: u32,
}

impl DamageAction {
    pub fn new(entity: Entity, value: u32) -> Self {
        Self { entity, value }
    }
}

impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()> {
        let Some(mut health) = world.get_mut::<Health>(self.entity) else { return Err(()) };
        health.value = health.value.saturating_sub(self.value);
        if health.value == 0 {
            world.despawn(self.entity);
        }
        Ok(Vec::new())
    }
}
