//! This is a Breakout clone.

use bevy::prelude::*;

use crate::{app_state::AppState, ball::BallPlugin, player::PlayerPlugin, world::WorldPlugin};

mod app_state;
mod ball;
mod player;
mod world;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, avian2d::PhysicsPlugins::default()));

    app.init_state::<AppState>();

    app.add_plugins((WorldPlugin, PlayerPlugin, BallPlugin));

    app.add_observer(start_game_observer)
        .add_observer(stop_game_observer);

    app.add_systems(
        Startup,
        (
            show_archetypes,
            // TODO: Temporary
            |mut commands: Commands| {
                commands.trigger(StartGame);
            },
        ),
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
