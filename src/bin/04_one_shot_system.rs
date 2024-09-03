use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, ecs::system::SystemId, log::LogPlugin, prelude::*,
    time::common_conditions::once_after_real_delay,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        .add_event::<MySystemId>()
        .add_systems(Startup, (register_and_run_system, run_system).chain())
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(3))),
        )
        .run()
}

#[derive(Event)]
struct MySystemId(SystemId);

fn register_and_run_system(mut commands: Commands, mut system_id_writer: EventWriter<MySystemId>) {
    info!("register_and_run_system");
    let system_id = commands.register_one_shot_system(one_shot_system);
    commands.run_system(system_id);
    system_id_writer.send(MySystemId(system_id));
    // system_id_writer.send_batch([MySystemId(system_id), MySystemId(system_id)]);
}

fn run_system(mut commands: Commands, mut system_id_reader: EventReader<MySystemId>) {
    info!("run_system");
    for &MySystemId(system_id) in system_id_reader.read() {
        commands.run_system(system_id);
    }
}

fn one_shot_system() {
    info!("one_shot_system");
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
