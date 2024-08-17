/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
*/

use bevy::prelude::*;

mod camera;
mod world;
mod audio;
mod player;

use camera::GameCameraPlugin;
use world::WorldPlugin;
use audio::audio::AudioPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameCameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
