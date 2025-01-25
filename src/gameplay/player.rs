use bevy::{prelude::*, render::render_resource::encase::vector::FromVectorParts};
use bevy_rapier2d::prelude::{
    ActiveCollisionTypes, ActiveEvents, Collider, CollisionEvent, LockedAxes, RigidBody, Velocity,
};

const PLAYER_SPEED: f32 = 100.0;
const JUMP_FORCE: f32 = 250.0;

#[derive(Component)]
pub struct Player {
    is_grounded: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self { is_grounded: false }
    }
}

impl Player {
    pub fn set_grounded(&mut self, new_state: bool) {
        self.is_grounded = new_state;
    }
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sprite = Sprite::from_image(asset_server.load("player.png"));
    commands.spawn((
        Player { is_grounded: false },
        sprite,
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
    ));
}

pub fn update_movement(
    mut player_query: Query<(&mut Velocity, &Player)>,
    inputs: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut velocity, player)) = player_query.get_single_mut() else {
        return;
    };

    if inputs.just_released(KeyCode::KeyD) || inputs.just_released(KeyCode::KeyA) {
        velocity.linvel.x = 0.0;
    }

    if inputs.pressed(KeyCode::KeyD) {
        velocity.linvel.x = PLAYER_SPEED;
    }
    if inputs.pressed(KeyCode::KeyA) {
        velocity.linvel.x = -PLAYER_SPEED;
    }

    if inputs.pressed(KeyCode::Space) && player.is_grounded {
        // Apply a force to the player
        velocity.linvel.y = JUMP_FORCE;
    }

    if !player.is_grounded && velocity.linvel.y > -100.0 {
        velocity.linvel.y -= 2.0;
    }

    if velocity.linvel.y < -100.0 {
        velocity.linvel.y = -100.0;
    }
}

pub fn update_grounded_flag(
    mut player_query: Query<(Entity, &mut Player)>,
    mut contact_events: EventReader<CollisionEvent>,
) {
    let (player_entity, mut player) = player_query.single_mut();

    for collision_event in contact_events.read() {
        let is_grounded = match collision_event {
            CollisionEvent::Started(e1, e2, _flags) => e1 == &player_entity || e2 == &player_entity,
            CollisionEvent::Stopped(e1, e2, _flags) => {
                !(e1 == &player_entity || e2 == &player_entity)
            }
        };
        player.set_grounded(is_grounded);
    }
}
