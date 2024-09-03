use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, time::common_conditions::once_after_real_delay};

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        // 初始化资源
        .init_resource::<ResourceTwo>()
        .init_resource::<ResourceThree>()
        // 插入资源
        .insert_resource(ResourceOne { field_1: 0, field_2: 0. })
        .add_systems(Startup, manage_resource)
        .add_systems(Update, (access_resource, change_resource))
        .add_systems(Update, app_exit.run_if(once_after_real_delay(Duration::from_secs(5))))
        .run()
}

#[derive(Resource)]
struct ResourceOne {
    field_1: u32,
    field_2: f32,
}

#[derive(Resource, Default)]
struct ResourceTwo {
    field_1: u64,
    field_2: f64,
}

#[derive(Resource)]
struct ResourceThree {
    field_1: u8,
    field_2: u16,
}

impl FromWorld for ResourceThree {
    fn from_world(_world: &mut World) -> Self {
        Self {
            field_1: 0,
            field_2: 0,
        }
    }
}

// 管理资源
fn manage_resource(mut commands: Commands) {
    info!("manage_resource");
    // 初始化资源
    commands.init_resource::<ResourceTwo>();
    commands.init_resource::<ResourceThree>();
    // 插入资源
    commands.insert_resource(ResourceOne { field_1: 0, field_2: 0. });
    // 删除资源
    commands.remove_resource::<ResourceThree>();
}

// 访问资源
fn access_resource(resource_1: Res<ResourceOne>, resource_2: Option<Res<ResourceTwo>>) {
    info!("resource_1 field_1: {}", resource_1.field_1);
    // 用 Option 包裹的资源要先判断是否存在
    if let Some(resource_2) = resource_2 {
        info!("resource_2 field_2: {}", resource_2.field_2);
    }
}

// 修改资源
fn change_resource(mut resource_1: ResMut<ResourceOne>, resource_2: Option<ResMut<ResourceTwo>>) {
    resource_1.field_1 += 1;
    if let Some(mut resource_2) = resource_2 {
        resource_2.field_2 += 1.;
    }
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}