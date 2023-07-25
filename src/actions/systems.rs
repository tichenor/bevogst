use bevy::prelude::*;

use crate::{pieces::components::Actor, player::Player};

use super::ActorQueue;

pub fn process_action_queue(world: &mut World) {
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

    let Some(action) = actor.0.take() else { 
        warn!(
            "Entity {:?} in action queue ({}) has action 'None' in their Actor component: {}",
            entity,
            std::any::type_name::<ActorQueue>(),
            std::any::type_name::<Actor>(),
        );
        return;
    };

    if !action.execute(world) && world.get::<Player>(entity).is_some() {
        world.send_event(super::InvalidPlayerActionEvent);
        return;
    }
    world.send_event(super::NextActorEvent);
}
