mod blender_editor;

use bevy::{
    asset::AssetPath,
    input::common_conditions::input_pressed,
    math::bounding::{Aabb3d, BoundingSphere, IntersectsVolume},
    prelude::*,
    scene::SceneInstanceReady,
};
use blender_editor::{BlenderEditorPlugin, SceneHandles};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BlenderEditorPlugin::new(
                vec![AssetPath::from(
                    "blender_as_bevy_editor/barbette.glb#Scene0",
                )],
                GameState::Start,
            ),
        ))
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .register_type::<ShadowsEnabled>()
        .register_type::<Barbette>()
        .register_type::<Enemy>()
        .register_type::<Collider>()
        .register_type::<Cannonball>()
        .init_state::<GameState>()
        .add_systems(OnEnter(GameState::Start), spawn_scene)
        .add_systems(
            Update,
            (
                (spawn_barbette_timer, shadows_enabled, add_red_material)
                    .run_if(on_event::<SceneInstanceReady>()),
                (
                    move_barbette,
                    tick_timer,
                    move_cannonballs,
                    despawn_cannonballs,
                    attack_enemies,
                    move_enemies,
                    emit_cannonballs.run_if(input_pressed(KeyCode::Space)),
                )
                    .run_if(in_state(GameState::Start)),
            ),
        )
        .run()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, States, Default)]
enum GameState {
    #[default]
    Loading,
    Start,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct ShadowsEnabled(bool);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Barbette;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Enemy;

#[derive(Component, Reflect)]
#[reflect(Component)]
enum Collider {
    Cuboid(Vec3),
    Sphere(f32),
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Cannonball;

#[derive(Component)]
struct AttackTimer(Timer);

#[derive(Resource)]
struct RedHandle(Handle<StandardMaterial>);

fn spawn_scene(mut commands: Commands, scene_handles: Res<SceneHandles>) {
    commands.spawn(SceneBundle {
        scene: scene_handles.0[0].clone(),
        ..default()
    });
}

fn shadows_enabled(mut lights: Query<(&mut DirectionalLight, &ShadowsEnabled)>) {
    for (mut light, shadows_enabled) in &mut lights {
        if shadows_enabled.0 {
            light.shadows_enabled = true;
        }
    }
}

fn move_barbette(
    mut barbette: Query<&mut Transform, With<Barbette>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let speed = 10. * time.delta_seconds();
    let mut velocity = Vec3::ZERO;
    if keyboard.pressed(KeyCode::ArrowLeft) {
        velocity.x -= speed;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        velocity.x += speed;
    }
    let Ok(mut barbette) = barbette.get_single_mut() else {
        return;
    };
    barbette.translation += velocity;
}

fn spawn_barbette_timer(mut commands: Commands, barbette: Query<Entity, With<Barbette>>) {
    let Ok(entity) = barbette.get_single() else {
        return;
    };
    commands
        .entity(entity)
        .insert(AttackTimer(Timer::from_seconds(
            1. / 5.,
            TimerMode::Once,
        )));
}

fn add_red_material(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(RedHandle(materials.add(Color::Srgba(Srgba::RED))));
}

fn tick_timer(mut cannonball_timer: Query<&mut AttackTimer>, time: Res<Time>) {
    for mut timer in &mut cannonball_timer {
        timer.0.tick(time.delta());
    }
}

fn emit_cannonballs(
    mut commands: Commands,
    mut barbette_timer: Query<(&mut AttackTimer, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    red_handle: Res<RedHandle>,
) {
    let Ok((mut timer, transform)) = barbette_timer.get_single_mut() else {
        return;
    };
    if timer.0.finished() {
        let translation = transform.translation;
        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(Sphere::new(0.25 / 2.)),
                material: red_handle.0.clone(),
                transform: Transform::from_xyz(translation.x, 0.43, 5.7),
                ..default()
            },
            Cannonball,
        ));
        timer.0.reset();
    }
}

fn move_cannonballs(mut cannonballs: Query<&mut Transform, With<Cannonball>>, time: Res<Time>) {
    let velocity = Vec3::NEG_Z * 10. * time.delta_seconds();
    for mut transform in &mut cannonballs {
        transform.translation += velocity;
    }
}

fn despawn_cannonballs(
    mut commands: Commands,
    cannonballs: Query<(Entity, &Transform), With<Cannonball>>,
) {
    for (entity, transform) in &cannonballs {
        if transform.translation.z < -20. {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn attack_enemies(
    mut commands: Commands,
    enemies: Query<(Entity, &Collider, &Transform), With<Enemy>>,
    cannonballs: Query<(Entity, &Transform), With<Cannonball>>,
) {
    let enemies: Vec<_> = enemies
        .iter()
        .map(|(entity, collider, transform)| (entity, collider, transform.translation))
        .collect();
    let cannonballs: Vec<_> = cannonballs
        .iter()
        .map(|(entity, transform)| (entity, transform.translation))
        .collect();
    for &(enemy_entity, enemy_collider, enemy_translation) in &enemies {
        for &(cannonball_entity, cannonball_translation) in &cannonballs {
            let cannonball_sphere = BoundingSphere::new(cannonball_translation, 0.25 / 2.);
            match enemy_collider {
                &Collider::Cuboid(size) => {
                    let enemy_aabb3d = Aabb3d::new(enemy_translation, size);
                    if cannonball_sphere.intersects(&enemy_aabb3d) {
                        commands.entity(enemy_entity).despawn_recursive();
                        commands.entity(cannonball_entity).despawn_recursive();
                    }
                }
                &Collider::Sphere(radius) => {
                    let enemy_sphere = BoundingSphere::new(enemy_translation, radius);
                    if cannonball_sphere.intersects(&enemy_sphere) {
                        commands.entity(enemy_entity).despawn_recursive();
                        commands.entity(cannonball_entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

fn move_enemies(mut enemies: Query<&mut Transform, With<Enemy>>, time: Res<Time>) {
    let velocity = Vec3::Z * time.delta_seconds();
    for mut transform in &mut enemies {
        transform.translation += velocity;
    }
}
