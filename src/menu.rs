/* SPDX-License-Identifier: MIT
* Copyright (c) 2024 Elieva Pignat, Florian Depraz, Louis Mayencourt
*/

use bevy::prelude::*;

use crate::GameState;
use crate::world::{Corn, YELLOW, GRID_SIZE, levels::LEVEL_1};

/// Global resource that contains the score of the game
#[derive(Resource, Default)]
struct GameScore {
    percent_correct: f32,
    mistakes_nbr: u32,
}

#[derive(Event, Default)]
struct ComputeScoreEvent;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_menu);
        app.add_systems(Update, manage_menu);
        app.add_systems(Update, compute_score);
        app.add_event::<ComputeScoreEvent>();
        app.insert_resource(GameScore::default());
    }
}

fn setup_menu(mut commands: Commands) {
    let text_style = TextStyle { ..default() };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(text_style.font_size * 2.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_sections([
                TextSection::new("press ".to_string(), text_style.clone()),
                TextSection::new(
                    "space".to_string(),
                    TextStyle {
                        color: YELLOW.into(),
                        ..text_style.clone()
                    },
                ),
                TextSection::new(" to mark the field".to_string(), text_style),
            ]));
        });
}

fn manage_menu(
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut event: EventWriter<ComputeScoreEvent>,
)
{
    match state.get() {
        GameState::LandingScreen => {
            if keyboard_input.any_pressed([KeyCode::ArrowDown, KeyCode::ArrowUp, KeyCode::ArrowLeft, KeyCode::AltRight]) {
                next_state.set(GameState::InGame);
            }
        }
        GameState::InGame => {
            if keyboard_input.pressed(KeyCode::Enter) {
                next_state.set(GameState::EndGame);
            }
        }
        GameState::EndGame => {
            info!("End of the game, computing the score");
            event.send_default();
            next_state.set(GameState::Score);
        }
        GameState::Score => {

        }
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

        let mut correct_pos = 0.0;
        for x in 0..GRID_SIZE as usize {
            for y in 0..GRID_SIZE as usize {
                if field_map[x][y] == LEVEL_1[x][y] {
                    correct_pos += 1.0;
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                        material: materials.add(Color::srgba(0.0, 0.0, 1.0, 0.4)),
                        transform: Transform::from_xyz(x as f32, 1.1, y as f32),
                        ..default()
                    });
                } else {
                    commands.spawn(PbrBundle {
                        mesh: meshes.add(Cuboid::new(1.0, 0.2, 1.0)),
                        material: materials.add(Color::srgba(1.0, 0.0, 0.0, 0.4)),
                        transform: Transform::from_xyz(x as f32, 1.1, y as f32),
                        ..default()
                    });
                    println!("x {}, y{} is different", x, y);
                }
            }
        }

        score.percent_correct = correct_pos/(GRID_SIZE*GRID_SIZE) * 100.0;
        score.mistakes_nbr = (GRID_SIZE*GRID_SIZE - correct_pos) as u32;
        info!("Score is {}%, {} mistakes", score.percent_correct, score.mistakes_nbr);
    }
}