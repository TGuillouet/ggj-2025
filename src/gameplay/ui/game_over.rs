use bevy::prelude::*;

pub fn spawn_menu(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::visible(),
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|children| {
            children.spawn((
                Text::new("Game Over !"),
                TextFont {
                    // This font is loaded and will be used instead of the default font.
                    font: assets_server.load("VT323-Regular.ttf"),
                    font_size: 67.0,
                    ..default()
                },
                // Set the justification of the Text
                TextLayout::new_with_justify(JustifyText::Center),
                Node {
                    width: Val::Percent(100.0),
                    ..default()
                },
            ));
        });
}
