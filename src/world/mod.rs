/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
 */

use bevy::prelude::*;

/// Size of the world and game grid
pub const WORLD_SIZE: f32 = 20.0;
pub const GRID_SIZE: f32 = 16.0;

pub const YELLOW: Color = Color::srgb(234.0 / 255.0, 189.0 / 255.0, 71.0 / 255.0);

/// Component to identify the Corn
#[derive(Component)]
pub struct Corn;

/// Plugin to be included in main application
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

/// System to spawn all the entities for the game world
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(WORLD_SIZE, WORLD_SIZE)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        transform: Transform::from_xyz(
            WORLD_SIZE / 2.0 - (WORLD_SIZE - GRID_SIZE) / 2.0,
            0.0,
            WORLD_SIZE / 2.0 - (WORLD_SIZE - GRID_SIZE) / 2.0,
        ),
        ..default()
    },));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
        ..default()
    });

    // 0 0 marker stone
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 2.0, 0.2)),
        material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..default()
    });
    // x marker stone
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 2.0, 0.2)),
        material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
        transform: Transform::from_xyz(1.0, 1.0, 0.0),
        ..default()
    });
    // z marker stone
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 2.0, 0.2)),
        material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
        transform: Transform::from_xyz(0.0, 1.0, 2.0),
        ..default()
    });
    // end of grid stone
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 2.0, 0.2)),
        material: materials.add(Color::srgb(0.0, 0.0, 1.0)),
        transform: Transform::from_xyz(GRID_SIZE - 1.0, 1.0, GRID_SIZE - 1.0),
        ..default()
    });

    // We need apparently to work on the X - Z plane, Y being the height for us.
    for x in 0..GRID_SIZE as usize {
        for z in 0..GRID_SIZE as usize {
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    material: materials.add(YELLOW),
                    transform: Transform::from_xyz(x as f32, 0.50, z as f32),
                    ..default()
                    },
                Corn,
                ));
        }
    }
}
