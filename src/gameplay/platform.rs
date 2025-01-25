use std::time::Duration;

use bevy::{prelude::*, render::render_resource::encase::vector::FromVectorParts};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, RigidBody, Velocity};
use rand::Rng;

#[derive(Component)]
pub struct Platform;

#[derive(Bundle)]
pub struct PlatformBundle {
    platform: Platform,
    sprite: Sprite,
    transform: Transform,
    rigidbody: RigidBody,
    collider: Collider,
    active_event: ActiveEvents,
    velocity: Velocity,
}

#[derive(Component)]
pub struct BubbleSlot(Timer);

pub fn spawn_platforms(mut commands: Commands, assets_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let slots_y = 200.0;
    let slots_x: [f32; 7] = [-300.0, -200.0, -100.0, 0.0, 100.0, 200.0, 300.0];

    for slot_x in slots_x.into_iter() {
        commands
            .spawn(BubbleSlot(Timer::new(
                Duration::from_millis(rng.gen_range(0.0..2000.0) as u64),
                TimerMode::Once,
            )))
            .insert(Transform::from_xyz(slot_x, slots_y, 0.0));

        let platform = create_platform(Transform::from_xyz(slot_x, 0.0, 0.0), &assets_server);
        commands.spawn(platform);
    }
}

pub fn update_slot_timer(
    mut slots_query: Query<(&mut BubbleSlot, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    delta_time: Res<Time>,
) {
    for (mut slot, transform) in slots_query.iter_mut() {
        slot.0.tick(delta_time.delta());
        if slot.0.just_finished() {
            let platform = create_platform(*transform, &asset_server);

            commands.spawn(platform);

            slot.0.set_duration(Duration::from_secs_f32(4.0));
            slot.0.reset();
        }
    }
}

fn create_platform(transform: Transform, assets_server: &Res<AssetServer>) -> PlatformBundle {
    let image_handle = assets_server.load("bubble.png");
    PlatformBundle {
        platform: Platform,
        transform,
        sprite: Sprite::from_image(image_handle),
        rigidbody: RigidBody::KinematicVelocityBased,
        collider: Collider::cuboid(10.0, 10.0),
        active_event: ActiveEvents::COLLISION_EVENTS,
        velocity: Velocity {
            linvel: Vec2::from_parts([0.0, -30.0]),
            angvel: 0.0,
        },
    }
}
