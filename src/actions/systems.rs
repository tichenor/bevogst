use bevy::prelude::*;

use crate::{pieces::components::{Actor, Walker, TileOccupier, Fighter}, player::Player, board::{components::Position, Board}, point::Point, pathfind};

use super::{ActorQueue, models::{MoveToAction, MeleeAttackAction}, InvalidPlayerActionEvent, NextActorEvent, PendingActions};

pub const MOVE_SCORE: i32 = 50;
pub const PLAYER_ATTACK_SCORE: i32 = 100;

/// This system assumes the action queue has already been populated with the 
/// player entity from the player input system.
///
/// Currently any [Entity] with an [Actor] component that is not the [Player]
/// is added to the queue each time this is called (which should be once 
/// whenever we leave the [PlayerInput](crate::state::GameState::PlayerInput) 
/// state---so, once per turn.
pub fn populate_actor_queue(
    query: Query<Entity, (With<Actor>, Without<Player>)>,
    mut queue: ResMut<ActorQueue>,
) {
    queue.0.extend(query.iter());
}

/// Attempts to perform an [Action](crate::actions::Action) of the [Entity] in the
/// front of the [ActorQueue]. On successful execution of the action, this system 
/// emits a [NextActorEvent](super::NextActorEvent) to trigger planning of actions
/// for the next entity in the queue. This is because each action undertaken
/// by an entity may affect the board state in ways that must be taken into account
/// when planning further actions.
pub fn process_action_queue(world: &mut World) {

    if process_pending_actions(world) { return };

    let Some(mut queue) = world.get_resource_mut::<ActorQueue>() else { 
        warn!("Could not find resource: {}", std::any::type_name::<ActorQueue>());
        return; 
    };
    
    let Some(entity) = queue.pop_front() else {
        world.send_event(super::ActionsCompleteEvent);
        return;
    };

    let Some(mut actor) = world.get_mut::<Actor>(entity) else {
        warn!(
            "Entity {:?} found in action queue ({}) but has no Actor component: {}", 
            entity, 
            std::any::type_name::<ActorQueue>(),
            std::any::type_name::<Actor>(),
        );
        return;
    };

    // clear the Vec of actions and sort it with highest score first
    let mut possible_actions = actor.0.drain(..).collect::<Vec<_>>();
    possible_actions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut success = false;
    for action in possible_actions {
        if let Ok(result) = action.0.execute(world) {
            if let Some(mut pending) = world.get_resource_mut::<PendingActions>() {
                pending.0 = result;
            }
            success = true;
            break;
        } else {
            println!("Action {:?} for entity {:?} failed", action, entity);
        }
    }

    if !success && world.get::<Player>(entity).is_some() {
        debug!("Invalid Player action");
        world.send_event(InvalidPlayerActionEvent);
        return;
    }

    world.send_event(NextActorEvent);
}

/// Returns `true` if at least one pending action has been successfully executed.
fn process_pending_actions(world: &mut World) -> bool {
    // Take the pending actions without holding the mutable reference to the world
    let pending = match world.get_resource_mut::<PendingActions>() {
        Some(mut res) => res.0.drain(..).collect::<Vec<_>>(),
        _ => return false,
    };

    let mut next = Vec::new();
    let mut success = false;
    for action in pending {
        if let Ok(result) = action.execute(world) {
            next.extend(result);
            success = true;
        }
    }

    // If there are any new actions, assign them back to the resource
    // Should be safe to call unwrap() since we confirmed the resource at the beginning
    let mut res = world.get_resource_mut::<PendingActions>().unwrap();
    res.0 = next;
    success
}

/// Temporary system that creates a MoveToAction in a random direction.
pub fn plan_walk(
    mut query: Query<(&Position, &mut Actor), With<Walker>>,
    queue: Res<ActorQueue>,
    player_query: Query<&Position, With<Player>>,
    occupier_query: Query<&Position, With<TileOccupier>>,
    board: Res<Board>,
) {
    let Some(entity) = queue.0.get(0) else { return };
    let Ok((pos, mut actor)) = query.get_mut(*entity) else { 
        // Entity in queue isnt an Actor and a
        // Walker with a Position
        return
    };
    let Ok(player_position) = player_query.get_single() else {
        warn!("Player has no Position component");
        return
    };
    let positions = Point::OCTANT.iter()
        .map(|dir| *dir + pos.p)
        .collect::<Vec<_>>();
    let path_to_player = pathfind::path_astar(
        pos.p, 
        player_position.p, 
        &board.iter_points().collect(), 
        &occupier_query.iter().map(|pos| pos.p).collect()
    );

    let actions = positions.iter()
        .map(|p| {
            // prefer not moving further from to player
            let mut score_mod = -player_position.p.dist_chebyshev(*p);
            if let Some(path) = &path_to_player {
                // prioritize a movement if it leads to the player
                if path.contains(p) {
                    score_mod += 5
                }
            }
            (Box::new(MoveToAction::new(*entity, *p)) as Box<dyn super::Action>, MOVE_SCORE + score_mod)
        })
        .collect::<Vec<_>>();
    actor.0.extend(actions);
}

pub fn plan_melee(
    mut query: Query<(&mut Actor, &Fighter)>,
    player_query: Query<&Position, With<Player>>,
    queue: Res<ActorQueue>,
) {
    let Some(entity) = queue.0.get(0) else {
        return
    };

    let Ok((mut actor, fighter)) = query.get_mut(*entity) else { return };
    let Ok(player_position) = player_query.get_single() else { return };
    let action = Box::new(MeleeAttackAction {
        attacker: *entity,
        target_pos: player_position.p,
        damage: fighter.strength,
    });
    actor.0.push((action, PLAYER_ATTACK_SCORE + fighter.strength as i32));
}
