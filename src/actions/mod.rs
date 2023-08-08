use std::{collections::VecDeque, fmt::Debug};

use bevy::prelude::*;

use crate::state::GameState;

pub(crate) mod models;
mod systems;

/// A type implementing [Action] is a request of some kind as a separate
/// object (see the Command pattern). An [Action] can be constructed beforehand
/// and stored in a queue or another data structure (to wait for an animation 
/// to finish, for example). 
/// The action object holds all the necessary parameters required for its execution.
/// All actions provide an `execute` method that allows for applying it to the
/// game world (e.g. move a character, interact with other game objects, and so on).
///
/// Actions can generate other further actions.
pub trait Action: Send + Sync + Debug {
    fn execute(&self, world: &mut World) -> Result<Vec<Box<dyn Action>>, ()>;
}

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActorQueue>()
            .init_resource::<PendingActions>()
            .add_event::<TickEvent>()
            .add_event::<NextActorEvent>()
            .add_event::<ActionsCompleteEvent>()
            .add_event::<InvalidPlayerActionEvent>()
            .configure_set(Update, ActionSet::Planning.run_if(on_event::<NextActorEvent>()))
            .configure_set(Update, ActionSet::Planning.before(ActionSet::PostPlanning))
            .add_systems(
                Update, 
                systems::process_action_queue
                    .run_if(on_event::<TickEvent>())
                    .in_set(ActionSet::PostPlanning)
            )
            .add_systems(
                OnExit(GameState::PlayerInput), 
                systems::populate_actor_queue
            )
            .add_systems(
                Update, 
                (systems::plan_walk, systems::plan_melee)
                .in_set(ActionSet::Planning)
            );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ActionSet {
    Planning,
    PostPlanning,
}

/// Bevy [Resource] holding a queue of [Entity]s that want to perform [Action]s.
#[derive(Resource, Default)]
pub struct ActorQueue(pub VecDeque<Entity>);

/// [Resource] holding the result of previously executed [Action]s.
#[derive(Resource, Default)]
pub struct PendingActions(pub Vec<Box<dyn Action>>);

impl ActorQueue {
    pub fn pop_front(&mut self) -> Option<Entity> {
        self.0.pop_front()
    }
}

/// Sent by the [ManagerPlugin](crate::manager::ManagerPlugin) whenever the game should advance (tick). This might
/// not be the case if the application is playing an animation or other graphical 
/// transition that should be finished before progressing the game logic.
#[derive(Event)]
pub struct TickEvent;

/// Sent at the end of each successful processing of an [Actor][a] in the [ActorQueue].
/// See [systems::process_action_queue].
///
/// [a]: crate::pieces::components::Actor
#[derive(Event)]
pub struct NextActorEvent;

/// Sent by [systems::process_action_queue] has no more [Actor][a]s in the [ActorQueue]
/// to process.
///
/// [a]: crate::pieces::components::Actor
#[derive(Event)]
pub struct ActionsCompleteEvent;

/// Sent by [systems::process_action_queue] whenever an [Action] executed by the player
/// returned `false`, i.e. the action was invalid.
#[derive(Event)]
pub struct InvalidPlayerActionEvent;


