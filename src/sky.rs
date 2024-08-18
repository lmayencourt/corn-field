use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_hanabi::Gradient;

use rand::Rng;

pub struct SkyPlugin;


#[derive(Component)]
struct Star {
    velocity: Vec3,
}

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        //app.add_plugins(HanabiPlugin);
        //app.add_systems(Startup, setupgpu);
        app.add_systems(Startup, setupstaticstar);
        app.add_systems(Startup, setupstar);
        app.add_systems(Update, star_movement);
    }
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
    let lifetime = module.lit(10.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(0., -3., 0.));
    let update_accel = AccelModifier::new(accel);
    let init_size = SetSizeModifier {
        size: CpuValue::Single(Vec2::new(0.1, 0.2)), // Size of each particle (0.2 units)

                                                     //size: CpuValue::Uniform((Vec2::new(0.1, 0.2), Vec2::new(0.1, 0.1))), // Size of each particle (0.2 units)
    };
    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        vec![100],
        // Spawn at a rate of 5 particles per second
        Spawner::rate(5.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("MyEffect")
    //.init(init_size) // Set the initial size
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier { gradient });

    // Insert into the asset system
    let effect_handle = effects.add(effect);

    commands.spawn(ParticleEffectBundle {
        effect: ParticleEffect::new(effect_handle),
        transform: Transform::from_xyz(0.0, 3.0, 32.0), // Specify the position (x, y, z)
        ..Default::default()
    });
}

fn setupstar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let num_stars = 20;
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    const STARFIELD_SIZE: Vec3 = Vec3::new(800.0, 600.0, 600.0);

    for _ in 0..num_stars {
        const STAR_SPEED: f32 = 100.9;

        let x = rng.gen_range(-STARFIELD_SIZE.x / 2.0..STARFIELD_SIZE.x / 2.0);
        let y = rng.gen_range(-STARFIELD_SIZE.y / 2.0..STARFIELD_SIZE.y / 2.0);
        let z = rng.gen_range(-STARFIELD_SIZE.z / 2.0..STARFIELD_SIZE.z / 2.0);
        let size = rng.gen_range(0.1..0.3);

        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(Sphere { radius: size })),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: LinearRgba::WHITE,
                    ..Default::default()
                }),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..Default::default()
            })
            .insert(Star {
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
