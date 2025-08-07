use avian2d::prelude::{Collider, Friction, GravityScale, LinearVelocity, Restitution, RigidBody};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::app_state::AppState;

const BALL_RADIUS: f32 = 16.0;
const MAX_SPEED: f32 = 600.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_ball_observer)
            .add_observer(initial_velocity_observer);

        app.add_systems(OnEnter(AppState::MainMenu), despawn_balls_system)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (check_ball_death, hold_speed_system).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Debug, Resource)]
struct BallHandles {
    pub mesh_handle: Handle<Mesh>,
    pub material_handle: Handle<ColorMaterial>,
}

#[derive(Debug, Component)]
pub struct Ball;

#[derive(Debug, Event)]
struct SpawnBallEvent;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(BallHandles {
        mesh_handle: meshes.add(Circle::new(BALL_RADIUS)),
        material_handle: materials.add(Color::linear_rgb(0.9, 0.9, 0.9)),
    });

    commands.trigger(SpawnBallEvent);
}

fn spawn_ball_observer(
    _: Trigger<SpawnBallEvent>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    ball_handles: Res<BallHandles>,
) -> Result {
    let size = windows.single()?.size();

    commands.spawn((
        Ball,
        Mesh2d(ball_handles.mesh_handle.clone()),
        MeshMaterial2d(ball_handles.material_handle.clone()),
        Transform::from_xyz(0.0, (-size.y / 2.0 / 8.0) * 6.0, 0.0),
        RigidBody::Dynamic,
        Collider::circle(BALL_RADIUS),
        Restitution::new(1.0),
        GravityScale(0.0),
        Friction::new(0.0),
    ));

    Ok(())
}

fn despawn_balls_system(mut commands: Commands, balls: Query<Entity, With<Ball>>) {
    for ball in balls {
        commands.entity(ball).despawn();
    }
}

fn initial_velocity_observer(trigger: Trigger<OnAdd, Ball>, mut commands: Commands) {
    commands
        .entity(trigger.target())
        .insert(LinearVelocity(Vec2::new(0.5, 1.0).normalize() * MAX_SPEED));
}

fn check_ball_death(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    balls: Query<(Entity, &Transform), With<Ball>>,
) -> Result {
    let y = windows.single()?.height();
    let half_y = y / 2.0;

    for (entity, transform) in balls.iter() {
        if transform.translation.y + BALL_RADIUS < -half_y {
            commands.entity(entity).despawn();
            commands.trigger(SpawnBallEvent);
        }
    }

    Ok(())
}

fn hold_speed_system(balls: Query<&mut LinearVelocity, With<Ball>>) {
    for mut velocity in balls {
        velocity.0 = velocity.normalize() * MAX_SPEED;
    }
}
