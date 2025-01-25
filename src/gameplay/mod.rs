use bevy::prelude::*;

mod platform;
mod player;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player::setup_player, platform::spawn_platforms))
            .add_systems(
                FixedUpdate,
                (
                    player::update_movement,
                    platform::handle_player_collision,
                    player::update_grounded_flag,
                ),
            );
    }
}
