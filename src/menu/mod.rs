use crate::AppState;
use bevy::prelude::*;

mod systems;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), (systems::spawn_menu,));
    }
}
