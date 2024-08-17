/* SPDX-License-Identifier: MIT
 * Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
 */

use std::isize;

use bevy::prelude::*;
use rand::Rng; // 0.8.5

pub mod levels;

/// Size of the world and game grid
pub const WORLD_OFFSET_OF_GRID: f32 = 5.0;
pub const GRID_SIZE: f32 = 17.0;
pub const WORLD_SIZE: f32 = GRID_SIZE + 2.0 * WORLD_OFFSET_OF_GRID;

pub const YELLOW: Color = Color::srgb(234.0 / 255.0, 189.0 / 255.0, 71.0 / 255.0);
pub const MILESTONE_COLOR: Color = Color::srgb(155.0/255.0, 34.0/255.0, 38.0/255.0);
pub const SCALE_COLOR: Color = Color::srgb(34.0/255.0, 34.0/255.0, 255.0/255.0);

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

    let FLOOR_SIZE = GRID_SIZE + WORLD_OFFSET_OF_GRID;
    let WORLD_OFFSET_OF_GRID2 = -WORLD_OFFSET_OF_GRID as isize;
    // This is the floor of the game, adding 2 tiles of margin
    for x in WORLD_OFFSET_OF_GRID2..FLOOR_SIZE  as isize {
        for z in WORLD_OFFSET_OF_GRID2..FLOOR_SIZE as isize {

            let num = rand::thread_rng().gen_range(0.0..0.10);

            if (x < 0) || (z < 0) || (z >= GRID_SIZE as isize) || (x >= GRID_SIZE as isize)
            {
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
                        transform: Transform::from_xyz(x as f32, -num,  z as f32),
                        ..default()
                        },
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
                    ));

            }
        }
    }

    // 0 0 marker stone
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 2.0, 0.2)),
        material: materials.add(MILESTONE_COLOR),
        transform: Transform::from_xyz(-1.0, 1.0, -1.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 1.0, 0.2)),
        material: materials.add(SCALE_COLOR),
        transform: Transform::from_xyz((GRID_SIZE - 1.0) / 2.0, 1.0, -1.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(0.2, 1.0, 0.2)),
        material: materials.add(SCALE_COLOR),
        transform: Transform::from_xyz(-1.0, 1.0, (GRID_SIZE - 1.0) / 2.0),
        ..default()
    });

    // We need apparently to work on the X - Z plane, Y being the height for us.
    for x in 0..GRID_SIZE as usize {
        for z in 0..GRID_SIZE as usize {

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
}
