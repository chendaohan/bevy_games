use std::time::Duration;

use bevy::{
    color::palettes::css,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::common_conditions::on_timer,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::default()))
        .add_systems(Startup, (spawn_camera, spawn_shapes))
        .add_systems(Update, transform_red_rectangle)
        .add_systems(
            PostUpdate,
            circle_global_transform_info
                .run_if(on_timer(Duration::from_secs(1)))
                .after(TransformSystem::TransformPropagate),
        )
        .run()
}

#[derive(Component)]
struct OrangeCircle;

#[derive(Component)]
struct RedRectangle;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_shapes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 橙色圆形
    let circle = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(50.))),
                material: materials.add(Color::Srgba(css::ORANGE)),
                transform: Transform::from_xyz(220., 0., 0.),
                ..default()
            },
            OrangeCircle,
            Name::new("Orange Circle"),
        ))
        .id();

    // 红色矩形
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(300., 150.))),
                material: materials.add(Color::Srgba(css::RED)),
                transform: Transform::from_xyz(0., 185., 0.),
                ..default()
            },
            RedRectangle,
            Name::new("Red Rectangle"),
        ))
        .add_child(circle);

    // 蓝色 2D 胶囊
    commands.spawn((
        Mesh2dHandle(meshes.add(Capsule2d::new(40., 100.))),
        materials.add(Color::Srgba(css::BLUE)),
        TransformBundle {
            local: Transform::from_xyz(0., -200., 0.),
            global: GlobalTransform::default(),
        },
        VisibilityBundle::default(),
        Name::new("Blue Capsule 2d"),
    ));
}

fn transform_red_rectangle(
    mut right: Local<bool>,
    mut rectangle: Query<&mut Transform, With<RedRectangle>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = rectangle.get_single_mut() else {
        return;
    };

    // 平移方向
    if transform.translation.x < -300. {
        *right = false;
    } else if transform.translation.x > 300. {
        *right = true;
    }

    // 平移和缩放
    let velocity = Dir3::X * 200. * time.delta_seconds();
    let scale = time.delta_seconds() / 3.;
    if *right {
        transform.translation -= velocity;
        transform.scale -= scale;
    } else {
        transform.translation += velocity;
        transform.scale += scale;
    }

    // 旋转
    transform.rotate_z(time.delta_seconds());
}

fn circle_global_transform_info(circle: Query<&GlobalTransform, With<OrangeCircle>>) {
    let Ok(transform) = circle.get_single() else {
        return;
    };
    let (scale, rotation, translation) = transform.to_scale_rotation_translation();
    info!("Orange Circle scale: {scale}, roatation: {rotation}, translation: {translation}");
}
