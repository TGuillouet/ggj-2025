use bevy::prelude::*;

pub fn spawn_menu(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Percent(15.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
        ))
        .with_children(|children| {
            children.spawn((
                Text::new("BUBBLE JUMP"),
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

            children
                .spawn(Node {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    ..default()
                })
                .with_children(|children| {
                    children.spawn((
                        ImageNode::new(assets_server.load("button.png")),
                        Text::new("Jouer"),
                        TextFont {
                            // This font is loaded and will be used instead of the default font.
                            font: assets_server.load("VT323-Regular.ttf"),
                            font_size: 30.0,
                            ..default()
                        },
                        // Set the justification of the Text
                        TextLayout::new_with_justify(JustifyText::Center),
                        Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                    ));
                });
        });
}
