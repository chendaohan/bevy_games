use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::{
        Cursor, CursorGrabMode, EnabledButtons,
        PresentMode, WindowLevel, WindowMode, WindowResolution, WindowTheme,
    },
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // 窗口标题
                title: "Window Properties".into(),
                // 窗口的应用程序 ID（Wayland)、WM_CLASS（X11) 或 窗口类名称（Windows）
                name: Some("bevy.window".into()),
                // 控制窗口模式
                mode: WindowMode::Windowed,
                // 窗口位置
                position: WindowPosition::Automatic,
                // 窗口透明
                transparent: true,
                // 窗口分辨率
                resolution: WindowResolution::new(500., 300.).with_scale_factor_override(1.),
                // 尺寸约束
                resize_constraints: WindowResizeConstraints {
                    min_width: 300.,
                    max_width: 800.,
                    min_height: 150.,
                    max_height: 500.,
                },
                // 重新调整尺寸
                resizable: true,
                // 呈现模式
                present_mode: PresentMode::AutoVsync,
                // 光标
                cursor: Cursor {
                    // 光标图标
                    icon: CursorIcon::Default,
                    // 光标显示
                    visible: true,
                    // 光标限制
                    grab_mode: CursorGrabMode::None,
                    // 捕获鼠标事件
                    hit_test: true,
                },
                // 窗口装饰
                decorations: true,
                // 窗口按钮
                enabled_buttons: EnabledButtons {
                    // 最大化
                    minimize: true,
                    // 最小化
                    maximize: true,
                    // 关闭
                    close: true,
                },
                // 焦点
                focused: true,
                // 窗口等级
                window_level: WindowLevel::Normal,
                // 输入法编辑器
                ime_enabled: false,
                // 输入法编辑器位置
                ime_position: Vec2::ZERO,
                // 窗口主题
                window_theme: Some(WindowTheme::Dark),
                // 窗口可见性
                visible: true,
                // 跳过任务栏
                skip_taskbar: false,
                // 最大排队帧数
                desired_maximum_frame_latency: None,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .add_systems(Update, app_exit.run_if(input_just_pressed(KeyCode::Escape)))
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle::new(200.))),
        material: materials.add(Color::Srgba(Srgba::BLUE)),
        ..default()
    });
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    exit_writer.send(AppExit::Success);
}
