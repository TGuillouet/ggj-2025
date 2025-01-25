use std::time::Duration;

use bevy::{
    color::palettes::css::{BLACK, RED},
    prelude::*,
};
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Collider, RigidBody, Sensor, Velocity,
};
use rand::Rng;

use super::player::Player;

#[derive(Component)]
pub struct Duck;

#[derive(Component)]
pub struct ShootingTimer(Timer);

#[derive(Component)]
pub struct Projectile;

#[derive(Resource)]
pub struct SpawnTimer(Timer, bool);
impl SpawnTimer {
    pub fn new(secs: u64) -> Self {
        Self(
            Timer::new(Duration::from_secs(secs), TimerMode::Once),
            false,
        )
    }
}

pub fn spawn_ducks_slots(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    let rng = rand::thread_rng().gen_range(-200.0..200.0);
    let x = 300.0 * (if spawn_timer.1 { 1.0 } else { -1.0 });

    spawn_timer
        .0
        .tick(Duration::from_secs_f64(time.delta_secs_f64()));

    if !spawn_timer.0.finished() {
        return;
    }

    spawn_timer.1 = !spawn_timer.1;

    commands.spawn((
        Duck,
        Transform::from_xyz(x, rng, 0.0),
        Collider::cuboid(10.0, 10.0),
        Sprite::from_color(BLACK, Vec2::splat(10.0)),
        ShootingTimer(Timer::from_seconds(1.0, TimerMode::Once)),
    ));

    spawn_timer.0.reset();
}

pub fn rotate_duck_toward_player(
    player_query: Query<&Transform, With<Player>>,
    mut ducks_query: Query<(&mut Transform, &Duck), Without<Player>>,
) {
    let player_transform = player_query.single();

    for (mut duck_transform, _duck) in ducks_query.iter_mut() {
        let diff = player_transform.translation - duck_transform.translation;
        let angle = diff.y.atan2(diff.x);
        duck_transform.rotation = Quat::from_axis_angle(Vec3::Z, angle);
    }
}

pub fn shoot(
    mut commands: Commands,
    mut ducks_query: Query<(Entity, &mut ShootingTimer, &Transform), With<Duck>>,
    time: Res<Time>,
) {
    for (entity, mut shooting_timer, duck_transform) in ducks_query.iter_mut() {
        shooting_timer.0.tick(time.delta());

        if !shooting_timer.0.finished() {
            continue;
        }

        // Shoot
        commands.spawn((
            Projectile,
            Sprite::from_color(RED, Vec2::splat(16.0)),
            RigidBody::KinematicVelocityBased,
            Velocity {
                linvel: duck_transform.clone().forward().xy() * 300.0,
                ..default()
            },
            duck_transform.clone(),
            ActiveEvents::COLLISION_EVENTS,
            Sensor,
            ActiveCollisionTypes::DYNAMIC_DYNAMIC,
            Collider::cuboid(10.0, 10.0),
        ));

        commands.entity(entity).despawn();
    }
}
