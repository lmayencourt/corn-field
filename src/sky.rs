use bevy::prelude::*;
use bevy_particle_systems::*;

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ParticleSystemPlugin);
        app.add_systems(Startup, spawn_particle_system);
    }
}

fn spawn_particle_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // Add the bundle specifying the particle system itself.
        .spawn(ParticleSystemBundle {
            particle_system: ParticleSystem {
                max_particles: 10_000,
                spawn_rate_per_second: 25.0.into(),
                initial_speed: JitteredValue::jittered(3.0, -1.0..1.0),
                lifetime: JitteredValue::jittered(8.0, -2.0..2.0),
                color: ColorOverTime::Gradient(Curve::new(vec![
                    CurvePoint::new(Color::WHITE, 0.0),
                    CurvePoint::new(Color::srgba(0.5, 0.5, 1.0, 1.0), 1.0),
                ])),
                looping: true,
                system_duration_seconds: 10.0,
              
                ..ParticleSystem::default()
            },
            transform: Transform::from_xyz(1.0, 0.0, 1.0),
            ..ParticleSystemBundle::default()
        })
        // Add the playing component so it starts playing. This can be added later as well.
        .insert(Playing);
}