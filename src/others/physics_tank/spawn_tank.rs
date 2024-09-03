use bevy::{prelude::*, render::mesh::VertexAttributeValues, scene::SceneInstanceReady, utils::HashMap};
use bevy_blendy_cameras::OrbitCameraController;
use bevy_rapier3d::prelude::*;

use crate::{blender_editor::SceneHandles, GameState};

pub fn plugin(app: &mut App) {
    app.register_type::<EnableLightShadows>()
        .register_type::<RemoveLayer>()
        .register_type::<LeftFrontWheel>()
        .register_type::<LeftBackWheel>()
        .register_type::<LeftFrontDriveWheel>()
        .register_type::<LeftBackDriveWheel>()
        .register_type::<RightFrontWheel>()
        .register_type::<RightBackWheel>()
        .register_type::<RightFrontDriveWheel>()
        .register_type::<RightBackDriveWheel>()
        .register_type::<TrackPad>()
        .register_type::<TankBody>()
        .register_type::<RigidBodyMarker>()
        .register_type::<ColliderMarker>()
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(GameState::Start), spawn_scene)
        .add_systems(
            Update,
            (
                remove_layer,
                markers_to_components,
                (
                    enable_light_shadows,
                    link_track_pads,
                    link_body_and_wheels,
                    track_pad_friction,
                ),
            )
                .chain()
                .run_if(
                    on_event::<SceneInstanceReady>(),
                ),
        );
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
struct EnableLightShadows;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
struct RemoveLayer;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct LeftFrontWheel;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct LeftBackWheel;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct LeftFrontDriveWheel;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct LeftBackDriveWheel;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct RightFrontWheel;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct RightBackWheel;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct RightFrontDriveWheel;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct RightBackDriveWheel;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
struct TrackPad;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
struct TankBody;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
enum RigidBodyMarker {
    Static,
    Dynamic,
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
enum ColliderMarker {
    Cuboid {
        x_length: f32,
        y_length: f32,
        z_length: f32,
    },
    Cylinder {
        radius: f32,
        height: f32,
    },
    ConvexHullFromMesh,
}

fn spawn_scene(mut commands: Commands, tank_scene_handle: Res<SceneHandles>) {
    commands.spawn(SceneBundle {
        scene: tank_scene_handle.0[0].clone(),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3., 3., 3.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        OrbitCameraController::default(),
    ));
}

fn enable_light_shadows(mut lights: Query<&mut DirectionalLight, With<EnableLightShadows>>) {
    for mut directional_light in &mut lights {
        directional_light.shadows_enabled = true;
    }
}

fn markers_to_components(
    mut commands: Commands,
    meshes: Res<Assets<Mesh>>,
    rigid_body_markers: Query<(Entity, &RigidBodyMarker)>,
    collider_markers: Query<(Entity, &Handle<Mesh>, &ColliderMarker)>,
) {
    for (entity, rigid_body_marker) in &rigid_body_markers {
        let rigid_body = match rigid_body_marker {
            &RigidBodyMarker::Static => RigidBody::Fixed,
            &RigidBodyMarker::Dynamic => RigidBody::Dynamic,
        };
        commands
            .entity(entity)
            .remove::<RigidBodyMarker>()
            .insert(rigid_body);
    }
    for (entity, mesh_handle, collider_marker) in &collider_markers {
        let VertexAttributeValues::Float32x3(mesh) = meshes
            .get(mesh_handle.id())
            .unwrap()
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
        else {
            continue;
        };
        let points: Vec<Vec3> = mesh
            .iter()
            .map(|point| Vec3::new(point[0], point[1], point[2]))
            .collect();
        let collider = match collider_marker {
            &ColliderMarker::Cuboid {
                x_length,
                y_length,
                z_length,
            } => Collider::cuboid(x_length / 2., y_length / 2., z_length / 2.),
            &ColliderMarker::Cylinder { radius, height } => Collider::cylinder(height, radius),
            &ColliderMarker::ConvexHullFromMesh => Collider::convex_hull(&points).unwrap(),
        };
        commands
            .entity(entity)
            .remove::<ColliderMarker>()
            .insert(collider);
    }
}

fn track_pad_friction(
    mut commands: Commands,
    track_pad_colliders: Query<&Children, With<TrackPad>>,
) {
    for children in &track_pad_colliders {
        commands.entity(children[0]).insert(Friction::new(1.5));
    }
}

fn remove_layer(
    mut commands: Commands,
    layers: Query<(Entity, &Parent, &Children), With<RemoveLayer>>,
) {
    for (entity, parent, children) in &layers {
        for &child_entity in children {
            commands
                .entity(child_entity)
                .set_parent_in_place(parent.get());
        }
        commands.entity(entity).despawn();
    }
}

fn link_track_pads(mut commands: Commands, track_pads: Query<(Entity, &Name), With<TrackPad>>) {
    let track_pad_map: HashMap<_, _> = track_pads
        .iter()
        .map(|(entity, name)| {
            let number = name
                .as_str()
                .strip_prefix("track_pad.")
                .expect("Strip prefix failed!")
                .parse::<usize>()
                .unwrap();
            (number, entity)
        })
        .collect();
    // const ANCHOR_LENGTH: f32 = 0.071191;
    const ANCHOR_LENGTH: f32 = 0.069;
    for number in 1..=98 {
        if number == 49 {
            commands
                .entity(track_pad_map[&number])
                .insert(ImpulseJoint::new(
                    track_pad_map[&1],
                    RevoluteJointBuilder::new(Vec3::X)
                        .local_anchor1(Vec3::Z * ANCHOR_LENGTH)
                        .local_anchor2(Vec3::NEG_Z * ANCHOR_LENGTH),
                ));
        } else if number == 98 {
            commands
                .entity(track_pad_map[&number])
                .insert(ImpulseJoint::new(
                    track_pad_map[&50],
                    RevoluteJointBuilder::new(Vec3::X)
                        .local_anchor1(Vec3::Z * ANCHOR_LENGTH)
                        .local_anchor2(Vec3::NEG_Z * ANCHOR_LENGTH),
                ));
        } else {
            commands
                .entity(track_pad_map[&number])
                .insert(ImpulseJoint::new(
                    track_pad_map[&(number + 1)],
                    RevoluteJointBuilder::new(Vec3::X)
                        .local_anchor1(Vec3::Z * ANCHOR_LENGTH)
                        .local_anchor2(Vec3::NEG_Z * ANCHOR_LENGTH),
                ));
        };
    }
}

fn link_body_and_wheels(
    mut commands: Commands,
    body: Query<Entity, With<TankBody>>,
    left_front_wheel: Query<Entity, With<LeftFrontWheel>>,
    left_back_wheel: Query<Entity, With<LeftBackWheel>>,
    left_front_drive_wheel: Query<Entity, With<LeftFrontDriveWheel>>,
    left_back_drive_wheel: Query<Entity, With<LeftBackDriveWheel>>,
    right_front_wheel: Query<Entity, With<RightFrontWheel>>,
    right_back_wheel: Query<Entity, With<RightBackWheel>>,
    right_front_drive_wheel: Query<Entity, With<RightFrontDriveWheel>>,
    right_back_drive_wheel: Query<Entity, With<RightBackDriveWheel>>,
) {
    let Ok(body_entity) = body.get_single() else {
        return;
    };
    const LEFT_FRONT_X: f32 = 0.948071;
    const LEFT_FRONT_Y: f32 = -0.42204;
    const LEFT_FRONT_Z: f32 = 0.650108;
    const LEFT_FRONT_DRIVE_Y: f32 = -0.006685;
    const LEFT_FRONT_DRIVE_Z: f32 = 1.23251;
    let entities_anchors = {
        [
            (
                left_front_wheel.single(),
                Vec3::new(LEFT_FRONT_X, LEFT_FRONT_Y, LEFT_FRONT_Z),
            ),
            (
                left_back_wheel.single(),
                Vec3::new(LEFT_FRONT_X, LEFT_FRONT_Y, -LEFT_FRONT_Z),
            ),
            (
                left_front_drive_wheel.single(),
                Vec3::new(LEFT_FRONT_X, LEFT_FRONT_DRIVE_Y, LEFT_FRONT_DRIVE_Z),
            ),
            (
                left_back_drive_wheel.single(),
                Vec3::new(LEFT_FRONT_X, LEFT_FRONT_DRIVE_Y, -LEFT_FRONT_DRIVE_Z),
            ),
            (
                right_front_wheel.single(),
                Vec3::new(-LEFT_FRONT_X, LEFT_FRONT_Y, LEFT_FRONT_Z),
            ),
            (
                right_back_wheel.single(),
                Vec3::new(-LEFT_FRONT_X, LEFT_FRONT_Y, -LEFT_FRONT_Z),
            ),
            (
                right_front_drive_wheel.single(),
                Vec3::new(-LEFT_FRONT_X, LEFT_FRONT_DRIVE_Y, LEFT_FRONT_DRIVE_Z),
            ),
            (
                right_back_drive_wheel.single(),
                Vec3::new(-LEFT_FRONT_X, LEFT_FRONT_DRIVE_Y, -LEFT_FRONT_DRIVE_Z),
            ),
        ]
    };
    for (wheel_entity, anchor_1) in entities_anchors {
        commands
            .entity(wheel_entity)
            .insert(Velocity::zero())
            .insert(ImpulseJoint::new(
                body_entity,
                RevoluteJointBuilder::new(Vec3::X)
                    .local_anchor1(anchor_1)
                    .local_anchor2(Vec3::ZERO),
            ));
    }
}
