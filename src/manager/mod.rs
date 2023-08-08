use bevy::prelude::*;

use crate::{state::{MainState, GameState}, input::PlayerInputReadyEvent, actions::{ActionsCompleteEvent, InvalidPlayerActionEvent, TickEvent}, gfx::GraphicsWaitEvent};

/// This [Plugin] puts together the scheduling/flow of the game logic during gameplay. 
pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Game), game_start)
            .add_systems(OnExit(MainState::Game), game_end)
            .add_systems(Update, turn_update_start.run_if(on_event::<PlayerInputReadyEvent>()))
            .add_systems(Update, turn_update_end.run_if(on_event::<ActionsCompleteEvent>()))
            .add_systems(Update, turn_update_cancel.run_if(on_event::<InvalidPlayerActionEvent>()))
            .add_systems(Update, tick.run_if(in_state(GameState::TurnUpdate)));
    }
}

fn game_start(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

fn game_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::None);
}

fn turn_update_start(
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    next_state.set(GameState::TurnUpdate);
    ev_tick.send(TickEvent);
}

fn turn_update_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

fn turn_update_cancel(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

fn tick(
    mut ev_wait: EventReader<GraphicsWaitEvent>,
    mut ev_tick: EventWriter<TickEvent>,
) {
    if ev_wait.iter().len() == 0 {
        ev_tick.send(TickEvent);
    }
}
