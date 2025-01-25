use bevy::prelude::*;

pub struct UiPlugin;

mod game_over;
mod win;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(super::ScreenState::Finished), (win::spawn_menu,))
            .add_systems(OnExit(super::ScreenState::Finished), (win::despawn_menu,))
            .add_systems(
                OnEnter(super::ScreenState::GameOver),
                (game_over::spawn_menu,),
            )
            .add_systems(
                OnExit(super::ScreenState::GameOver),
                (game_over::despawn_menu,),
            )
            .add_systems(
                Update,
                (game_over::return_to_main_menu, win::return_to_main_menu),
            );
    }
}
