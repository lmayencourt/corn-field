use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::sprite::Mesh2dHandle;

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
        //app.add_plugins(HanabiPlugin);
        //app.add_systems(Startup, setupgpu);
        app.add_systems(Startup, setupstaticstar);
        app.add_systems(Startup, setupstar);
        app.add_systems(Update, (draw_ghost_trail, star_movement));
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
