use avian2d::prelude::{
    Collider, CollisionEventsEnabled, CollisionStarted, Friction, Restitution, RigidBody,
};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{StopGame, app_state::AppState, ball::Ball};

const ROWS: usize = 18;
const COLUMNS: usize = 12;

#[derive(Debug)]
pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BlockBreakEvent>();

        app.add_observer(break_block_observer);

        app.add_systems(OnEnter(AppState::InGame), load_blocks_system)
            .add_systems(OnEnter(AppState::MainMenu), unload_blocks_system)
            .add_systems(
                FixedUpdate,
                (trigger_ball_break_event_system, check_for_win_system)
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Debug, Component)]
pub struct Block;

#[derive(Debug, Event)]
pub struct BlockBreakEvent(Entity);

fn load_blocks_system(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result {
    let size = windows.single()?.size();

    let block_space = 5.0;
    let rect_width = size.x / COLUMNS as f32 - block_space;
    let rect_height = 20.0;
    let mesh_handle = meshes.add(Rectangle::new(rect_width, rect_height));
    let material_handle = materials.add(Color::hsv(319.0, 0.95, 0.9));

    let origin = Vec2::new(-size.x, size.y) * 0.5
        + Vec2::new(
            (rect_width + block_space) / 2.0,
            -(rect_height + block_space) / 2.0,
        );

    let mut blocks = Vec::with_capacity(ROWS * COLUMNS);
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let x = origin.x + column as f32 * (rect_width + block_space);
            let y = origin.y - row as f32 * (rect_height + block_space);

            blocks.push((
                Block,
                Mesh2d(mesh_handle.clone()),
                MeshMaterial2d(material_handle.clone()),
                Transform::from_xyz(x, y, 0.0),
                RigidBody::Static,
                Collider::rectangle(rect_width, rect_height),
                Friction::new(0.0),
                Restitution::new(1.0),
                CollisionEventsEnabled,
            ));
        }
    }

    commands.spawn_batch(blocks);

    Ok(())
}

fn unload_blocks_system(mut commands: Commands, blocks: Query<Entity, With<Block>>) {
    for block in blocks {
        commands.entity(block).despawn();
    }

    info!("Blocks unloaded")
}

fn trigger_ball_break_event_system(
    mut commands: Commands,
    mut collision_started: EventReader<CollisionStarted>,
    balls: Query<(), With<Ball>>,
    blocks: Query<(), With<Block>>,
) {
    for &CollisionStarted(a, b) in collision_started.read() {
        let block = match (
            balls.contains(a) && blocks.contains(b),
            balls.contains(b) && blocks.contains(a),
        ) {
            (true, _) => b,
            (_, true) => a,
            _ => return,
        };

        debug!("Ball touched {}", block);

        commands.trigger(BlockBreakEvent(block));
    }
}

fn break_block_observer(trigger: Trigger<BlockBreakEvent>, mut commands: Commands) {
    commands.entity(trigger.0).despawn();
    debug!("Despawned block {}", trigger.0);
}

fn check_for_win_system(mut commands: Commands, blocks: Query<(), With<Block>>) {
    if blocks.is_empty() {
        info!("Won");
        commands.trigger(StopGame);
    };
}
