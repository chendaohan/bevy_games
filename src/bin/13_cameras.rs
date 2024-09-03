use bevy::{
    color::palettes::css, input::common_conditions::input_just_pressed, prelude::*,
    render::view::RenderLayers,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_blendy_cameras::BlendyCamerasPlugin)
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                scale_camera.run_if(input_just_pressed(KeyCode::KeyS)),
                toggle_perspective_orthographic.run_if(input_just_pressed(KeyCode::KeyP)),
                toggle_render_layers.run_if(input_just_pressed(KeyCode::KeyR)),
                toggle_camera_active.run_if(input_just_pressed(KeyCode::KeyC)),
            ),
        )
        .run()
}

#[derive(Component)]
struct MyCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(15., 20., 30.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        bevy_blendy_cameras::OrbitCameraController::default(),
        MyCamera,
        Name::new("My Camera"),
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(1., 2., 1.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Directional Light"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(50., 1., 50.)),
            material: materials.add(Color::Srgba(css::ORANGE)),
            ..default()
        },
        RenderLayers::layer(0),
        Name::new("Ground"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(5.)),
            material: materials.add(Color::Srgba(css::BLUE)),
            transform: Transform::from_xyz(10., 5., 0.),
            ..default()
        },
        RenderLayers::layer(0),
        Name::new("Blue Sphere"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(10., 10., 10.)),
            material: materials.add(Color::Srgba(css::RED)),
            transform: Transform::from_xyz(-10., 5., 0.),
            ..default()
        },
        Name::new("Red Cube"),
    ));
}

fn scale_camera(mut projection: Query<&mut Projection, With<MyCamera>>) {
    let Ok(projection) = projection.get_single_mut() else {
        return;
    };
    match projection.into_inner() {
        Projection::Orthographic(projection) => {
            if (projection.scale - 0.15).abs() <= f32::EPSILON {
                projection.scale = 0.05;
            } else {
                projection.scale = 0.15;
            }
        }
        Projection::Perspective(projection) => {
            if (projection.fov - 0.785).abs() <= f32::EPSILON {
                projection.fov = std::f32::consts::PI / 2.;
            } else {
                projection.fov = 0.785;
            }
        }
    }
}

fn toggle_perspective_orthographic(mut projection: Query<&mut Projection, With<MyCamera>>) {
    let Ok(mut projection) = projection.get_single_mut() else {
        return;
    };
    if let Projection::Perspective(_) = projection.as_ref() {
        *projection = Projection::Orthographic(OrthographicProjection {
            scale: 0.15,
            ..default()
        });
    } else {
        *projection = Projection::Perspective(PerspectiveProjection::default());
    }
}

fn toggle_render_layers(mut render_layers: Query<&mut RenderLayers>) {
    for mut render_layer in &mut render_layers {
        if render_layer.iter().next().unwrap() == 0 {
            *render_layer = RenderLayers::layer(1);
        } else {
            *render_layer = RenderLayers::layer(0);
        }
    }
}

fn toggle_camera_active(mut camera: Query<&mut Camera, With<MyCamera>>) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    if camera.is_active {
        camera.is_active = false;
    } else {
        camera.is_active = true;
    }
}
