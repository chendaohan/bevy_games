use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    render::view::VisibilitySystems,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            toggle_square_visibility.run_if(input_just_pressed(KeyCode::Space)),
        )
        .add_systems(
            PostUpdate,
            (
                print_triangle_iherited_visibility
                    .after(VisibilitySystems::VisibilityPropagate)
                    .run_if(input_just_pressed(KeyCode::KeyI)),
                print_triangle_view_visibility
                    .after(VisibilitySystems::CheckVisibility)
                    .run_if(input_just_pressed(KeyCode::KeyV)),
            ),
        )
        .run()
}

#[derive(Component)]
struct MySquare;

#[derive(Component)]
struct MyTriangle;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let triangle_entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(Triangle2d::new(
                        Vec2::new(0., 100.),
                        Vec2::new(100., -75.),
                        Vec2::new(-100., -75.),
                    ))
                    .into(),
                material: materials.add(Color::Srgba(Srgba::GREEN)),
                transform: Transform::from_xyz(0., -250., 0.),
                ..default()
            },
            Name::new("Green Triangle"),
            MyTriangle,
        ))
        .id();

    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::new(200., 200.)).into(),
                material: materials.add(Color::Srgba(Srgba::RED)),
                transform: Transform::from_xyz(-400., 0., 0.),
                visibility: Visibility::Visible,
                ..default()
            },
            Name::new("Red Square"),
            MySquare,
        ))
        .add_child(triangle_entity);

    commands.spawn((
        Mesh2dHandle(meshes.add(Circle::new(100.))),
        materials.add(Color::Srgba(Srgba::BLUE)),
        // SpatialBundle::default(),
        // TransformBundle::default(),
        Transform::default(),
        GlobalTransform::default(),
        // VisibilityBundle::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),

        Name::new("Blue Circle"),
    ));
}

fn toggle_square_visibility(mut square: Query<&mut Visibility, With<MySquare>>) {
    let Ok(mut visibility) = square.get_single_mut() else {
        return;
    };
    if let Visibility::Visible = *visibility {
        *visibility = Visibility::Hidden;
    } else {
        *visibility = Visibility::Visible;
    }
}

fn print_triangle_iherited_visibility(triangle: Query<&InheritedVisibility, With<MyTriangle>>) {
    let Ok(inherited_visibility) = triangle.get_single() else {
        return;
    };
    info!("triangle inherited visibility: {inherited_visibility:?}");
}

fn print_triangle_view_visibility(triangle: Query<&ViewVisibility, With<MyTriangle>>) {
    let Ok(view_visibility) = triangle.get_single() else {
        return;
    };
    info!("triangle view visibility: {view_visibility:?}");
}
