use bevy::{
    input::{
        common_conditions::{input_just_pressed, input_pressed},
        mouse::{MouseMotion, MouseScrollUnit, MouseWheel},
    },
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                toggle_color.run_if(input_just_pressed(MouseButton::Left)),
                move_camera.run_if(input_pressed(MouseButton::Middle)),
                scroll_camera,
                cursor_circle,
            ),
        )
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mesh_handle = Mesh2dHandle(meshes.add(Rectangle::new(80., 80.)));
    for x in -10..10 {
        for y in -10..10 {
            commands.spawn(MaterialMesh2dBundle {
                mesh: mesh_handle.clone(),
                material: materials.add(Color::Srgba(Srgba::interpolate(
                    &Srgba::interpolate(&Srgba::RED, &Srgba::BLUE, (x + 10) as f32 / 20.),
                    &Srgba::GREEN,
                    (y + 10) as f32 / 20.,
                ))),
                transform: Transform::from_xyz(x as f32 * 100., y as f32 * 100., 0.),
                ..default()
            });
        }
    }
}

fn toggle_color(
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    squares: Query<(&Handle<ColorMaterial>, &GlobalTransform)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (camera, camera_transform) = camera.single();
    let Some(cursor_position) = window.single().cursor_position() else {
        return;
    };
    let Some(cursor_position) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    else {
        return;
    };
    for (square_color_handle, square_transform) in &squares {
        let translation = square_transform.translation();
        if cursor_position.x > translation.x - 40.
            && cursor_position.x < translation.x + 40.
            && cursor_position.y > translation.y - 40.
            && cursor_position.y < translation.y + 40.
        {
            let Some(material) = materials.get_mut(square_color_handle) else {
                continue;
            };
            let hsva = Hsva::from(material.color);
            material.color = Color::Hsva(hsva.with_hue((hsva.hue + 180.) % 360.));
        }
    }
}

fn move_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut motion_reader: EventReader<MouseMotion>,
) {
    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };
    for motion in motion_reader.read() {
        camera_transform.translation += Vec3::new(-motion.delta.x, motion.delta.y, 0.);
    }
}

fn scroll_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    mut wheel_reader: EventReader<MouseWheel>,
) {
    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };
    for wheel in wheel_reader.read() {
        let unit = if let MouseScrollUnit::Line = wheel.unit {
            35.
        } else {
            1.
        };
        camera_transform.translation += Vec3::new(0., wheel.y * unit, 0.);
    }
}

fn cursor_circle(
    camera: Query<(&Camera, &GlobalTransform)>,
    mut gizmo: Gizmos,
    mut moved_reader: EventReader<CursorMoved>,
) {
    let Ok((camera, camera_transform)) = camera.get_single() else {
        return;
    };
    for moved in moved_reader.read() {
        let Some(point) = camera.viewport_to_world_2d(camera_transform, moved.position) else {
            continue;
        };
        gizmo.circle_2d(point, 20., Color::WHITE);
    }
}
