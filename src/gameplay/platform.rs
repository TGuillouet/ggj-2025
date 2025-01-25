use bevy::{
    color::palettes::css::{GREEN, RED},
    prelude::*,
    render::render_resource::encase::vector::FromVectorParts,
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, GravityScale, RigidBody, Sensor, Velocity};

use super::player::Player;

#[derive(Component)]
// #[require(Transform)]
pub struct Platform;

// #[derive(Component)]
// pub struct Collider(Rect);

#[derive(Bundle)]
pub struct PlatformBundle {
    platform: Platform,
    sprite: Sprite,
    transform: Transform,
    rigidbody: RigidBody,
    collider: Collider,
}

pub fn spawn_platforms(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_handle = asset_server.load("bubble.png");
    let platform = PlatformBundle {
        platform: Platform,
        transform: Transform::from_xyz(0.0, -100.0, 0.0),
        sprite: Sprite::from_image(image_handle),
        rigidbody: RigidBody::KinematicVelocityBased,
        collider: Collider::cuboid(10.0, 10.0),
    };

    commands
        .spawn(platform)
        .insert(ActiveEvents::COLLISION_EVENTS)
        // .insert(Sensor)
        .insert(Velocity {
            linvel: Vec2::from_parts([0.0, -30.0]),
            angvel: 0.0,
        });
}

pub fn handle_player_collision(
    // platforms_query: Query<(&Transform, &Collider, &Sprite), With<Platform>>,
    platforms_query: Query<(&Platform)>,
    // assets: Res<Assets<Image>>,
    mut player_query: Query<(&Transform, &Sprite, &mut Player)>,
    // mut gizmos: Gizmos,
) {
    let Ok((player_transform, player_sprite, mut player)) = player_query.get_single_mut() else {
        return;
    };

    // let player_dimensions = assets.get(&player_sprite.image).unwrap().size_f32();
    // let player_collider =
    //     Rect::from_center_size(player_transform.translation.truncate(), player_dimensions);
    //
    // gizmos.rect_2d(
    //     Isometry2d::from_translation(player_transform.translation.truncate()),
    //     player_collider.size(),
    //     GREEN,
    // );
    //
    // for platform in platforms_query.iter() {
    //     let image_dimensions = assets.get(&platform.2.image).unwrap().size_f32();
    //     let collider_position =
    //         Vec2::from_parts([platform.0.translation.x, platform.0.translation.y + 2.]);
    //     let mut collider_size = platform.0.scale.truncate();
    //     collider_size.x += 1.0;
    //
    //     let collider = Rect::from_center_size(collider_position, image_dimensions * collider_size);
    //
    //     gizmos.rect_2d(
    //         Isometry2d::from_xy(collider_position.x, collider_position.y),
    //         collider.size(),
    //         RED,
    //     );
    //
    //     if !collider.intersect(player_collider).is_empty() {
    //         player.set_grounded(true);
    //     }
    // }
}
