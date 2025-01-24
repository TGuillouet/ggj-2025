use bevy::{prelude::*, window::WindowResolution};

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(Sprite::from_image(asset_server.load("bevy_logo_dark.png")));
}

fn main() {
    App::new()
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
        .add_systems(Startup, setup)
        .run();
}
