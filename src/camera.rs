/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
*/

use bevy::prelude::*;

use crate::world::levels::LEVELS;
use crate::menu::CurrentLevel;

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(
    mut commands: Commands,
    current_level: Res<CurrentLevel>,
) {
    let level_size = LEVELS[current_level.idx].grid_size as f32;
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(level_size/2.0, 18.0, -level_size/2.0)
            .looking_at(Vec3::new(level_size / 2.0, 0.0, level_size / 2.0), Vec3::Y),
        ..default()
    });
}
