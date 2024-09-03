use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());    

    commands.spawn(SpriteBundle {
        texture: asset_server.load("14_load_assets/bevy_bird_dark.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("14_load_assets/bevy_bird_dark.png"),
        transform: Transform::from_xyz(-200., 0., 0.),
        ..default()
    });
}