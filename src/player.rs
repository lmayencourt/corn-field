use std::f32::consts::PI;

use bevy::prelude::*;

use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use bevy_hanabi::prelude::*;
use bevy_hanabi::Gradient;

use crate::GameState;
use crate::world::{Corn, levels::LEVELS};
use crate::menu::{RestartGame, CurrentLevel};

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
        app.add_systems(Update, reset_player);
        app.add_plugins(HanabiPlugin);
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {


    let COLOR_LIGHT_SCALE: Color = Color::srgb(0.0, 0.0, 1.0);

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
    )
    ).with_children(
        |children| {
            children.spawn(PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(1.0, 1.0, 0.0),
                    intensity: 500_000.0,
                    range: 10.0,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 2.0, 0.0),
                ..default()
            });
        }
    );

}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    state: Res<State<GameState>>,
    current_level: Res<CurrentLevel>,
) {
    let level_size = LEVELS[current_level.idx].grid_size as f32;
    let (mut tt, mut player) = query.single_mut();
    // Access the x, y, z coordinates
    let mut x = tt.translation.x;
    let mut z = tt.translation.z;

    let mut rotation: f32 = 0.0;
    if *state.get() == GameState::InGame {
        if player.move_delay.tick(time.delta()).finished() {
            let mut moved = false;

            if keyboard_input.pressed(KeyCode::ArrowUp) {
                if z < level_size - 1.0 {
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
                if x < level_size - 1.0 {
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

            if moved {
                player.move_delay.reset();

                tt.translation.x = x;
                tt.translation.z = z;
                tt.rotation = Quat::from_rotation_y(rotation);
            }
        }
    }
}

fn cut_corn(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    corn: Query<(&Transform, Entity), With<Corn>>,
    state: Res<State<GameState>>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    // Define a color gradient from white to transparent black
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 1., 1.0, 0.5));
    gradient.add_key(1.0, Vec4::splat(0.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(0.05),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(10.),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(1.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., -3., 0.));
    let update_accel = AccelModifier::new(accel);

    let main_set_size_modifier = SetSizeModifier {
        size: Vec2::splat(0.1).into(),
    };

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        vec![100],
        // Spawn at a rate of 5 particles per second
        Spawner::once(10.0.into(), true),
        // Move the expression module into the asset
        module,
    )
    .with_name("MyEffect2")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(main_set_size_modifier)
    .render(ColorOverLifetimeModifier { gradient });

    let effect_handle = effects.add(effect);
    let effecttt = ParticleEffect::new(effect_handle);

    let player = player.single();
    if *state.get() == GameState::InGame {
        if keyboard_input.pressed(KeyCode::Space) {
            // If space bar pressed, remove the corn at the position of the player
            for (corn_position, corn) in corn.iter() {
                if player.translation.x == corn_position.translation.x &&
                    player.translation.z == corn_position.translation.z {
                        commands.entity(corn).despawn();

                        // Insert into the asset system
                        commands.spawn(ParticleEffectBundle {
                            effect: effecttt.clone(),
                            transform: Transform::from_xyz(player.translation.x, player.translation.y + 2.0, player.translation.z), // Specify the position (x, y, z)
                            ..Default::default()
                        });
                }
            }   
        }
    }
}

fn reset_player(
    event: EventReader<RestartGame>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if !event.is_empty() {
        let mut transform = query.single_mut();
        transform.translation.x = 0.0;
        transform.translation.z = 0.0;
        transform.rotation= Quat::from_rotation_y(PI);
    }
}