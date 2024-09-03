use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*,
    time::common_conditions::once_after_real_delay,
};
use std::time::Duration;

fn main() -> AppExit {
    // 实现了 Fn / FnMut / FnOnce Trait 的函数，fn() 是类型
    fn function() {
        info!("function");
    }

    let name = String::from("fn_closure");
    // 实现了 Fn / FnMut / FnOnce Trait 的闭包
    let fn_closure = move || info!("{name}");

    let mut name = String::from("fn_mut");
    // 实现了 FnMut / FnOnce Trait 的闭包
    let fn_mut_closure = move || {
        name.push_str("_closure");
        info!("{name}");
    };

    let name = String::from("fn_once_closure");
    // 实现了 FnOnce Trait 的闭包
    let fn_once_closure = || {
        info!("{name}");
        // 消耗掉所有权
        {
            name
        };
    };

    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        .add_systems(Update, (function, fn_closure, fn_mut_closure))
        // .add_systems(Update, fn_once_closure)
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(3))),
        )
        .run()
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
