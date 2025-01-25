use bevy::prelude::*;
use events::PlayerLostEvent;

mod enemies;
mod events;
mod platform;
mod player;
mod state;
mod ui;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum ScreenState {
    #[default]
    InGame,
    GameOver,
    Finished,
}

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(enemies::SpawnTimer::new(3))
            .insert_state(ScreenState::default())
            .add_event::<PlayerLostEvent>()
            .add_systems(
                OnEnter(ScreenState::InGame),
                (player::setup_player, platform::spawn_platforms),
            )
            .add_systems(
                OnExit(ScreenState::InGame),
                (
                    player::despawn_player,
                    platform::despawn_platforms,
                    enemies::despawn_enemies,
                ),
            )
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
                )
                    .run_if(in_state(ScreenState::InGame)),
            );
    }
}
