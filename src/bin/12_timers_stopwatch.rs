use bevy::{
    color::palettes::css,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::Stopwatch,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                (tick_timers, tick_stopwatches),
                (
                    spawn_small_circle,
                    move_small_circles,
                    despawn_small_circles,
                ),
            )
                .chain(),
        )
        .run()
}

const SMALL_CIRCLES_COUNT: usize = 16;

#[derive(Component)]
struct BigCircleTimer(Timer);

#[derive(Component)]
struct SmallCircleStopwatch(Stopwatch);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(120.))),
            material: materials.add(Color::Srgba(css::RED)),
            ..default()
        },
        BigCircleTimer(Timer::from_seconds(1., TimerMode::Repeating)),
        Name::new("Red Circle"),
    ));
}

fn spawn_small_circle(
    mut commands: Commands,
    big_circle_timers: Query<(Entity, &BigCircleTimer)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, BigCircleTimer(timer)) in &big_circle_timers {
        if timer.just_finished() {
            let small_circle_handle = Mesh2dHandle(meshes.add(Circle::new(30.)));
            for index in 0..SMALL_CIRCLES_COUNT {
                let mut transform = Transform::from_translation(Vec3::X * 160.);
                transform.translate_around(
                    Vec3::ZERO,
                    Quat::from_rotation_z(std::f32::consts::TAU * index as f32 / SMALL_CIRCLES_COUNT as f32),
                );
                let small_circle = commands
                    .spawn((
                        MaterialMesh2dBundle {
                            mesh: small_circle_handle.clone(),
                            material: materials.add(Color::Srgba(Srgba::interpolate(
                                &css::RED,
                                &css::BLUE,
                                index as f32 / SMALL_CIRCLES_COUNT as f32,
                            ))),
                            transform,
                            ..default()
                        },
                        SmallCircleStopwatch(Stopwatch::new()),
                        Name::new("Small Circle"),
                    ))
                    .id();
                commands.entity(entity).add_child(small_circle);
            }
        }
    }
}

fn tick_timers(mut timers: Query<&mut BigCircleTimer>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.0.tick(time.delta());
    }
}

fn tick_stopwatches(mut stopwatches: Query<&mut SmallCircleStopwatch>, time: Res<Time>) {
    for mut stopwatch in &mut stopwatches {
        stopwatch.0.tick(time.delta());
    }
}

fn move_small_circles(
    mut small_circles: Query<&mut Transform, With<SmallCircleStopwatch>>,
    time: Res<Time>,
) {
    for mut transform in &mut small_circles {
        let direction = transform.translation.normalize();
        transform.translation += direction * 200. * time.delta_seconds();
    }
}

fn despawn_small_circles(
    mut commands: Commands,
    small_circles: Query<(Entity, &SmallCircleStopwatch)>,
) {
    for (entity, SmallCircleStopwatch(stopwatch)) in &small_circles {
        if stopwatch.elapsed_secs() > 3. {
            commands.entity(entity).despawn();
        }
    }
}
