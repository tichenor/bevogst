use std::collections::VecDeque;

use bevy::prelude::*;

pub(crate) mod models;
mod systems;

pub trait Action: Send + Sync {
    /// Return true if an action is valid, false if it is invalid.
    fn execute(&self, world: &mut World) -> bool;
}

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .add_systems(
                Update, 
                systems::process_action_queue.run_if(on_event::<TickEvent>())
                );
    }
}

#[derive(Resource, Default)]
pub struct ActorQueue(pub VecDeque<Entity>);

impl ActorQueue {
    pub fn pop_front(&mut self) -> Option<Entity> {
        self.0.pop_front()
    }
}

#[derive(Event)]
pub struct TickEvent;

#[derive(Event)]
pub struct NextActorEvent;

#[derive(Event)]
pub struct ActionsCompleteEvent;

#[derive(Event)]
pub struct InvalidPlayerActionEvent;


