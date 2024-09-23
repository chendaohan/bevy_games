use std::time::Duration;

use bevy::{
    app::{MainScheduleOrder, ScheduleRunnerPlugin},
    ecs::schedule::ScheduleLabel,
    log::LogPlugin,
    prelude::*,
    state::app::StatesPlugin,
    time::common_conditions::once_after_real_delay,
};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))),
            StatesPlugin,
            LogPlugin::default(),
        ))
        // .add_plugins(configure_system_plugin)
        .add_plugins(schedules_plugin)
        .add_plugins(custom_schedule_plugin)
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(2))),
        )
        .insert_state(MyState::One)
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(2)))
        .insert_resource(Time::<Fixed>::from_hz(2.))
        .run()
}

fn configure_system_plugin(app: &mut App) {
    // 配置系统集
    app.configure_sets(Startup, MySystemSet.after(system::<0>))
        // 运行条件
        .add_systems(Startup, system::<0>.run_if(return_true))
        // 排序
        .add_systems(Startup, (system::<1>, system::<2>).chain())
        // 系统集
        .add_systems(Startup, system::<3>.in_set(MySystemSet));
}

fn schedules_plugin(app: &mut App) {
    // Main
    app.add_systems(PreStartup, system::<0>)
        .add_systems(Startup, system::<1>)
        .add_systems(PostStartup, system::<2>)

        .add_systems(First, system::<3>)
        .add_systems(PreUpdate, (system::<4>, change_state.run_if(run_once())))
        // StateTransition
        .add_systems(OnExit(MyState::One), system::<5>)
        .add_systems(
            OnTransition {
                exited: MyState::One,
                entered: MyState::Two,
            },
            system::<6>,
        )
        .add_systems(OnEnter(MyState::Two), system::<7>)
        // RunFixedMainLoop
        .add_systems(FixedFirst, system::<8>)
        .add_systems(FixedPreUpdate, system::<9>)
        .add_systems(FixedUpdate, system::<10>)
        .add_systems(FixedPostUpdate, system::<11>)
        .add_systems(FixedLast, system::<12>)

        .add_systems(Update, system::<13>)
        .add_systems(PostUpdate, system::<14>)
        .add_systems(Last, system::<15>);
}

fn custom_schedule_plugin(app: &mut App) {
    let my_schedule = Schedule::new(MyScheddule);
    app.add_schedule(my_schedule);

    app.world_mut()
        .resource_mut::<MainScheduleOrder>()
        .insert_before(Update, MyScheddule);

    app.add_systems(MyScheddule, system::<16>);
}

// Schedule 标签
#[derive(Debug, ScheduleLabel, Clone, PartialEq, Eq, Hash)]
struct MyScheddule;

// 系统集
#[derive(Debug, SystemSet, Clone, PartialEq, Eq, Hash)]
struct MySystemSet;

// 状态
#[derive(Debug, States, Clone, PartialEq, Eq, Hash)]
enum MyState {
    One,
    Two,
}

fn change_state(mut state: ResMut<NextState<MyState>>) {
    state.set(MyState::Two);
}

fn system<const N: usize>() {
    info!("system {N}");
}

fn return_true() -> bool {
    info!("return true");
    true
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app exit");
    exit_writer.send_default();
}
