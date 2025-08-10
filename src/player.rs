use std::ops::Deref;

use avian2d::prelude::{Collider, Friction, LockedAxes, Restitution, RigidBody};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::app_state::AppState;

const SPEED: f32 = 512.0;

const PADDLE_HEIGHT: f32 = 10.0;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_paddle_observer);

        app.add_systems(Startup, setup)
            .add_systems(OnEnter(AppState::InGame), player_spawn_system)
            .add_systems(OnEnter(AppState::MainMenu), player_despawn_system)
            .add_systems(
                Update,
                (player_movement_system, handle_border_collision_system)
                    .chain()
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Debug, Resource)]
struct PlayerMeshResource(Handle<Mesh>);

#[derive(Debug, Resource)]
struct PlayerSize(Vec2);

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub enum Player {
    First,
    Second,
}

#[derive(Debug, Event)]
struct SpawnPlayer(Player);

impl Deref for PlayerSize {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn setup(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
) -> Result {
    let size = windows.single()?.size();

    let paddle_size = get_paddle_size(&size);
    commands.insert_resource(PlayerSize(paddle_size));

    let player_mesh = meshes.add(Rectangle::from_size(paddle_size));
    commands.insert_resource(PlayerMeshResource(player_mesh));

    Ok(())
}

fn player_spawn_system(mut commands: Commands) {
    commands.trigger(SpawnPlayer(Player::First));
    commands.trigger(SpawnPlayer(Player::Second));

    info!("Spawned first player");
}

fn player_despawn_system(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player in players {
        commands.entity(player).despawn();
    }

    info!("Despawned players");
}

fn spawn_paddle_observer(
    trigger: Trigger<SpawnPlayer>,
    player_size: Res<PlayerSize>,
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    player_mesh: Res<PlayerMeshResource>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    players: Query<&Player>,
) -> Result {
    for player in players {
        if player == &trigger.0 {
            return Err(BevyError::from("Can not create the same player twice"));
        };
    }

    let size = windows.single()?.size();
    let half_size = size / 2.0;
    let paddle_center = (-half_size.y / 8.0) * 7.0;

    let material = materials.add(get_paddle_color(&trigger.0));
    let transform = match &trigger.0 {
        Player::First => Transform::from_xyz(0.0, paddle_center + PADDLE_HEIGHT * 0.75, 0.0),
        Player::Second => Transform::from_xyz(0.0, paddle_center - PADDLE_HEIGHT * 0.75, 0.0),
    };

    commands.spawn((
        trigger.0.clone(),
        Mesh2d(player_mesh.0.clone()),
        MeshMaterial2d(material),
        transform,
        RigidBody::Kinematic,
        Collider::rectangle(player_size.x, player_size.y),
        LockedAxes::new().lock_rotation().lock_translation_y(),
        Restitution::new(1.0),
        Friction::new(0.0),
    ));

    Ok(())
}

fn player_movement_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in players.iter_mut() {
        let (left, right) = match player {
            Player::First => (KeyCode::KeyA, KeyCode::KeyD),
            Player::Second => (KeyCode::ArrowLeft, KeyCode::ArrowRight),
        };

        let axis = i8::from(keys.pressed(right)) - i8::from(keys.pressed(left));
        if axis != 0 {
            transform.translation.x += axis as f32 * SPEED * time.delta_secs();
        };
    }
}

fn handle_border_collision_system(
    player_size: Res<PlayerSize>,
    windows: Query<&Window, With<PrimaryWindow>>,
    players: Query<&mut Transform, With<Player>>,
) -> Result {
    let win_width = windows.single()?.width();
    let half_limit = win_width * 0.5 - player_size.x * 0.5;

    for mut transform in players {
        transform.translation.x = transform.translation.x.clamp(-half_limit, half_limit);
    }

    Ok(())
}

fn get_paddle_color(player: &Player) -> Color {
    match player {
        Player::First => Color::linear_rgb(0.0, 0.0, 1.0),
        Player::Second => Color::linear_rgb(1.0, 0.0, 0.0),
    }
}

fn get_paddle_size(window_size: &Vec2) -> Vec2 {
    Vec2::new(window_size.x / 8.0, PADDLE_HEIGHT)
}
