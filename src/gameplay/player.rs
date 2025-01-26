use bevy::{prelude::*, render::render_resource::encase::vector::FromVectorParts};
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Collider, CollisionEvent, LockedAxes, RigidBody, Velocity,
};

use super::platform::Platform;

const PLAYER_SPEED: f32 = 100.0;
const JUMP_FORCE: f32 = 250.0;
const FALL_SPEED: f32 = 2.0;

#[derive(Component, Default)]
pub struct Player {
    is_grounded: bool,
    flipped: bool,
}

impl Player {
    pub fn set_grounded(&mut self, new_state: bool) {
        self.is_grounded = new_state;
    }
}

#[derive(Component)]
pub struct IdleAnimation(Timer, usize, usize);

#[derive(Component)]
pub struct JumpAnimation(Timer, usize, usize);

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load("player/player-idle.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Player {
            is_grounded: false,
            flipped: false,
        },
        Transform::from_xyz(0.0, 30.0, 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 10.0),
        Velocity {
            linvel: Vec2::from_parts([0.0, -100.0]),
            angvel: 0.0,
        },
        ActiveCollisionTypes::DYNAMIC_DYNAMIC,
        LockedAxes::ROTATION_LOCKED,
        ActiveEvents::COLLISION_EVENTS,
        Sprite::from_atlas_image(image.clone(), TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: 0,
        }),
        IdleAnimation(Timer::from_seconds(0.2, TimerMode::Repeating), 1, 3),
        Name::new("Player"),
    ));
}

pub fn update_movement(
    mut player_query: Query<(&mut Velocity, &mut Sprite, &mut Player)>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut velocity, mut sprite, mut player)) = player_query.get_single_mut() else {
        return;
    };

    if inputs.just_released(KeyCode::KeyD) || inputs.just_released(KeyCode::KeyA) {
        velocity.linvel.x = 0.0;
    }

    if inputs.pressed(KeyCode::KeyD) {
        velocity.linvel.x = PLAYER_SPEED;
        player.flipped = false;
    }
    if inputs.pressed(KeyCode::KeyA) {
        velocity.linvel.x = -PLAYER_SPEED;
        player.flipped = true;
    }

    if player.is_grounded {
        if inputs.just_pressed(KeyCode::Space) {
            // Apply a force to the player
            velocity.linvel.y += JUMP_FORCE;
        }
    } else {
        if velocity.linvel.y > -150.0 {
            velocity.linvel.y -= FALL_SPEED;
        }
    }

    velocity.linvel.y = velocity.linvel.y.clamp(-150.0, 200.0);
    sprite.flip_x = player.flipped;
}

pub fn animate_player_idle(
    mut animation_query: Query<(&mut IdleAnimation, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut animation, mut sprite) in &mut animation_query {
        animation.0.tick(time.delta());

        if animation.0.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == animation.2 {
                    animation.1
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

pub fn update_visibility(
    mut commands: Commands,
    inputs: Res<ButtonInput<KeyCode>>,
    idle_animation_query: Query<Entity, With<IdleAnimation>>,
    jump_animation_query: Query<Entity, With<JumpAnimation>>,
) {
    let idle_animation_entity = idle_animation_query.single();
    let jump_animation_entity = jump_animation_query.single();

    if inputs.pressed(KeyCode::Space) {
        commands
            .entity(jump_animation_entity)
            .remove::<Visibility>()
            .insert(Visibility::Visible);
        commands
            .entity(idle_animation_entity)
            .remove::<Visibility>()
            .insert(Visibility::Hidden);
    }

    if inputs.just_released(KeyCode::Space) {
        commands
            .entity(idle_animation_entity)
            .remove::<Visibility>()
            .insert(Visibility::Visible);
        commands
            .entity(jump_animation_entity)
            .remove::<Visibility>()
            .insert(Visibility::Hidden);
    }
}

pub fn animate_player_jump(mut animation_query: Query<(&mut JumpAnimation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut animation_query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = animation.2;
        }
    }
}

pub fn update_grounded_flag(
    mut player_query: Query<(Entity, &mut Player)>,
    mut platforms_query: Query<Entity, With<Platform>>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    let (player_entity, mut player) = player_query.single_mut();

    for collision_event in contact_events.read() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _flags) => {
                let is_player = e1 == &player_entity || e2 == &player_entity;

                let mut is_platform = false;
                for platform_entity in platforms_query.iter() {
                    if &platform_entity == e1 || &platform_entity == e2 {
                        is_platform = true;
                    }
                }

                if is_player && is_platform {
                    player.set_grounded(true);
                }
            }
            CollisionEvent::Stopped(e1, e2, _flags) => {
                let is_player = e1 == &player_entity || e2 == &player_entity;

                let mut is_platform = false;
                for platform_entity in platforms_query.iter() {
                    if &platform_entity == e1 || &platform_entity == e2 {
                        is_platform = true;
                    }
                }

                if is_player && is_platform {
                    player.set_grounded(false);
                }
            }
        };
    }
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    player_query.iter().for_each(|item| {
        commands.entity(item).despawn_recursive();
    });
}
