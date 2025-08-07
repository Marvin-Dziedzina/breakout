use bevy::state::state::States;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}
