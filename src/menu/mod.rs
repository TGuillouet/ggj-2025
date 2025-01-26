use crate::AppState;
use bevy::prelude::*;

mod systems;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), (systems::spawn_menu,))
            .add_systems(OnExit(AppState::MainMenu), systems::despawn_menu)
            .add_systems(
                Update,
                (systems::handle_play_interaction,).run_if(in_state(AppState::MainMenu)),
            );
    }
}
