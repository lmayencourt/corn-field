/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
*/

use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::GameState;
use crate::menu::GameScore;
use crate::world::{Corn, levels::{LEVELS, LEVEL_COUNT}};
use crate::menu::{CurrentLevel, RestartGame};

#[derive(Component)]
struct CropCircleLights;

#[derive(Event, Default)]
pub struct ShowLights;

pub struct CropCircleLightsPlugin;

impl Plugin for CropCircleLightsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_lights);
        app.add_systems(Update, remove_lights);
        app.add_event::<ShowLights>();
    }
}

fn draw_lights(
    event: EventReader<ShowLights>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    current_level: Res<CurrentLevel>,
    mut effects: ResMut<Assets<EffectAsset>>
) {
    let COLOR_LIGHT_SCALE: Color = Color::srgb(0.0, 0.0, 1.0);
    if !event.is_empty() {
            for (y, line) in LEVELS[current_level.idx].data.lines().enumerate() {
                for (x, char) in line.chars().enumerate() {
                    if char == '0' {
                        commands.spawn((PbrBundle {
                            mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                            material: materials.add(Color::srgba(0.0, 0.0, 1.0, 0.4)),
                            transform: Transform::from_xyz(x as f32, 1.2, y as f32),
                            ..default()
                            },
                            CropCircleLights
                        // ));
                        )).with_children(
                            |children| {
                                children.spawn(PointLightBundle {
                                    point_light: PointLight {
                                        color: COLOR_LIGHT_SCALE,
                                        intensity: 1_000_000.0,
                                        range: 4.0,
                                        ..default()
                                    },
                                    transform: Transform::from_xyz(0.0, 1.2, 0.0),
                                    ..default()
                                });
                            });

                    }
                }
            }
        }
}

fn remove_lights(
    event: EventReader<RestartGame>,
    mut commands: Commands,
    lights: Query<Entity, With<CropCircleLights>>,
) {
    if !event.is_empty() {
        for light in lights.iter() {
            commands.entity(light).despawn_recursive();
        }
    }
}