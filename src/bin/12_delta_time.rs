use bevy::{
    color::palettes::css,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() -> AppExit {
    App::new()
        .add_plugins(
            DefaultPlugins
                // .set(WindowPlugin {
                //     primary_window: Some(Window {
                //         present_mode: bevy::window::PresentMode::AutoNoVsync,
                //         ..default()
                //     }),
                //     ..default()
                // })
                ,
        )
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (move_red_circle, move_blue_circle))
        .run()
}

#[derive(Component)]
struct RedCircle;

#[derive(Component)]
struct BlueCircle;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(150.))),
            material: materials.add(Color::Srgba(css::RED)),
            transform: Transform::from_xyz(-450., 175., 0.),
            ..default()
        },
        RedCircle,
        Name::new("Red Circle"),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(150.))),
            material: materials.add(Color::Srgba(css::BLUE)),
            transform: Transform::from_xyz(-450., -175., 0.),
            ..default()
        },
        BlueCircle,
        Name::new("Blue Circle"),
    ));
}

fn move_red_circle(mut red_circle: Query<&mut Transform, With<RedCircle>>) {
    let Ok(mut transform) = red_circle.get_single_mut() else {
        return;
    };
    transform.translation.x += 200. / 60.;
}

fn move_blue_circle(mut blue_circle: Query<&mut Transform, With<BlueCircle>>, time: Res<Time>) {
    let Ok(mut transform) = blue_circle.get_single_mut() else {
        return;
    };
    transform.translation.x += 200. * time.delta_seconds();
}
