use avian2d::prelude::{Collider, Friction, Restitution, RigidBody};
use bevy::prelude::*;

use crate::{app_state::AppState, world::block::BlockPlugin};

mod block;

#[derive(Debug)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BlockPlugin);

        app.add_systems(OnEnter(AppState::InGame), load_level_system)
            .add_systems(OnEnter(AppState::MainMenu), unload_level_system);
    }
}

#[derive(Debug, Component)]
pub struct Border;

fn load_level_system(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) -> Result {
    let size = windows.single()?.size();

    let border_material = materials.add(Color::linear_rgb(1.0, 1.0, 1.0));

    let top_mesh = meshes.add(Rectangle::new(size.x, 5.0));
    let top_collider = Collider::rectangle(size.x, 5.0);
    let side_mesh = meshes.add(Rectangle::new(5.0, size.y));
    let side_collider = Collider::rectangle(5.0, size.y);

    commands.spawn_batch([
        // Top
        (
            Border,
            Mesh2d(top_mesh),
            MeshMaterial2d(border_material.clone()),
            Transform::from_xyz(0.0, size.y / 2.0, 0.0),
            RigidBody::Static,
            top_collider,
            Restitution::new(1.0),
            Friction::new(0.0),
        ),
        // Left
        (
            Border,
            Mesh2d(side_mesh.clone()),
            MeshMaterial2d(border_material.clone()),
            Transform::from_xyz(-size.x / 2.0, 0.0, 0.0),
            RigidBody::Static,
            side_collider.clone(),
            Restitution::new(1.0),
            Friction::new(0.0),
        ),
        // Right
        (
            Border,
            Mesh2d(side_mesh),
            MeshMaterial2d(border_material),
            Transform::from_xyz(size.x / 2.0, 0.0, 0.0),
            RigidBody::Static,
            side_collider,
            Restitution::new(1.0),
            Friction::new(0.0),
        ),
    ]);

    info!("Level loaded");

    Ok(())
}

fn unload_level_system(mut commands: Commands, borders: Query<Entity, With<Border>>) {
    for border in borders {
        commands.entity(border).despawn();
    }

    info!("Level unloaded")
}
