use std::collections::VecDeque;

use bevy::{prelude::*, app::AppExit};

use crate::{point::Point, board::components::Position, player::Player, state::GameState, pieces::components::Actor, actions::{ActorQueue, models::MoveToAction}};

const DIR_KEY_MAP: [(KeyCode, Point); 20] = [
    // wsad movement
    (KeyCode::W, Point::NORTH),                 (KeyCode::S, Point::SOUTH),
    (KeyCode::A, Point::WEST),                  (KeyCode::D, Point::EAST),
    // hjkl movement (Rogue vi keys)
    (KeyCode::H, Point::WEST),                  (KeyCode::J, Point::SOUTH),
    (KeyCode::K, Point::NORTH),                 (KeyCode::L, Point::EAST),
    (KeyCode::Y, Point::NORTH_WEST),            (KeyCode::U, Point::NORTH_EAST),
    (KeyCode::B, Point::SOUTH_WEST),            (KeyCode::N, Point::SOUTH_EAST),
    // numpad movement
    (KeyCode::Numpad4, Point::WEST),            (KeyCode::Numpad2, Point::SOUTH),
    (KeyCode::Numpad8, Point::NORTH),           (KeyCode::Numpad6, Point::EAST),
    (KeyCode::Numpad7, Point::NORTH_WEST),      (KeyCode::Numpad9, Point::NORTH_EAST),
    (KeyCode::Numpad1, Point::SOUTH_WEST),      (KeyCode::Numpad3, Point::SOUTH_EAST),
];

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerInputReadyEvent>()
            .add_systems(Update, handle_movement_keys.run_if(in_state(GameState::PlayerInput)))
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

#[derive(Event)]
pub struct PlayerInputReadyEvent;

fn handle_movement_keys(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    // If number of query items is not == 1, return
    let Ok((entity, pos, mut actor)) = player_query.get_single_mut() else { return };
    for (key, dir) in DIR_KEY_MAP {
        if !keys.just_pressed(key) { continue; }
        let action = MoveToAction::new(entity, pos.p + dir);
        // action score does not matter for the player
        actor.0 = vec![(Box::new(action), 0)];
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
    }
}
