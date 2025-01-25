use std::time::Duration;

use bevy::{math::NormedVectorSpace, prelude::*};
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, Collider, ExternalForce, RigidBody, Sensor, Velocity,
};
use rand::Rng;

use super::player::Player;

#[derive(Component)]
pub struct Duck(DuckState, Vec2, Timer);

#[derive(Component)]
pub struct DuckSlot(Timer, Vec2);

#[derive(Component, Debug)]
pub enum DuckState {
    Init,
    Ready,
    Shooting,
}

#[derive(Component)]
pub struct Projectile;

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
        commands.spawn((
            DuckSlot(
                Timer::from_seconds(1.0, TimerMode::Repeating),
                // Transform::from_xyz(duck_spawn_points[index].x, duck_spawn_points[index].y, 0.0),
                duck_spawn_points[index],
            ),
            Transform::from_xyz(slot_pos.x, slot_pos.y, 0.0),
            Name::new(format!("Slot {}", index + 1)),
        ));
    }
}

pub fn handle_duck_spawning(
    mut commands: Commands,
    mut slots_query: Query<(&Name, &mut DuckSlot, &Transform)>,
    time: Res<Time>,
    assets_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    for (name, mut slot, transform) in slots_query.iter_mut() {
        // Advance the timer
        slot.0.tick(time.delta());

        if !slot.0.finished() {
            continue;
        }

        let image = assets_server.load("player.png");
        let duration = Duration::from_millis(rng.gen_range(1000..2000));
        commands.spawn((
            Duck(
                DuckState::Init,
                transform.translation.xy(),
                Timer::new(duration, TimerMode::Once),
            ),
            Sprite::from_image(image),
            transform.clone(),
            // Transform::from_xyz(slot.1.x, slot.1.y, 0.0),
            Collider::cuboid(10.0, 10.0),
        ));
    }
}

const DUCK_INIT_MOVEMENT: f32 = 5.0;
pub fn move_spawning_ducks(mut duck_query: Query<(&mut Duck, &mut Transform)>, time: Res<Time>) {
    for (mut duck, mut duck_transform) in duck_query.iter_mut() {
        // If the duck is at the right place
        if let DuckState::Init = duck.0 {
            let final_position = duck.1.extend(0.0);
            duck_transform.translation = duck_transform.translation.move_towards(
                final_position,
                duck_transform.translation.distance(final_position), // duck_transform.translation.distance(duck.1.translation),
            );

            if duck_transform.translation == final_position {
                duck.0 = DuckState::Ready;
            }
        }
    }
}

const PROJECTILE_FORCE: f32 = 300.0;
pub fn handle_shooting(
    mut commands: Commands,
    mut ducks_query: Query<(&mut Duck, &mut Transform, Entity)>,
    player_query: Query<(&Transform, &Player), Without<Duck>>,
    time: Res<Time<Fixed>>,
) {
    let (player_transform, _player) = player_query.single();
    for (mut duck, mut duck_transform, entity) in ducks_query.iter_mut() {
        match duck.0 {
            DuckState::Ready => {
                // Target the player at this instant
                duck_transform.look_at(player_transform.translation, Vec3::Z);

                // Setup the shooting timer
                duck.0 = DuckState::Shooting;
            }
            DuckState::Shooting => {
                // Update the timer
                duck.2.tick(time.delta());

                if !duck.2.finished() {
                    return;
                }

                let projectile_transform = duck_transform.clone();

                // Shoot
                commands.spawn((
                    Projectile,
                    Velocity {
                        linvel: projectile_transform.forward().truncate() * PROJECTILE_FORCE,
                        ..default()
                    },
                    RigidBody::Dynamic,
                    Sensor,
                    ActiveCollisionTypes::DYNAMIC_DYNAMIC,
                    projectile_transform,
                    Collider::cuboid(5.0, 5.0),
                ));

                // Despawn the duck
                commands.entity(entity).despawn();
            }
            _ => {}
        }
    }
}
