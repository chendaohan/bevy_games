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
            app_exit.run_if(once_after_real_delay(Duration::from_secs(5))),
        )
        .add_systems(Update, (system::<2>, system::<1>.after(system::<2>)))
        // .add_systems(Update, system::<1>.after(system::<2>))
        .run()
}

fn system<const N: usize>() {
    info!("system_{N}");
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
