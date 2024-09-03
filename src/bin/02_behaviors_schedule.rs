use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, state::app::StatesPlugin,
    time::common_conditions::once_after_real_delay,
};
use std::time::Duration;

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins((LogPlugin::default(), StatesPlugin))
        .init_state::<GameState>()
        .insert_resource(Time::<Virtual>::from_max_delta(Duration::from_secs(2)))
        .insert_resource(Time::<Fixed>::from_hz(2.)) // 每秒运行两次
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(10))),
        )
        .add_systems(
            Update,
            change_state(GameState::Start).run_if(once_after_real_delay(Duration::from_secs(4))),
        )
        .add_systems(
            Update,
            change_state(GameState::Paused).run_if(once_after_real_delay(Duration::from_secs(7))),
        )
        // 应用程序启动时运行
        .add_systems(Startup, system::<1>)
        // 每帧运行
        .add_systems(Update, system::<2>)
        // 固定时间运行
        .add_systems(FixedUpdate, system::<3>)
        // 状态转换时运行
        // 进入 GameState::Start 时运行
        .add_systems(OnEnter(GameState::Start), system::<4>)
        // 退出 GameState::Menu 时运行
        .add_systems(OnExit(GameState::Menu), system::<5>)
        // 退出 GameState::Start ，进入 GameState::Paused 时运行
        .add_systems(
            OnTransition {
                exited: GameState::Start,
                entered: GameState::Paused,
            },
            system::<6>,
        )
        .run()
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Start,
    Paused,
}

fn system<const N: usize>() {
    info!("system_{N}");
}

fn change_state(next_state: GameState) -> impl FnMut(ResMut<NextState<GameState>>) {
    move |mut state| {
        info!("=================================");
        info!("change_state");
        state.set(next_state.clone());
    }
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
