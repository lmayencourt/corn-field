use bevy::{color::palettes::css::{ANTIQUE_WHITE, WHITE}, prelude::*};


pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let text_style = TextStyle::default();

    let texture_handle = asset_server.load("pixil-frame-0.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::splat(16), 1, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

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
                TextureAtlas::from(texture_atlas_handle),
                Outline::new(Val::Px(5.0), Val::ZERO, WHITE.into()),
            ));
            parent.spawn((
                TextBundle::from_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "> We have an urgent situation on Earth.\n> I need you to create the crop circle in sector 42.\n> Be precise!\n\nArrows: Move\nSpace: Cut the plants\nEnter: Finish mission",
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
                    right: Val::Px(100.0),
                    ..default()
                }),
            ));
        });
}
