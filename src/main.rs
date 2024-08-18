/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
*/

use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

mod audio;
mod camera;
mod menu;
mod player;
mod world;
mod minimap;
mod sky;

use audio::audio::AudioPlugin;
use camera::GameCameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
use minimap::MinimapPlugin;
use menu::MenuPlugin;
use sky::SkyPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    LandingScreen,
    InGame,
    EndGame,
    Score,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin{mode: PluginMode::ReplaceDefault})
        .add_plugins(DefaultPlugins)
        .insert_state(GameState::LandingScreen)
        .add_plugins(SkyPlugin)
        .add_plugins(GameCameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MinimapPlugin)
        .add_plugins(MenuPlugin)
        
        .run();
}
