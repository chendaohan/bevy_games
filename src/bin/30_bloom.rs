use bevy::{
    asset::RecursiveDependencyLoadState,
    core_pipeline::bloom::BloomSettings,
    ecs::system::RunSystemOnce,
    pbr::NotShadowCaster,
    prelude::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "assets/30_bloom".into(),
                ..default()
            }),
            WorldInspectorPlugin::default(),
        ))
        .add_systems(Startup, load_bloom_scene)
        .add_systems(
            Update,
            modify_bloom_scene.run_if(bloom_scene_loaded.and_then(run_once())),
        )
        .add_systems(
            Update,
            (toggle_bloom_presets, add_icosphere_emissive_luminance),
        )
        .run()
}

#[derive(Resource)]
struct BloomScene(Handle<Scene>);

fn load_bloom_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bloom_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("bloom.glb"));
    commands.insert_resource(BloomScene(bloom_scene));
}

fn bloom_scene_loaded(asset_server: Res<AssetServer>, bloom_scene: Res<BloomScene>) -> bool {
    let Some(load_state) = asset_server.get_recursive_dependency_load_state(bloom_scene.0.id())
    else {
        return false;
    };
    if let RecursiveDependencyLoadState::Loaded = load_state {
        true
    } else {
        false
    }
}

fn toggle_bloom_presets(
    mut bloom_settings: Query<&mut BloomSettings>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut bloom_settings) = bloom_settings.get_single_mut() else {
        return;
    };
    if keys.just_pressed(KeyCode::Digit1) {
        *bloom_settings = BloomSettings::NATURAL;
    } else if keys.just_pressed(KeyCode::Digit2) {
        *bloom_settings = BloomSettings::OLD_SCHOOL;
    } else if keys.just_pressed(KeyCode::Digit3) {
        *bloom_settings = BloomSettings::SCREEN_BLUR;
    }
}

fn add_icosphere_emissive_luminance(
    icospheres: Query<(&Handle<StandardMaterial>, &Name)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for (material_handle, name) in &icospheres {
        if name.ends_with("Icosphere") {
            if let Some(material) = materials.get_mut(material_handle.id()) {
                let increment = if keys.just_pressed(KeyCode::ArrowUp) {
                    0.5
                } else if keys.just_pressed(KeyCode::ArrowDown) {
                    -0.5
                } else {
                    0.
                };

                let LinearRgba {
                    red,
                    green,
                    blue,
                    alpha: _,
                } = material.emissive;

                material.emissive = LinearRgba::rgb(
                    red + increment * 0.2126,
                    green + increment * 0.7152,
                    blue + increment * 0.0722,
                );
            }
        }
    }
}

fn modify_bloom_scene(
    mut commands: Commands,
    mut scenes: ResMut<Assets<Scene>>,
    bloom_scene: Res<BloomScene>,
) {
    let Some(Scene { world }) = scenes.get_mut(bloom_scene.0.id()) else {
        return;
    };
    world.run_system_once(enable_lights_shadows);
    world.run_system_once(enable_camera_hdr);
    world.run_system_once(open_bloom);
    world.run_system_once(icosphere_not_shadow_caster);

    commands.spawn(SceneBundle {
        scene: bloom_scene.0.clone(),
        ..default()
    });
}

fn enable_lights_shadows(mut lights: Query<&mut PointLight>) {
    for mut light in &mut lights {
        light.shadows_enabled = true;
    }
}

fn icosphere_not_shadow_caster(
    mut commands: Commands,
    icospheres: Query<(Entity, &Name), With<Handle<Mesh>>>,
) {
    for (entity, name) in &icospheres {
        if name.ends_with("Icosphere") {
            commands.entity(entity).insert(NotShadowCaster);
        }
    }
}

fn enable_camera_hdr(mut camera: Query<&mut Camera>) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    camera.hdr = true;
}

fn open_bloom(mut commands: Commands, camera: Query<Entity, With<Camera>>) {
    let Ok(entity) = camera.get_single() else {
        return;
    };
    commands.entity(entity).insert(BloomSettings::NATURAL);
}
