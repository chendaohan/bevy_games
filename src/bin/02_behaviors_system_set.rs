use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*,
    time::common_conditions::once_after_real_delay,
};
use std::time::Duration;

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        // 配置系统集
        .configure_sets(Update, MySystemSet::First.before(MySystemSet::Last))
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(5))),
        )
        // 把系统添加到系统集中
        .add_systems(Update, system::<1>.in_set(MySystemSet::First))
        .add_systems(Update, (system::<2>, system::<3>).in_set(MySystemSet::Last))
        .run()
}

// 创建系统集
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
enum MySystemSet {
    First,
    Last,
}

fn system<const N: usize>() {
    info!("system_{N}");
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
