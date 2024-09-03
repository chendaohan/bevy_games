use bevy::{
    color::palettes::css,
    dev_tools::fps_overlay::FpsOverlayPlugin,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() -> AppExit {
    App::new()
        // 使用默认插件组
        .add_plugins(DefaultPlugins)
        // 在左上角显示帧数
        .add_plugins(FpsOverlayPlugin::default())
        .add_systems(Startup, setup)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 生成 2D 相机
    commands.spawn(Camera2dBundle::default());
    // 生成红色矩形
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle::new(500., 300.))),
        material: materials.add(ColorMaterial::from_color(Color::Srgba(css::RED))),
        ..default()
    });
}
