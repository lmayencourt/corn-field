use bevy::{color::palettes::css::{ANTIQUE_WHITE, WHITE, GRAY, YELLOW}, prelude::*};

use crate::GameState;
use crate::world::levels::LEVELS;
use crate::menu::CurrentLevel;

pub struct MinimapPlugin;

#[derive(Component)]
pub struct TextIntro;

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

fn update_text(mut query: Query<&mut Visibility, With<TextIntro>>, state: Res<State<GameState>>) {
    if *state.get() != GameState::LandingScreen {
        let mut visible = query.single_mut();
        *visible = Visibility::Hidden;
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
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "> We have an urgent situation on Earth.\n> I need you to create the crop circle in sector 42.",
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
            parent.spawn((
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "> We have an urgent situation on Earth.\n> I need you to create the crop circle in sector 42.",
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
            parent.spawn((
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Arrows:      Move\nSpacebar:    Cut the plants\nEnter:       Finish the mission",
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
                    top: Val::Px(100.0),
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
            TextIntro {}
            )
            );
        });
}
