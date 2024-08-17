/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
*/

use bevy::prelude::*;

use crate::world::WORLD_SIZE;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands:Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 10.0, -10.0).looking_at(Vec3::new(WORLD_SIZE/2.0, 0.0, WORLD_SIZE/2.0), Vec3::Y),
        ..default()
    });
}