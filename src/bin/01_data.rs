use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, time::common_conditions::once_after_real_delay};
use std::time::Duration;

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        .add_systems(Update, app_exit.run_if(once_after_real_delay(Duration::from_secs(3))))
        .add_systems(Startup, spawn_entities)
        .add_systems(Update, player_infos)
        .add_systems(Startup, spawn_player_bundle)
        .add_systems(Startup, setup_game_settings)
        .add_systems(Update, game_settings_info)
        .run()
}

#[derive(Debug, Component)]
struct Translation {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Camera;

#[derive(Debug, Component)]
struct Health(f32);

// 添加实体
fn spawn_entities(mut commands: Commands) {
    commands.spawn((
        Translation {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        Player,
        Health(150.),
    ));
    commands.spawn((
        Translation {
            x: 5.,
            y: 7.,
            z: 0.,
        },
        Enemy,
        Health(100.),
    ));
    commands.spawn((
        Translation {
            x: 20.,
            y: 13.,
            z: 0.,
        },
        Camera,
    ));
    commands.spawn((
        Translation {
            x: 79.,
            y: 43.,
            z: 0.,
        },
        Enemy,
        Health(250.),
    ));
}

// 查询 Health 组件数据
fn player_infos(health: Query<&Health>) {
    for health in &health {
        info!("health: {health:?}");
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    translation: Translation,
    player: Player,
    health: Health,
}

fn spawn_player_bundle(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        translation: Translation {
            x: 4.,
            y: 3.,
            z: 0.,
        },
        player: Player,
        health: Health(50.),
    });
}

#[derive(Debug, Resource)]
struct GameSettings {
    current_level: u32,
    difficulty: u32,
    max_time_seconds: u32,
}

// 插入资源
fn setup_game_settings(mut commands: Commands) {
    commands.insert_resource(GameSettings {
        current_level: 1,
        difficulty: 100,
        max_time_seconds: 60,
    });
}

// 读取资源数据
fn game_settings_info(settings: Res<GameSettings>) {
    info!("game settings: {settings:?}");
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}