use bevy::{
    asset::{AssetPath, LoadState},
    ecs::system::RunSystemOnce,
    gltf::{GltfMaterialExtras, GltfMeshExtras, GltfSceneExtras},
    prelude::*,
    reflect::{serde::ReflectDeserializer, TypeRegistration, TypeRegistry},
    scene::ron,
    state::state::FreelyMutableState,
};
use serde::de::DeserializeSeed;

pub struct BlenderEditorPlugin<T: States + FreelyMutableState> {
    pub scene_paths: Vec<AssetPath<'static>>,
    pub processed: T,
}

impl<T: States + FreelyMutableState> BlenderEditorPlugin<T> {
    pub fn new(scene_paths: Vec<AssetPath<'static>>, processed: T) -> Self {
        Self {
            scene_paths,
            processed,
        }
    }
}

impl<T: States + FreelyMutableState> Plugin for BlenderEditorPlugin<T> {
    fn build(&self, app: &mut App) {
        let asset_server = app.world().resource_ref::<AssetServer>();
        let handles = self
            .scene_paths
            .iter()
            .map(|path| asset_server.load(path))
            .collect();
        app.insert_resource(SceneHandles(handles))
            .add_systems(
                Update,
                all_reflect_components_from_extras(self.processed.clone())
                    .run_if(scene_loaded().and_then(run_once())),
            );
    }
}

#[derive(Resource)]
pub struct SceneHandles(pub Vec<Handle<Scene>>);

fn scene_loaded() -> impl FnMut(Res<SceneHandles>, Res<AssetServer>) -> bool {
    let mut passed = false;
    move |scene_handles, asset_server| {
        if !passed {
            let mut loaded = true;
            for handle in &scene_handles.0 {
                if asset_server.load_state(handle) != LoadState::Loaded {
                    loaded = false;
                }
            }
            passed = loaded;
        }
        passed
    }
}

fn all_reflect_components_from_extras<T>(
    processed: T,
) -> impl FnMut(Res<AppTypeRegistry>, ResMut<Assets<Scene>>, Res<SceneHandles>, ResMut<NextState<T>>)
where
    T: States + FreelyMutableState,
{
    move |app_type_registry, mut scenes, scene_handles, mut next_state| {
        for scene_handle in &scene_handles.0 {
            let Some(Scene { world }) = scenes.get_mut(scene_handle.id()) else {
                continue;
            };
            let app_type_registry = app_type_registry.clone();
            world.run_system_once(insert_reflect_components(app_type_registry));
        }
        next_state.set(processed.clone());
    }
}

// 插入反射组件
fn insert_reflect_components(
    app_type_registry: AppTypeRegistry,
) -> impl FnMut(
    ParallelCommands,
    Query<(
        Entity,
        Option<&GltfSceneExtras>,
        Option<&GltfExtras>,
        Option<&GltfMeshExtras>,
        Option<&GltfMaterialExtras>,
    )>,
) {
    move |commands, gltf_extras| {
        gltf_extras.par_iter().for_each(
            |(extras_entity, scene_extras, extras, mesh_extras, material_extras)| {
                let mut extras_vec: Vec<&str> = Vec::new();
                if let Some(GltfSceneExtras { value }) = scene_extras {
                    extras_vec.push(value);
                }
                if let Some(GltfExtras { value }) = extras {
                    extras_vec.push(value);
                }
                if let Some(GltfMeshExtras { value }) = mesh_extras {
                    extras_vec.push(value);
                }
                if let Some(GltfMaterialExtras { value }) = material_extras {
                    extras_vec.push(value);
                }
                if extras_vec.is_empty() {
                    return;
                }
                let type_registry = app_type_registry.clone();
                let reflect_components: Vec<_> = extras_vec
                    .into_iter()
                    .filter_map(reflect_components_of_extras(
                        extras_entity,
                        type_registry,
                    ))
                    .flatten()
                    .collect();
                let type_registry = app_type_registry.clone();
                commands.command_scope(move |mut commands| {
                    commands.add(move |world: &mut World| {
                        let type_registry = type_registry.read();
                        let mut entity_mut = world.entity_mut(extras_entity);
                        for (component, type_registration) in &reflect_components {
                            type_registration
                                .data::<ReflectComponent>()
                                .unwrap()
                                .insert(&mut entity_mut, component.as_reflect(), &type_registry);
                        }
                    });
                });
            },
        );
    }
}

// 反射多个组件
fn reflect_components_of_extras(
    extras_entity: Entity,
    app_type_registry: AppTypeRegistry,
) -> impl FnMut(&str) -> Option<Vec<(Box<dyn Reflect>, TypeRegistration)>> {
    move |extras| {
        let json_map = match serde_json::from_str::<serde_json::Map<_, _>>(extras) {
            Ok(map) => map,
            Err(error) => {
                error!("{extras_entity} extras json parse failed: {error}");
                return None;
            }
        };
        let type_registry = app_type_registry.read();
        let components: Vec<_> = json_map
            .into_iter()
            .filter_map(reflect_component(extras_entity, &type_registry))
            .collect();
        Some(components)
    }
}

// 反射组件
fn reflect_component<'a>(
    extras_entity: Entity,
    type_registry: &'a TypeRegistry,
) -> impl FnMut((String, serde_json::Value)) -> Option<(Box<dyn Reflect>, TypeRegistration)> + 'a {
    move |(key, value)| {
        let Some(value) = value.as_str() else {
            return None;
        };
        let Some(type_registration) = type_registry.get_with_short_type_path(&key) else {
            error!("{extras_entity} extra get type registration failed!");
            return None;
        };
        let type_path = type_registration.type_info().type_path();
        let ron_string = format!("{{\"{type_path}\":{}}}", value);
        let reflect_deserializer = ReflectDeserializer::new(&type_registry);
        let mut deserializer = match ron::Deserializer::from_str(&ron_string) {
            Ok(deserializer) => deserializer,
            Err(error) => {
                error!("{extras_entity} extra ron parse failed: {error}");
                return None;
            }
        };
        let component = reflect_deserializer.deserialize(&mut deserializer).unwrap();
        Some((component, type_registration.clone()))
    }
}
