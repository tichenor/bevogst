use std::collections::VecDeque;

use bevy::prelude::*;

use crate::{point::Point, board::components::Position, player::Player, state::GameState, pieces::components::Actor, actions::{ActorQueue, models::MoveToAction}};

const DIR_KEY_MAP: [(KeyCode, Point); 4] = [
    (KeyCode::W, Point::NORTH), (KeyCode::S, Point::SOUTH),
    (KeyCode::A, Point::WEST), (KeyCode::D, Point::EAST),
];

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerInputReadyEvent>()
            .add_systems(Update, player_position.run_if(in_state(GameState::PlayerInput)));

    }
}

#[derive(Event)]
pub struct PlayerInputReadyEvent;

fn player_position(
    keys: ResMut<Input<KeyCode>>,
    mut player_query: Query<(Entity, &Position, &mut Actor), With<Player>>,
    mut queue: ResMut<ActorQueue>,
    mut ev_input: EventWriter<PlayerInputReadyEvent>,
) {
    // If number of query items is not == 1, return
    let Ok((entity, pos, mut actor)) = player_query.get_single_mut() else { return };
    for (key, dir) in DIR_KEY_MAP {
        if !keys.just_pressed(key) { continue; }
        let action = MoveToAction(entity, pos.p + dir);
        actor.0 = Some(Box::new(action));
        queue.0 = VecDeque::from([entity]);
        ev_input.send(PlayerInputReadyEvent);
    }
}
