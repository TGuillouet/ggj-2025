use bevy::{ecs::system::SystemParam, input::common_conditions::input_pressed, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::{BevyPhysicsHooks, PairFilterContextView, SolverFlags};

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
            .insert_resource(WinTimer(Timer::from_seconds(30.0, TimerMode::Once)))
            .insert_state(ScreenState::default())
            .add_event::<events::WinEvent>()
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(ui::UiPlugin)
            //
            // Startup systems
            //
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
                OnEnter(ScreenState::InGame),
                (
                    reset_win_timer,
                    player::setup_player,
                    platform::spawn_platforms,
                )
                    .run_if(in_state(ScreenState::InGame))
                    .run_if(in_state(AppState::Game)),
            )
            //
            // Cleanup systems
            //
            .add_systems(
                OnExit(ScreenState::InGame),
                (
                    player::despawn_player,
                    platform::despawn_platforms,
                    enemies::despawn_enemies,
                )
                    .run_if(in_state(AppState::Game)),
            )
            //
            // Update systems
            //
            .add_systems(
                FixedUpdate,
                (
                    player::update_movement.after(player::update_grounded_flag),
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
                (
                    update_win_timer,
                    player::animate_player_idle.run_if(not(input_pressed(KeyCode::Space))),
                    // player::animate_player_jump.run_if(input_pressed(KeyCode::Space)),
                    // player::update_visibility,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(ScreenState::InGame)),
            );
    }
}
