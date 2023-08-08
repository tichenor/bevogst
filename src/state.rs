use bevy::prelude::States;


#[derive(Clone, Debug, Default, Hash, States, Eq, PartialEq)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game,
}

/// We have two different states during gameplay: Waiting for 
/// a player's input, or performing/updating the game logic.
#[derive(Clone, Debug, Default, Hash, States, Eq, PartialEq)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate,
}
