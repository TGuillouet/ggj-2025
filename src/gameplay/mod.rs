use bevy::prelude::*;

mod enemies;
mod platform;
mod player;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(enemies::SpawnTimer::new(3))
            .add_systems(Startup, (player::setup_player, platform::spawn_platforms))
            // .add_systems(Update, ())
            .add_systems(
                FixedUpdate,
                (
                    player::update_movement,
                    player::update_grounded_flag,
                    platform::update_slot_timer,
                    enemies::spawn_ducks_slots,
                    enemies::rotate_duck_toward_player,
                    enemies::shoot,
                    enemies::collide_player_with_projectile,
                ),
            );
    }
}
