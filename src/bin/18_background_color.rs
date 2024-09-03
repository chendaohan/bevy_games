use bevy::{color::palettes::tailwind, prelude::*, render::camera::Viewport};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::Srgba(tailwind::GREEN_300)))
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        camera: Camera {
            viewport: Some(Viewport {
                physical_size: UVec2::new(1280 / 2, 720),
                ..default()
            }),
            clear_color: ClearColorConfig::Custom(Color::Srgba(tailwind::BLUE_300)),
            ..default()
        },
        transform: Transform::from_xyz(89., 140., 14.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        camera: Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(1280 / 2, 0),
                physical_size: UVec2::new(1280 / 2, 720),
                ..default()
            }),
            order: 1,
            clear_color: ClearColorConfig::Custom(Color::Srgba(tailwind::RED_300)),
            ..default()
        },
        transform: Transform::from_xyz(0., 140., 106.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1., 2., 1.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("18_background/Fox.glb#Scene0"),
        ..default()
    });
}
