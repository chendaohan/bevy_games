use bevy::{asset::LoadState, input::common_conditions::input_just_pressed, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "assets/28_states_and_data_of_assets".into(),
            ..default()
        }))
        .add_systems(Startup, (setup, spawn_suzanne, spawn_red_sphere))
        .add_systems(
            Update,
            (
                print_asset_event,
                print_load_state.run_if(input_just_pressed(KeyCode::Space)),
                suzanne_to_cube.run_if(input_just_pressed(KeyCode::KeyS)),
            ),
        )
        .run()
}

#[derive(Resource)]
struct Suzanne(Handle<Mesh>);

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 3., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(1., 2., 1.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_suzanne(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let suzanne_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("suzanne.glb"),
    );

    commands.insert_resource(Suzanne(suzanne_handle.clone()));

    commands.spawn((
        MaterialMeshBundle {
            mesh: suzanne_handle,
            material: materials.add(StandardMaterial::default()),
            ..default()
        },
        Name::new("Suzanne"),
    ));
}

fn spawn_red_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Sphere::new(1.)),
            material: materials.add(Color::Srgba(Srgba::RED)),
            transform: Transform::from_xyz(-2.5, 0., 0.),
            ..default()
        },
        Name::new("Red Sphere"),
    ));
}

fn print_asset_event(mut asset_events: EventReader<AssetEvent<Mesh>>) {
    for asset_event in asset_events.read() {
        match asset_event {
            AssetEvent::Added { id } => info!("asset event added: {id}"),
            AssetEvent::Modified { id } => info!("asset event modified: {id}"),
            AssetEvent::Removed { id } => info!("asset event removed: {id}"),
            AssetEvent::Unused { id } => info!("asset event unused: {id}"),
            AssetEvent::LoadedWithDependencies { id } => info!("asset event loaded: {id}"),
        }
    }
}

fn print_load_state(asset_server: Res<AssetServer>, suzanne: Res<Suzanne>) {
    let Some(load_state) = asset_server.get_load_state(suzanne.0.id()) else {
        return;
    };
    match load_state {
        LoadState::NotLoaded => info!("asset not loaded"),
        LoadState::Loading => info!("asset loading"),
        LoadState::Loaded => info!("asset loaded"),
        LoadState::Failed(error) => error!("asset failed: {error}"),
    }
}

fn suzanne_to_cube(mut meshes: ResMut<Assets<Mesh>>, suzanne: Res<Suzanne>) {
    let Some(mesh) = meshes.get_mut(suzanne.0.id()) else {
        return;
    };
    *mesh = Cuboid::from_size(Vec3::splat(2.)).mesh().build();
}
