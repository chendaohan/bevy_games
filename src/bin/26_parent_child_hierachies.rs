use bevy::{
    input::common_conditions::input_just_pressed, prelude::*, sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::default()))
        .add_systems(Startup, (spawn_parent_children, setup))
        .add_systems(
            Update,
            (
                despawn_my_parent.run_if(input_just_pressed(KeyCode::KeyD)),
                despawn_my_children.run_if(input_just_pressed(KeyCode::KeyC)),
                despawn_my_children_correction.run_if(input_just_pressed(KeyCode::KeyO)),
                camera_with_parent.run_if(input_just_pressed(KeyCode::Space)),
            ),
        )
        .run()
}

#[derive(Component)]
struct MyParent;

#[derive(Component)]
struct MyChild;

#[derive(Component)]
struct MyComponent;

#[derive(Component)]
struct MyParentMarker;

fn spawn_parent_children(mut commands: Commands) {
    let child_0 = commands.spawn(MyChild).id();
    // commands.entity(entity).add_child(child_0);
    commands
        .spawn((MyParent, Name::new("Parent 1"))) 
        .add_child(child_0);

    let child_1 = commands.spawn(MyChild).id();
    let child_2 = commands.spawn((MyChild, MyComponent)).id();
    commands
        .spawn((MyParent, MyParentMarker, Name::new("Parent 2")))
        .push_children(&[child_1, child_2]);

    commands
        .spawn((MyParent, MyParentMarker, Name::new("Parent 3")))
        .with_children(|parent| {
            parent.spawn(MyChild);
            parent.spawn((MyChild, MyComponent));
        });
}

fn despawn_my_parent(mut commands: Commands, parents: Query<Entity, With<MyParentMarker>>) {
    let Some(parent_entity) = parents.iter().next() else {
        return;
    };
    commands.entity(parent_entity).despawn_recursive(); // despawn() , despawn_descendants()
}

fn despawn_my_children(mut commands: Commands, parents: Query<&Children, With<MyParentMarker>>) {
    let Some(children) = parents.iter().next() else {
        return;
    };
    for &child_entity in children.iter() {
        commands.entity(child_entity).despawn();
    }
}

fn despawn_my_children_correction(mut commands: Commands, parents: Query<(Entity, &Children), With<MyParentMarker>>) {
    let Some((self_entity, children)) = parents.iter().next() else {
        return;
    };
    commands.entity(self_entity).clear_children();
    for &child_entity in children.iter() {
        commands.entity(child_entity).despawn();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let camera_entity = commands.spawn(Camera2dBundle::default()).id();

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(200., 200.)).into(),
                material: materials.add(Color::Srgba(Srgba::RED)),
                ..default()
            },
            Name::new("Red Square"),
        ))
        .add_child(camera_entity);
}

fn camera_with_parent(
    camera: Query<(&Parent, &Transform), With<Camera>>,
    transforms: Query<&GlobalTransform>,
) {
    let Ok((parent, child_transform)) = camera.get_single() else {
        return;
    };
    info!("child transform: {child_transform:?}");
    let Ok(parent_global_transform) = transforms.get(parent.get()) else {
        return;
    };
    info!("parent global transform: {parent_global_transform:?}");
}
