use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*,
    time::common_conditions::once_after_real_delay,
};
use std::time::Duration;

fn main() -> AppExit {
    App::new()
        // 添加插件
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins((LogPlugin::default(), StatesPlugin))
        // 注册反射类型
        .register_type::<MyReflectCompoent>()
        // 注册反射 Trait
        .register_type_data::<MyReflectCompoent, ReflectMyReflectTrait>()
        // 初始化状态
        .init_state::<GameState>()
        // 插入状态
        .insert_state(GameState::Start)
        // 初始化资源
        .init_resource::<MyResource>()
        // 插入资源
        .insert_resource(InsertMyResource {
            field_1: 0.,
            field_2: 0,
        })
        // 添加事件
        .add_event::<MyEvent>()
        // 配置系统集
        .configure_sets(Startup, MySystemSet)
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(3))),
        )
        // 添加系统
        .add_systems(Startup, system.in_set(MySystemSet))
        // 运行 App
        .run()
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MyReflectCompoent {
    field_1: u32,
    field_2: f32,
    field_3: String,
}

impl MyReflectTrait for MyReflectCompoent {
    fn field_count(&self) -> usize {
        3
    }
}

#[reflect_trait]
trait MyReflectTrait {
    fn field_count(&self) -> usize;
}

#[derive(Default, Resource)]
struct MyResource {
    field_1: u32,
    field_2: String,
}

#[derive(Resource)]
struct InsertMyResource {
    field_1: f64,
    field_2: u128,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Start,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, States)]
enum InsertGameState {
    Menu,
    Start,
}

#[derive(Event)]
struct MyEvent {
    field_1: usize,
    field_2: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
struct MySystemSet;

fn system() {
    info!("system");
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
