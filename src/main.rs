//! This is a Breakout clone.

use bevy::prelude::*;

use crate::{
    app_state::AppState, ball::BallPlugin, main_menu::MainMenuPlugin, player::PlayerPlugin,
    world::WorldPlugin,
};

mod app_state;
mod ball;
mod main_menu;
mod player;
mod world;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, avian2d::PhysicsPlugins::default()));

    app.init_state::<AppState>();

    app.add_plugins((MainMenuPlugin, WorldPlugin, PlayerPlugin, BallPlugin));

    app.add_observer(start_game_observer)
        .add_observer(stop_game_observer);

    app.add_systems(
        Startup,
        (show_archetypes, |mut commands: Commands| {
            commands.spawn(Camera2d);
        }),
    )
    .add_systems(
        Update,
        stop_game_on_esc_system.run_if(in_state(AppState::InGame)),
    );

    app.run();
}

fn show_archetypes(world: &World) {
    debug!("Archetypes: {}", world.archetypes().len());
}

#[derive(Debug, Event)]
pub struct StartGame;

#[derive(Debug, Event)]
pub struct StopGame;

fn start_game_observer(_: Trigger<StartGame>, mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::InGame);
}

fn stop_game_observer(_: Trigger<StopGame>, mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::MainMenu);
}

fn stop_game_on_esc_system(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        commands.trigger(StopGame);
    };
}
