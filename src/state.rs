use bevy::prelude::States;


#[derive(Clone, Debug, Default, Hash, States, Eq, PartialEq)]
pub enum MainState {
    #[default]
    LoadAssets,
    Game,
}

#[derive(Clone, Debug, Default, Hash, States, Eq, PartialEq)]
pub enum GameState {
    #[default]
    None,
    PlayerInput,
    TurnUpdate,
}
