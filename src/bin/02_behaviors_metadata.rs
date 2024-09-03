use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*,
    time::common_conditions::once_after_real_delay,
};
use std::time::Duration;

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(2))),
        )
        // 运行条件
        .add_systems(Startup, system::<1>.run_if(system_2))
        // 排序
        // system::<4> -> system::<3>
        .add_systems(Startup, (system::<3>, system::<4>.before(system::<3>)))
        // system::<5> -> system::<6>
        .add_systems(Startup, (system::<5>, system::<6>.after(system::<5>)))
        // system::<7> -> system::<8> -> system::<9>
        .add_systems(Startup, (system::<7>, system::<8>, system::<9>).chain())
        .run()
}

fn system<const N: usize>() {
    info!("system_{N}");
}

fn system_2() -> bool {
    info!("system_2");
    true
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
