use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sound);
    }
}

fn setup_sound(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(AudioBundle {
        source: asset_server.load("embedded://sounds/game.ogg"),
        settings: PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            ..default()
        },
        ..default()
    });
}
