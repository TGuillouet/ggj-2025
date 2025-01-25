use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;

#[derive(Component)]
pub struct Duck(DuckState, Transform);

#[derive(Component)]
pub struct DuckSlot(Timer, Transform);

#[derive(Component)]
pub enum DuckState {
    Init,
    Ready,
}

pub fn spawn_ducks_slots(mut commands: Commands) {
    let slots_pos: [Vec2; 8] = [
        // Up and down
        Vec2::new(-180.0, -180.0), // Bottom Left
        Vec2::new(-180.0, 180.0),  // Top Left
        Vec2::new(180.0, -180.0),  // Bottom Right
        Vec2::new(180.0, 180.0),   // Top Right
        // Right and left
        Vec2::new(-300.0, 100.0),  // Bottom Left
        Vec2::new(300.0, 100.0),   // Top Left
        Vec2::new(300.0, -100.0),  // Bottom Right
        Vec2::new(-300.0, -100.0), // Top Right
    ];

    let duck_spawn_points: [Vec2; 8] = [
        // Up and down
        Vec2::new(-180.0, -220.0), // Bottom Left
        Vec2::new(-180.0, 220.0),  // Top Left
        Vec2::new(180.0, -220.0),  // Bottom Right
        Vec2::new(180.0, 220.0),   // Top Right
        // Right and left
        Vec2::new(-340.0, 100.0),  // Bottom Left
        Vec2::new(340.0, 100.0),   // Top Left
        Vec2::new(340.0, -100.0),  // Bottom Right
        Vec2::new(-340.0, -100.0), // Top Right
    ];

    for (index, slot_pos) in slots_pos.into_iter().enumerate() {
        commands
            .spawn(DuckSlot(
                Timer::from_seconds(1.0, TimerMode::Repeating),
                Transform::from_xyz(duck_spawn_points[index].x, duck_spawn_points[index].y, 0.0),
            ))
            .insert(Transform::from_xyz(slot_pos.x, slot_pos.y, 0.0))
            .insert(Collider::cuboid(10.0, 10.0));
    }
}

pub fn handle_duck_spawning(
    mut commands: Commands,
    mut slots_query: Query<(&mut DuckSlot, &Transform)>,
    time: Res<Time>,
    assets_server: Res<AssetServer>,
) {
    for (mut slot, transform) in slots_query.iter_mut() {
        // Advance the timer
        slot.0.tick(time.delta());

        if !slot.0.just_finished() {
            continue;
        }

        let image = assets_server.load("player.png");
        commands
            .spawn(Duck(DuckState::Init, transform.clone()))
            .insert(Sprite::from_image(image))
            .insert(slot.1)
            .insert(Collider::cuboid(10.0, 10.0));
    }
}

const DUCK_INIT_MOVEMENT: f32 = 5.0;
pub fn move_spawning_ducks(mut duck_query: Query<(&mut Duck, &mut Transform)>, time: Res<Time>) {
    for (mut duck, mut duck_transform) in duck_query.iter_mut() {
        duck_transform.translation = duck_transform.translation.lerp(
            duck.1.translation,
            time.delta_secs() * DUCK_INIT_MOVEMENT, // duck_transform.translation.distance(duck.1.translation),
        );

        // If the duck is at the right place
        if duck_transform.translation == duck.1.translation {
            duck.0 = DuckState::Ready;
        }
    }
}
