use avian2d::prelude::*;
use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use super::GamePhase;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GamePhase::Introduction),
        spawn_rigid_body_and_sensor,
    );
}

// 玩家刚体
#[derive(Component)]
pub struct PlayerRigidBody;

// 尖刺传感器
#[derive(Component)]
pub struct SpikeSensor;

// 火坑传感器
#[derive(Component)]
pub struct FirePitSensor;

// 食物传感器
#[derive(Component)]
pub struct FoodSensor;

// 根据场景中专门用于生成碰撞体的网格，生成碰撞体
fn spawn_rigid_body_and_sensor(
    mut commands: Commands,
    objects: Query<(&Parent, Entity, &Children, &Name)>,
    collider_objects: Query<(&Children, &Transform), Without<Handle<Mesh>>>,
    mesh_objects: Query<&Handle<Mesh>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (parent, entity, children, name) in &objects {
        // 生成墙壁刚体
        if name.starts_with("wall_object") {
            if children.len() == 2 {
                if let Ok((collider_children, _)) = collider_objects.get(children[1]) {
                    if let Ok(mesh_handle) = mesh_objects.get(collider_children[0]) {
                        if let Some(mesh) = meshes.get(mesh_handle) {
                            if let Some(VertexAttributeValues::Float32x3(positions)) =
                                mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                            {
                                let positions = positions
                                    .iter()
                                    .map(|position| Vec2::new(position[0], position[1]))
                                    .collect();
                                commands
                                    .entity(children[0])
                                    .insert((
                                        RigidBody::Static,
                                        Collider::convex_hull(positions).unwrap(),
                                    ))
                                    .set_parent_in_place(parent.get());
                                let mut entity_commands = commands.entity(entity);
                                entity_commands.remove_parent();
                                entity_commands.despawn_recursive();
                            }
                        }
                    }
                }
            } else {
                let mut sub_colliders = Vec::new();
                for &collider_entity in children.iter().skip(1) {
                    if let Ok((collider_children, transform)) =
                        collider_objects.get(collider_entity)
                    {
                        if let Ok(mesh_handle) = mesh_objects.get(collider_children[0]) {
                            if let Some(mesh) = meshes.get(mesh_handle) {
                                if let Some(VertexAttributeValues::Float32x3(positions)) =
                                    mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                                {
                                    let translation = transform.translation;
                                    let positions = positions
                                        .iter()
                                        .map(|position| {
                                            Vec2::new(
                                                position[0] + translation.x,
                                                position[1] + translation.y,
                                            )
                                        })
                                        .collect();
                                    sub_colliders.push(
                                        commands
                                            .spawn(Collider::convex_hull(positions).unwrap())
                                            .id(),
                                    );
                                }
                            }
                        }
                    }
                }
                commands
                    .entity(children[0])
                    .insert(RigidBody::Static)
                    .push_children(&sub_colliders)
                    .set_parent_in_place(parent.get());
                let mut entity_commands = commands.entity(entity);
                entity_commands.remove_parent();
                entity_commands.despawn_recursive();
            }
        }

        // 生成尖刺传感器
        if name.starts_with("spike_collider_object") {
            if let Ok((children, transform)) = collider_objects.get(entity) {
                if let Ok(mesh_handle) = mesh_objects.get(children[0]) {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        if let Some(VertexAttributeValues::Float32x3(positions)) =
                            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                        {
                            let translation = transform.translation;
                            let positions = positions
                                .iter()
                                .map(|position| {
                                    Vec2::new(
                                        position[0] + translation.x,
                                        position[1] + translation.y,
                                    )
                                })
                                .collect();
                            commands.spawn((
                                RigidBody::Static,
                                Collider::convex_hull(positions).unwrap(),
                                Sensor,
                                SpikeSensor,
                            ));
                            commands.entity(entity).despawn_recursive();
                        }
                    }
                }
            }
        }

        // 生成火坑传感器
        if name.starts_with("fire_pit_object") {
            if let Ok((collider_children, _)) = collider_objects.get(children[1]) {
                if let Ok(mesh_handle) = mesh_objects.get(collider_children[0]) {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        if let Some(VertexAttributeValues::Float32x3(positions)) =
                            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                        {
                            let positions = positions
                                .iter()
                                .map(|position| Vec2::new(position[0], position[1]))
                                .collect();
                            commands
                                .entity(children[0])
                                .insert((
                                    RigidBody::Static,
                                    Collider::convex_hull(positions).unwrap(),
                                    Sensor,
                                    FirePitSensor,
                                ))
                                .set_parent_in_place(parent.get());
                            let mut entity_commands = commands.entity(children[1]);
                            entity_commands.remove_parent();
                            entity_commands.despawn_recursive();
                        }
                    }
                }
            }
        }

        // 生成食物传感器
        if name.starts_with("food_object") {
            if let Ok((collider_children, _)) = collider_objects.get(children[1]) {
                if let Ok(mesh_handle) = mesh_objects.get(collider_children[0]) {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        if let Some(VertexAttributeValues::Float32x3(positions)) =
                            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                        {
                            let positions = positions
                                .iter()
                                .map(|position| Vec2::new(position[0], position[1]))
                                .collect();
                            commands
                                .entity(children[0])
                                .insert((
                                    RigidBody::Static,
                                    Collider::convex_hull(positions).unwrap(),
                                    Sensor,
                                    FoodSensor,
                                ))
                                .set_parent_in_place(parent.get());
                            let mut entity_commands = commands.entity(children[1]);
                            entity_commands.remove_parent();
                            entity_commands.despawn_recursive();
                        }
                    }
                }
            }
        }

        // 生成玩家刚体
        if name.starts_with("player_collider_object") {
            if let Ok((children, transform)) = collider_objects.get(entity) {
                if let Ok(mesh_handle) = mesh_objects.get(children[0]) {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        if let Some(VertexAttributeValues::Float32x3(positions)) =
                            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                        {
                            let translation = transform.translation;
                            let positions = positions
                                .iter()
                                .map(|position| {
                                    Vec2::new(
                                        position[0] + translation.x,
                                        position[1] + translation.y,
                                    )
                                })
                                .collect();
                            commands.entity(parent.get()).insert((
                                RigidBody::Dynamic,
                                Collider::convex_hull(positions).unwrap(),
                                LockedAxes::ROTATION_LOCKED,
                                PlayerRigidBody,
                            ));
                            let mut entity_commands = commands.entity(entity);
                            entity_commands.remove_parent();
                            entity_commands.despawn_recursive();
                        }
                    }
                }
            }
        }
    }
}
