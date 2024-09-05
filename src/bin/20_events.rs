use bevy::{
    color::palettes::tailwind,
    math::bounding::{Aabb3d, BoundingSphere, IntersectsVolume},
    prelude::*,
};
use bevy_blendy_cameras::{BlendyCamerasPlugin, OrbitCameraController};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_billboard::prelude::*;
use rand::{distributions::Distribution, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::default(),
            BlendyCamerasPlugin,
            BillboardPlugin,
        ))
        .add_event::<AddExperience>()
        .add_event::<PlayerUpgrade>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                (detect_collision, add_experiences).chain(),
                move_player,
                alter_level_text,
                spawn_upgrade_text,
            ),
        )
        .add_systems(Update, despawn_upgrade_text)
        .run()
}

#[derive(Component)]
struct Player {
    level: u8,
    upgrade_experience: [u32; 20],
    current_experience: u32,
}

#[derive(Component)]
struct ExperienceValue(u32);

#[derive(Component)]
struct LevelText;

#[derive(Component)]
struct UpgradeText {
    timer: Timer,
}

#[derive(Event)]
struct AddExperience {
    entity: Entity,
    experience: u32,
}

#[derive(Event)]
struct PlayerUpgrade(u8);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 相机
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 21., 24.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCameraController::default(),
    ));

    // 太阳光
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1., 2., 1.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // 地面
    let ground = Rectangle::new(30., 30.);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(ground),
            material: materials.add(Color::Srgba(tailwind::ORANGE_700)),
            transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.)),
            ..default()
        },
        Name::new("Ground"),
    ));

    let level_text = commands
        .spawn((
            BillboardTextBundle {
                text: Text::from_section(
                    "Level: 0",
                    TextStyle {
                        font_size: 120.,
                        ..default()
                    },
                ),
                transform: Transform::from_xyz(0., 3.5, 0.).with_scale(Vec3::splat(0.015)),
                ..default()
            },
            LevelText,
        ))
        .id();

    // 玩家
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.5, 3., 1.5)),
                material: materials.add(Color::Srgba(tailwind::BLUE_700)),
                transform: Transform::from_xyz(0., 1.5, 0.),
                ..default()
            },
            Player {
                level: 0,
                upgrade_experience: [
                    10, 25, 48, 83, 104, 158, 237, 325, 473, 597, 783, 947, 1221, 1532, 1924, 2553,
                    3372, 5226, 8349, 1143,
                ],
                current_experience: 0,
            },
            Name::new("Player"),
        ))
        .add_child(level_text);

    // 经验值
    let rng = ChaCha8Rng::seed_from_u64(42870);
    let sphere_handle = meshes.add(Sphere::new(0.5));
    let colors_translations_experiences = [
        (tailwind::RED_200, 41),
        (tailwind::RED_400, 87),
        (tailwind::RED_600, 163),
        (tailwind::RED_800, 223),
    ];
    for (index, sample) in ground.interior_dist().sample_iter(rng).take(60).enumerate() {
        let (color, experience) =
            colors_translations_experiences[index % colors_translations_experiences.len()];
        commands.spawn((
            PbrBundle {
                mesh: sphere_handle.clone(),
                material: materials.add(Color::Srgba(color)),
                transform: Transform::from_translation(Vec3::new(sample.x, 0.5, sample.y)),
                ..default()
            },
            ExperienceValue(experience),
            Name::new(format!("Experience {experience}")),
        ));
    }
}

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 10. * time.delta_seconds();
    let mut velocity = Vec3::ZERO;
    if keyboard.pressed(KeyCode::ArrowUp) {
        velocity += Dir3::NEG_Z * speed;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        velocity += Dir3::Z * speed;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        velocity += Dir3::NEG_X * speed;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        velocity += Dir3::X * speed;
    }

    let Ok(mut transform) = player.get_single_mut() else {
        return;
    };
    transform.translation += velocity;
}

fn detect_collision(
    player: Query<&Transform, With<Player>>,
    experiences: Query<(Entity, &ExperienceValue, &Transform)>,
    mut experience_writer: EventWriter<AddExperience>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };
    let player_aabb3d = Aabb3d::new(player_transform.translation, Vec3::new(0.75, 1.5, 0.75));
    for (entity, experience_value, experience_transform) in &experiences {
        let experience_sphere = BoundingSphere::new(experience_transform.translation, 0.5);
        if player_aabb3d.intersects(&experience_sphere) {
            experience_writer.send(AddExperience {
                entity,
                experience: experience_value.0,
            });
        }
    }
}

fn add_experiences(
    mut commands: Commands,
    mut player: Query<&mut Player>,
    mut experience_reader: EventReader<AddExperience>,
    mut upgrade_writer: EventWriter<PlayerUpgrade>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
    let previous_level = player.level;
    for &AddExperience { entity, experience } in experience_reader.read() {
        player.current_experience += experience;
        while player.current_experience >= player.upgrade_experience[player.level as usize] {
            player.current_experience -= player.upgrade_experience[player.level as usize];
            player.level += 1;
        }
        commands.entity(entity).despawn();
    }
    let upgrade_level = player.level - previous_level;
    if upgrade_level > 0 {
        upgrade_writer.send(PlayerUpgrade(upgrade_level));
    }
}

fn alter_level_text(
    player: Query<&Player>,
    mut level_text: Query<&mut Text, With<LevelText>>,
    mut upgrade_reader: EventReader<PlayerUpgrade>,
) {
    for _ in upgrade_reader.read() {
        let Ok(player) = player.get_single() else {
            return;
        };
        let Ok(mut level_text) = level_text.get_single_mut() else {
            return;
        };
        level_text.sections[0].value = format!("Level: {}", player.level);
    }
}

fn spawn_upgrade_text(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    mut upgrade_reader: EventReader<PlayerUpgrade>,
) {
    for &PlayerUpgrade(level) in upgrade_reader.read() {
        let upgrade_text = commands
            .spawn((
                BillboardTextBundle {
                    text: Text::from_section(
                        format!("Up {level}"),
                        TextStyle {
                            font_size: 120.,
                            color: Color::Srgba(tailwind::YELLOW_400),
                            ..default()
                        },
                    ),
                    transform: Transform::from_xyz(0., 6., 0.).with_scale(Vec3::splat(0.015)),
                    ..default()
                },
                UpgradeText {
                    timer: Timer::from_seconds(2., TimerMode::Once),
                },
            ))
            .id();

        let Ok(entity) = player.get_single() else {
            return;
        };
        commands.entity(entity).add_child(upgrade_text);
    }
}

fn despawn_upgrade_text(
    mut commands: Commands,
    mut upgrade_texts: Query<(Entity, &mut Text, &mut UpgradeText)>,
    time: Res<Time>,
) {
    for (entity, mut text, mut upgrade_text) in &mut upgrade_texts {
        if upgrade_text.timer.tick(time.delta()).remaining_secs() > 0. {
            text.sections[0]
                .style
                .color
                .set_alpha(upgrade_text.timer.remaining_secs());
        } else {
            commands.entity(entity).despawn();
        }
    }
}
