use bevy::prelude::*;

use std::f32::consts::PI;

#[derive(Default)]
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    move_delay: Timer,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);

       app.add_systems(
            Update,
            update_player);

    }
}

fn update_player(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transforms: Query<&mut Transform, With<Player>>,
    mut player: Query<&mut Player>,
    time: Res<Time>
)
{

    let mut tt = transforms.get_single_mut().unwrap();
    let mut player = player.get_single_mut().unwrap();
    // Access the x, y, z coordinates
    let mut x = tt.translation.x;
    let mut z = tt.translation.z;


    let mut rotation: f32 = 0.0;
    if player.move_delay.tick(time.delta()).finished() {
        let mut moved = false;

        if keyboard_input.pressed(KeyCode::ArrowUp) {
            if z < 16.0 - 1.0 {
                z += 1.0;
            }
            rotation = -PI / 2.;
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowDown) {
            if z > 0.0 {
                z-= 1.0;
            }
            rotation = PI / 2.;
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            if x < 16.0 - 1.0{ 
                x += 1.0;
            }
            rotation = PI;
            moved = true;
        }

        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if x > 0.0 {
                x -= 1.0;
            }
            rotation = 0.0;
            moved = true;
        }

        println!("Coordinates: x: {},  z: {}", x, z);

        if moved {
            player.move_delay.reset();
        }

        *tt = Transform {
            translation: Vec3::new(
                x,
                1.0,
                z,
            ),
            rotation: Quat::from_rotation_y(rotation),
            ..default()
        };
        
        
    }

}

fn setup_player(
    mut commands:Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
            material: materials.add(Color::srgb(0.0, 0.0, 0.0)),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        },
        Player {
            move_delay: Timer::from_seconds(0.1,TimerMode::Once),
        },
    ));
}

