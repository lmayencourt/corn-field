use bevy::prelude::*;

use std::f32::consts::PI;

use crate::world::{GRID_SIZE, Corn};

#[derive(Default)]
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    move_delay: Timer,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, move_player);
        app.add_systems(Update, cut_corn);
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        SceneBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(2.0)),
            scene: asset_server
                .load(GltfAssetLabel::Scene(0).from_asset("models/alien.glb")),
            ..default()
        },
        Player {
            move_delay: Timer::from_seconds(0.18, TimerMode::Once),
        },
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
) {
    let (mut tt, mut player) = query.single_mut();
    // Access the x, y, z coordinates
    let mut x = tt.translation.x;
    let mut z = tt.translation.z;

    let mut rotation: f32 = 0.0;
    if player.move_delay.tick(time.delta()).finished() {
        let mut moved = false;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            if z < GRID_SIZE - 1.0 {
                z += 1.0;
            }
            rotation = PI;
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            if z > 0.0 {
                z -= 1.0;
            }
            rotation = 0.0;
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if x < GRID_SIZE - 1.0 {
                x += 1.0;
            }
            rotation = -PI / 2.;
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            if x > 0.0 {
                x -= 1.0;
            }
            rotation = PI/2.0;
            moved = true;
        }

        debug!("Coordinates: x: {},  z: {}", x, z);

        if moved {
            player.move_delay.reset();

            tt.translation.x = x;
            tt.translation.z = z;
            tt.rotation = Quat::from_rotation_y(rotation);
        }

    }
}

fn cut_corn(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    corn: Query<(&Transform, Entity), With<Corn>>
) {
    let player = player.single();
    if keyboard_input.pressed(KeyCode::Space) {
        // If space bar pressed, remove the corn at the position of the player
        for (corn_position, corn) in corn.iter() {
            if player.translation.x == corn_position.translation.x &&
                player.translation.z == corn_position.translation.z {
                    commands.entity(corn).despawn();
                }
        }
    }
}