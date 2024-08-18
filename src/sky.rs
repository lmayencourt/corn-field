use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;
use bevy_hanabi::prelude::*;
use bevy_hanabi::Gradient;
use bevy_particle_systems::ColorOverTime;
use bevy_particle_systems::Curve;
use bevy_particle_systems::CurvePoint;
use bevy_particle_systems::EmitterShape;
use bevy_particle_systems::JitteredValue;
use bevy_particle_systems::Line;
use bevy_particle_systems::ParticleSystem;
use bevy_particle_systems::ParticleSystemBundle;
use bevy_particle_systems::ParticleSystemPlugin;

use bevy_particle_systems::Playing;
use rand::Rng;

pub struct SkyPlugin;

#[derive(Component)]
struct Star {
    velocity: Vec3,
    hasTrail: bool,
}

#[derive(Component)]
struct GhostTrail {
    life: u32,
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setupgpu);
        app.add_systems(Startup, setupstaticstar);
        app.add_systems(Startup, setupstar);
        app.add_systems(Startup, spawn_particle_system);
        app.add_systems(Update, (draw_ghost_trail, star_movement));
    }
}

fn spawn_particle_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // Add the bundle specifying the particle system itself.
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 1024,
                emitter_shape: EmitterShape::Line(Line {
                    length: 200.0,
                    angle: JitteredValue::jittered(std::f32::consts::PI, -0.1..0.1),
                }),
                spawn_rate_per_second: 50.0.into(),
                initial_speed: JitteredValue::jittered(150.0, -50.0..50.0),
                lifetime: JitteredValue::jittered(18.0, -2.0..2.0),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::WHITE, 0.0),
                    CurvePoint::new(Color::rgba(0.5, 0.5, 1.0, 0.0), 1.0),
                ])),
                initial_scale: JitteredValue::jittered(2.5, -1.0..1.0),
                looping: true,
                system_duration_seconds: 10.0,
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(500.0, 0.0, 200.0),
            ..ParticleSystemBundle::default()
        })
        // Add the playing component so it starts playing. This can be added later as well.
        .insert(Playing);
}

fn setupstaticstar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Generate stars
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let num_stars = 2000;
    const STARFIELD_SIZE: Vec3 = Vec3::new(800.0, 600.0, 600.0);

    for _ in 0..num_stars {
        let x = rng.gen_range(-STARFIELD_SIZE.x / 2.0..STARFIELD_SIZE.x / 2.0);
        let y = rng.gen_range(-STARFIELD_SIZE.y / 2.0..STARFIELD_SIZE.y / 2.0);
        let z = rng.gen_range(-STARFIELD_SIZE.z / 2.0..STARFIELD_SIZE.z / 2.0);
        let size = rng.gen_range(0.1..0.3);

        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Sphere { radius: size })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: LinearRgba::WHITE,
                ..Default::default()
            }),
            transform: Transform::from_translation(Vec3::new(x, y, z)),
            ..Default::default()
        });
    }
}

fn setupgpu(
    mut commands: Commands,
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
    .with_name("MyEffect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(main_set_size_modifier)
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands.spawn(ParticleEffectBundle {
        effect: ParticleEffect::new(effect_handle),
        transform: Transform::from_xyz(0.0, 3.0, 0.0), // Specify the position (x, y, z)
        ..Default::default()
    });
}

fn setupstar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_stars = 60;
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    const STARFIELD_SIZE: Vec3 = Vec3::new(800.0, 600.0, 600.0);

    for _ in 0..num_stars {
        const STAR_SPEED: f32 = 100.9;

        let x = rng.gen_range(-STARFIELD_SIZE.x / 2.0..STARFIELD_SIZE.x / 2.0);
        let y = rng.gen_range(-STARFIELD_SIZE.y / 2.0..STARFIELD_SIZE.y / 2.0);
        let z = rng.gen_range(-STARFIELD_SIZE.z / 2.0..STARFIELD_SIZE.z / 2.0);
        let size = rng.gen_range(0.1..0.3);

        let emissive = rng.gen_range(1.0..10000.0);
        let alpha = rng.gen_range(1.0..10.0);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere { radius: size })),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: LinearRgba {
                        red: emissive,
                        green: emissive,
                        blue: emissive,
                        alpha: alpha,
                    },
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..Default::default()
            })
            .insert(Star {
                hasTrail: true,
                velocity: Vec3::new(
                    rng.gen_range(-1.0..1.0) * STAR_SPEED,
                    rng.gen_range(-1.0..1.0) * STAR_SPEED,
                    rng.gen_range(-1.0..1.0) * STAR_SPEED,
                ),
            });
    }
}

fn star_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Star)>) {
    const STARFIELD_SIZE: Vec3 = Vec3::new(800.0, 600.0, 600.0);

    for (mut transform, star) in query.iter_mut() {
        transform.translation += star.velocity * time.delta_seconds();

        // Wrap around if stars go out of bounds
        if transform.translation.x > STARFIELD_SIZE.x / 2.0 {
            transform.translation.x = -STARFIELD_SIZE.x / 2.0;
        } else if transform.translation.x < -STARFIELD_SIZE.x / 2.0 {
            transform.translation.x = STARFIELD_SIZE.x / 2.0;
        }

        if transform.translation.y > STARFIELD_SIZE.y / 2.0 {
            transform.translation.y = -STARFIELD_SIZE.y / 2.0;
        } else if transform.translation.y < -STARFIELD_SIZE.y / 2.0 {
            transform.translation.y = STARFIELD_SIZE.y / 2.0;
        }

        if transform.translation.z > STARFIELD_SIZE.z / 2.0 {
            transform.translation.z = -STARFIELD_SIZE.z / 2.0;
        } else if transform.translation.z < -STARFIELD_SIZE.z / 2.0 {
            transform.translation.z = STARFIELD_SIZE.z / 2.0;
        }
    }
}

fn draw_ghost_trail(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&mut Transform, &Star)>,
    mut trail_query: Query<(&mut GhostTrail, &mut Transform, Entity), Without<Star>>,
) {
    for (mut trail, mut transform, entity) in &mut trail_query {
        if trail.life > 0 {
            trail.life -= 1;
            transform.scale.x *= 0.9;
            transform.scale.y *= 0.9;
        } else {
            command.entity(entity).despawn();
        }
    }

    for (visible, star) in query.iter_mut() {
        if star.hasTrail {
            let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
            let size = rng.gen_range(0.1..0.3);

            // Spawn a new trail at the current pendulum position
            command
                .spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle { radius: size })),
                    material: materials.add(Color::WHITE),
                    transform: visible.clone(),
                    ..default()
                })
                .insert(GhostTrail { life: 40 });
        }
    }
}
