use bevy::prelude::*;

pub struct UiPlugin;

mod game_over;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(super::ScreenState::GameOver),
            (game_over::spawn_menu,),
        );
    }
}
