use bevy::prelude::*;

use crate::AppState;

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

#[derive(Resource)]
struct WinTimer(Timer);

fn reset_win_timer(mut win_timer: ResMut<WinTimer>) {
    win_timer.0.reset();
}

fn update_win_timer(
    mut win_timer: ResMut<WinTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<ScreenState>>,
) {
    win_timer.0.tick(time.delta());

    if !win_timer.0.finished() {
        return;
    }

    next_state.set(ScreenState::Finished);
}

pub struct GameplayPlugin;
impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(enemies::SpawnTimer::new(3))
            .insert_resource(WinTimer(Timer::from_seconds(5.0, TimerMode::Once)))
            .insert_state(ScreenState::default())
            .add_event::<events::WinEvent>()
            .add_plugins(ui::UiPlugin)
            .add_systems(
                OnEnter(AppState::Game),
                (
                    reset_win_timer,
                    player::setup_player,
                    platform::spawn_platforms,
                )
                    .run_if(in_state(ScreenState::InGame))
                    .run_if(in_state(AppState::Game)),
            )
            .add_systems(
                OnExit(ScreenState::InGame),
                (
                    player::despawn_player,
                    platform::despawn_platforms,
                    enemies::despawn_enemies,
                )
                    .run_if(in_state(AppState::Game)),
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
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(ScreenState::InGame)),
            )
            .add_systems(
                Update,
                (update_win_timer,)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(ScreenState::InGame)),
            );
    }
}
