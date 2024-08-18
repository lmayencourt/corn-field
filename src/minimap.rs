use bevy::{
    color::palettes::css::{ANTIQUE_WHITE, GRAY, WHITE, YELLOW},
    prelude::*,
};

use crate::{menu::CurrentLevel, world::levels::LEVEL_COUNT};
use crate::menu::GameScore;
use crate::world::levels::LEVELS;
use crate::GameState;

pub struct MinimapPlugin;

#[derive(Component)]
pub struct TextLabel {
    label: i32,
}

//Accepts a `String` or any type that converts into a `String`, such as `&str`

const LABEL_INTRO: i32 = 0;
const LABEL_SCORE: i32 = 1;
const LABEL_LEVEL: i32 = 2;
const LABEL_INDIC: i32 = 3;

// If you add minimap component you cannot add minimap2 component
#[derive(Component)]
pub struct Minimap;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_text);
        app.add_systems(Update, update_minimap);
    }
}

fn update_text(
    mut query: Query<(&mut Visibility, &TextLabel, &mut Text)>,
    state: Res<State<GameState>>,
    score: ResMut<GameScore>,
    current_level: ResMut<CurrentLevel>,
) {
    let level_size = LEVELS[current_level.idx].grid_size as f32;

    for (mut visible, label, mut text) in query.iter_mut() {
        if label.label == LABEL_LEVEL {
            text.sections[1].value = (current_level.idx + 1).to_string().clone();
            text.sections[3].value = level_size.to_string().clone();
            text.sections[5].value = level_size.to_string().clone();
        }
        if label.label == LABEL_SCORE {
            *visible = Visibility::Hidden;
        }

        if *state.get() != GameState::LandingScreen {
            if label.label == LABEL_INTRO {
                *visible = Visibility::Hidden;
            }
        }

        if *state.get() == GameState::LandingScreen {
            if label.label == LABEL_INDIC {
                text.sections[0].value = "> We have an urgent situation on Earth.\n> No time to explain!\n> I need you to create the crop circle in sector 42.".to_string();
            }
        }

        if *state.get() == GameState::InGame {
            if label.label == LABEL_INDIC {
                if current_level.idx == 0 {
                    text.sections[0].value = "> Let start simple.".to_string();
                } else if current_level.idx < (LEVEL_COUNT -1) {
                    text.sections[0].value = " > You are ready to scale up!".to_string();
                } else {
                    text.sections[0].value = " > Last one to go!".to_string();
                }
            }
        }

        if *state.get() == GameState::Score {
            if label.label == LABEL_SCORE {
                *visible = Visibility::Visible;
                text.sections[2].value = score.mistakes.to_string().clone();
                text.sections[4].value = score.forgotten.to_string().clone();
            }
            if label.label == LABEL_INDIC {
                if score.mistakes == 0 && score.forgotten == 0 {
                    text.sections[0].value = " > Excellent!".to_string();
                } else {
                    text.sections[0].value = " > Not perfect but we will make do...".to_string();   
                }
            }
        }
        if *state.get() == GameState::EndGame {
            if label.label == LABEL_SCORE {
                *visible = Visibility::Visible;
                text.sections[2].value = score.mistakes.to_string().clone();
                text.sections[4].value = score.forgotten.to_string().clone();
            }
        }
    }
}

fn update_minimap(
    asset_server: Res<AssetServer>,
    mut query: Query<&mut UiImage, With<Minimap>>,
    current_level: ResMut<CurrentLevel>,
) {
    let mut image = query.single_mut();
    let texture_handle = asset_server.load(LEVELS[current_level.idx].image);

    *image = UiImage::new(texture_handle);
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: ResMut<CurrentLevel>,
) {
    let text_style = TextStyle::default();

    let texture_handle = asset_server.load(LEVELS[current_level.idx].image);

    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                margin: UiRect { left: Val::Px(10.0), right: Val::Px(10.0), top: Val::Px(10.0), bottom: Val::Px(10.0)},
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                row_gap: Val::Px(text_style.font_size * 2.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    style: Style {
                        width: Val::Px(164.),
                        height: Val::Px(164.),
                        position_type: PositionType::Absolute,

                        ..default()
                    },
                    image: UiImage::new(texture_handle),
                    background_color: BackgroundColor(ANTIQUE_WHITE.into()),
                    ..default()
                },
                Minimap,
                Outline::new(Val::Px(5.0), Val::ZERO, GRAY.into()),
            ));
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::from_style(
                
                    TextStyle {
                        color: WHITE.into(),
                        font_size: 24.0,
                        ..default()
                    },
                )
                ]) // Set the justification of the Text
                .with_text_justify(JustifyText::Left)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(40.0),
                    left: Val::Px(200.0),
                    ..default()
                }),
                TextLabel {
                    label: LABEL_INDIC,
                },
            ));
            parent.spawn((
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Arrows: Move           Spacebar: Cut the plants        Enter: Finish the mission",
                    TextStyle {
                        color: WHITE.into(),
                        font_size: 24.0,
                        ..default()
                    },
                ) // Set the justification of the Text
                .with_text_justify(JustifyText::Left)
                // Set the style of the TextBundle itself.
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(200.0),
                    ..default()
                }),
            ));
            parent.spawn((TextBundle::from_sections([
                TextSection::new("Press ".to_string(), text_style.clone()),
                TextSection::new(
                    "any arrows".to_string(),
                    TextStyle {
                        color: YELLOW.into(),
                        ..text_style.clone()
                    },
                ),
                TextSection::new(" to start".to_string(), text_style),
            ])

            .with_text_justify(JustifyText::Center)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                top: Val::Px(350.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }),
            TextLabel {
                label: LABEL_INTRO,
            },
        ));
            parent.spawn((TextBundle::from_sections([
                TextSection::new(
                    "SCORE: \n".to_string(),
                    TextStyle {
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                TextSection::new(
                    "Mistakes: ".to_string(),
                    TextStyle {
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                TextSection::from_style(
                    TextStyle {
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                TextSection::new(
                    "\nForgotten: ".to_string(),
                    TextStyle {
                        color: WHITE.into(),
                        ..default()
                    },
                ),
                TextSection::from_style(
                    TextStyle {
                        color: WHITE.into(),
                        ..default()
                    },
                ),
            ])

            .with_text_justify(JustifyText::Left)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                top: Val::Px(250.0),
                left: Val::Px(5.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }),
            TextLabel {
                label: LABEL_SCORE,
            },
        ));
        parent.spawn((TextBundle::from_sections([
            TextSection::new(
                "Level ".to_string(),
                TextStyle {
                    color: WHITE.into(),
                    ..default()
                },
            ),
            TextSection::from_style(
                TextStyle {
                    color: WHITE.into(),
                    ..default()
                },
            ),
            TextSection::new(
                ": ".to_string(),
                TextStyle {
                    color: WHITE.into(),
                    ..default()
                },
            ),
            TextSection::from_style(
                TextStyle {
                    color: WHITE.into(),
                    ..default()
                },
            ),
            TextSection::new(
                "x".to_string(),
                TextStyle {
                    color: WHITE.into(),
                    ..default()
                },
            ),
            TextSection::from_style(
                TextStyle {
                    color: WHITE.into(),
                    ..default()
                },
            ),
        ])

        .with_text_justify(JustifyText::Left)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            top: Val::Px(180.0),
            left: Val::Px(5.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }),
        TextLabel {
            label: LABEL_LEVEL,
        },
    ));
          });
}
