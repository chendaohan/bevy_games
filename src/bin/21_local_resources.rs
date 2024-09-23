use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(
                1. / 60.,
            ))),
            LogPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (info_my_component, app_exit(60 * 7)))
        .run()
}

#[derive(Component)]
struct MyComponent(u32);

fn setup(mut commands: Commands) {
    commands.spawn(MyComponent(0));
}

#[derive(Default)]
struct FrameCount(usize);

fn info_my_component(mut count: Local<FrameCount>, my_component: Query<&MyComponent>) {
    if count.0 % 60 == 0 {
        if let Ok(my_component) = my_component.get_single() {
            info!("my component: {}", my_component.0);
        }
    }
    count.0 += 1;
}

fn app_exit(mut frame_count: isize) -> impl FnMut(EventWriter<AppExit>) {
    move |mut exit_writer| {
        if frame_count <= 0 {
            exit_writer.send_default();
        }
        frame_count -= 1;
    }
}
