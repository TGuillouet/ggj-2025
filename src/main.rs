use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierConfiguration, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

mod gameplay;

fn setup(mut commands: Commands, mut rapier_config: Query<&mut RapierConfiguration>) {
    let mut rapier_config = rapier_config.single_mut();
    rapier_config.gravity = Vec2::ZERO;
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera,
        Projection::from(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: 1600.0,
                height: 900.0,
            },
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::linear_rgb(0.3, 0.3, 0.3)))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "GGJ2025".to_string(),
                        resolution: WindowResolution::new(1600.0, 900.0),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(2.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_plugins(gameplay::GameplayPlugin)
        .run();
}
