/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
*/

use bevy::prelude::*;

mod audio;
mod camera;
mod player;
mod world;

use audio::audio::AudioPlugin;
use camera::GameCameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameCameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
