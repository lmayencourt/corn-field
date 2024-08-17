/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
*/

use bevy::prelude::*;

use crate::GameState;
use crate::world::{Corn, YELLOW, GRID_SIZE, levels::LEVEL_1};

/// Global resource that contains the score of the game
#[derive(Resource, Default)]
struct GameScore {
    forgotten: u32,
    mistakes: u32,
}

/// Component used to mark the mistakes at the end of the game
#[derive(Component)]
struct ScoreMarker;

#[derive(Event, Default)]
struct ComputeScoreEvent;

/// Restart all the game element
#[derive(Event, Default)]
struct RestartGame;

/// Old the previous input to provide a cool down to the enter key
#[derive(Resource)]
struct PreviousKeyboardInput {
    previous_key: Option<KeyCode>,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, manage_menu);
        app.insert_resource(PreviousKeyboardInput{previous_key: None});
        app.add_systems(Update, compute_score);
        app.add_event::<ComputeScoreEvent>();
        app.insert_resource(GameScore::default());
        app.add_event::<RestartGame>();
        app.add_systems(Update, restart_all);
    }
}

fn manage_menu(
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut event: EventWriter<ComputeScoreEvent>,
    mut restart: EventWriter<RestartGame>,
    mut old_input: ResMut<PreviousKeyboardInput>,
)
{
    match state.get() {
        GameState::LandingScreen => {
            if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::ArrowUp, KeyCode::ArrowLeft, KeyCode::AltRight]) {
                next_state.set(GameState::InGame);
            }
        }
        GameState::InGame => {
            if keyboard_input.pressed(KeyCode::Enter) && old_input.previous_key.is_none(){
                next_state.set(GameState::EndGame);
                old_input.previous_key = Some(KeyCode::Enter);
            }
        }
        GameState::EndGame => {
            event.send_default();
            next_state.set(GameState::Score);
        }
        GameState::Score => {
            if keyboard_input.pressed(KeyCode::Enter) && old_input.previous_key.is_none(){
                restart.send_default();
                next_state.set(GameState::InGame);
                old_input.previous_key = Some(KeyCode::Enter);
            }
        }
    }

    if !keyboard_input.pressed(KeyCode::Enter) {
        old_input.previous_key = None;
    }
}

fn compute_score(
    event: EventReader<ComputeScoreEvent>,
    corn: Query<(&Transform), With<Corn>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut score: ResMut<GameScore>,
) {
    if !event.is_empty() {
        let mut field_map = [[0; GRID_SIZE as usize]; GRID_SIZE as usize];
        for corn_position in corn.iter() {
            field_map[corn_position.translation.x as usize][corn_position.translation.z as usize] = 1;
        }

        score.mistakes = 0;
        score.forgotten = 0;
        let mut correct_pos = 0;
        let mut number_of_tile_to_clear = 0;
        for x in 0..GRID_SIZE as usize {
            for y in 0..GRID_SIZE as usize {
                if LEVEL_1[x][y] == 0 {
                    number_of_tile_to_clear += 1;
                    if field_map[x][y] == 1 {
                        score.forgotten += 1;
                        commands.spawn((PbrBundle {
                            mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                            material: materials.add(Color::srgba(0.0, 0.0, 1.0, 0.4)),
                            transform: Transform::from_xyz(x as f32, 1.1, y as f32),
                            ..default()
                            },
                            ScoreMarker
                        ));
                    }
                } else {
                    if field_map[x][y] == 0 {
                        score.mistakes += 1;
                        commands.spawn((PbrBundle {
                            mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                            material: materials.add(Color::srgba(1.0, 0.0, 0.0, 0.4)),
                            transform: Transform::from_xyz(x as f32, 1.1, y as f32),
                            ..default()
                            },
                            ScoreMarker
                        ));
                    }
                }
            }
        }

        info!("Score: {} forgotten, {} mistakes", score.forgotten, score.mistakes);
    }
}

fn restart_all(
    event: EventReader<RestartGame>,
    mut commands: Commands,
    markers: Query<Entity, With<ScoreMarker>>,
) {
    if !event.is_empty() {
        for marker in markers.iter() {
            commands.entity(marker).despawn();
        }
    }
}