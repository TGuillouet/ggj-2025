use bevy::prelude::*;

use crate::gameplay::ScreenState;

#[derive(Component)]
pub struct GameOverMenu;

pub fn spawn_menu(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn((
            GameOverMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                overflow: Overflow::visible(),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Percent(15.0),
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

            children.spawn((
                Text::new("Appuyez sur une touche pour continuer !"),
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
                    ..default()
                },
            ));
        });
}

pub fn return_to_main_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<ScreenState>>,
) {
    if !keyboard_input.any_pressed([KeyCode::Space, KeyCode::Enter]) {
        return;
    }

    next_state.set(ScreenState::InGame);
}

pub fn despawn_menu(mut commands: Commands, menu_query: Query<Entity, With<GameOverMenu>>) {
    let menu = menu_query.single();
    commands.entity(menu).despawn_recursive();
}
