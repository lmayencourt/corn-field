/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
 */

use std::isize;

use bevy::prelude::*;
use rand::Rng; // 0.8.5

use crate::menu::{RestartGame, CurrentLevel};
use crate::world::levels::LEVELS;

pub mod levels;

/// Size of the world and game grid
pub const WORLD_OFFSET_OF_GRID: isize = 5;

pub const YELLOW: Color = Color::srgb(234.0 / 255.0, 189.0 / 255.0, 71.0 / 255.0);
pub const MILESTONE_COLOR: Color = Color::srgb(155.0/255.0, 34.0/255.0, 38.0/255.0);
pub const SCALE_COLOR: Color = Color::srgb(34.0/255.0, 34.0/255.0, 255.0/255.0);

/// Component to identify the Corn
#[derive(Component)]
pub struct Corn;

/// Component to identify the Grid Floor
#[derive(Component)]
pub struct Floor;

/// Component to identify the Grid Marks
#[derive(Component)]
pub struct Marker;

/// Plugin to be included in main application
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
        app.add_systems(Update, reset_world);
    }
}

/// System to spawn all the entities for the game world
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_level: Res<CurrentLevel>,
) {
    let level_size = LEVELS[current_level.idx].grid_size as f32;

    // 0 0 marker stone
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 2.0, 0.2)),
        material: materials.add(MILESTONE_COLOR),
        transform: Transform::from_xyz(-1.0, 1.0, -1.0),
        ..default()
    });

    spawn_board(commands, meshes, materials, current_level);
}

fn reset_world(
    event: EventReader<RestartGame>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    corns: Query<Entity, With<Corn>>,
    markers: Query<Entity, With<Marker>>,
    floors: Query<Entity, With<Floor>>,
    current_level: Res<CurrentLevel>,
) {
    if !event.is_empty() {
        for corn in corns.iter() {
            commands.entity(corn).despawn();
        }

        for marker in markers.iter() {
            commands.entity(marker).despawn();
        }

        for floor in floors.iter() {
            commands.entity(floor).despawn();
        }

        spawn_board(commands, meshes, materials, current_level);
    }
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_level: Res<CurrentLevel>,
) {
    let level_size = LEVELS[current_level.idx].grid_size as f32;

    let floor_size = level_size as isize + WORLD_OFFSET_OF_GRID;
    let world_offset_of_grid = -WORLD_OFFSET_OF_GRID;
    // This is the floor of the game, adding 2 tiles of margin
    for x in world_offset_of_grid..floor_size {
        for z in world_offset_of_grid..floor_size {

            let num = rand::thread_rng().gen_range(0.0..0.10);

            if (x < 0) || (z < 0) || (z >= level_size as isize) || (x >= level_size as isize)
            {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
                        transform: Transform::from_xyz(x as f32, -num,  z as f32),
                        ..default()
                        },
                        Floor,
                    ));
            }
            else
            {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(1.0, 0.3, 1.0)),
                        material: materials.add(Color::srgba_u8(53, 33, 0, 255)),
                        transform: Transform::from_xyz(x as f32, -num,  z as f32),
                        ..default()
                        },
                        Floor,
                    ));

            }
        }
    }

    // We need apparently to work on the X - Z plane, Y being the height for us.
    for x in 0..level_size as usize {
        for z in 0..level_size as usize {

            let num = rand::thread_rng().gen_range(0.45..0.55);

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                    material: materials.add(StandardMaterial {
                        reflectance: 0.00,
                        unlit: false,
                        base_color: YELLOW,
                    ..Default::default()}),
                    transform: Transform::from_xyz(x as f32, num,  z as f32),
                    ..default()
                    },
                Corn,
                ));
        }
    }

    // Grid Markers
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 1.0, 0.2)),
            material: materials.add(SCALE_COLOR),
            transform: Transform::from_xyz((level_size - 1.0) / 2.0, 1.0, -1.0),
            ..default()
        },
        Marker,
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.2, 1.0, 0.2)),
            material: materials.add(SCALE_COLOR),
            transform: Transform::from_xyz(-1.0, 1.0, (level_size - 1.0) / 2.0),
            ..default()
        },
        Marker,
    ));
}