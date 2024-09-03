use bevy::{
    core_pipeline::tonemapping::Tonemapping, prelude::*, render::render_resource::{AsBindGroup, ShaderRef}
};
use bevy_blendy_cameras::{BlendyCamerasPlugin, OrbitCameraController};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BlendyCamerasPlugin,
            MaterialPlugin::<GrassMaterial>::default(),
        ))
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (insert_orbit_camera_controller, replace_to_grass_material),
        )
        .run()
}

#[derive(Debug, Resource)]
struct GrassMaterialHandle(Handle<GrassMaterial>);

#[derive(Asset, TypePath, AsBindGroup, Clone)]
struct GrassMaterial {}

impl Material for GrassMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::from("14_load_assets/grass.wgsl")
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grass_materials: ResMut<Assets<GrassMaterial>>,
) {
    commands.spawn(SceneBundle {
        // asset_server.load("14_load_assets/grass.glb#Scene0");
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("14_load_assets/grass.glb")),
        ..default()
    });

    commands.insert_resource(GrassMaterialHandle(grass_materials.add(GrassMaterial {})));
}

fn insert_orbit_camera_controller(
    mut commands: Commands,
    camera: Query<Entity, (With<Camera>, Without<OrbitCameraController>)>,
) {
    let Ok(entity) = camera.get_single() else {
        return;
    };
    commands
        .entity(entity)
        .insert((Tonemapping::AgX, OrbitCameraController::default()));
}

fn replace_to_grass_material(
    mut commands: Commands,
    grasses: Query<(&Parent, Entity), With<Handle<StandardMaterial>>>,
    entities: Query<&Name, (With<Transform>, Without<Handle<Mesh>>)>,
    grass_material_handle: Res<GrassMaterialHandle>,
) {
    for (parent, entity) in &grasses {
        if let Ok(name) = entities.get(parent.get()) {
            if name.as_str() != "ground" {
                commands
                    .entity(entity)
                    .remove::<Handle<StandardMaterial>>()
                    .insert(grass_material_handle.0.clone());
            }
        }
    }
}
