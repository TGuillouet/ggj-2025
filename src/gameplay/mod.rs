use bevy::prelude::*;

mod enemies;
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
                    player::update_grounded_flag,
                    platform::update_slot_timer,
                ),
            );
    }
}
