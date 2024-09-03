use std::time::Duration;

use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, time::common_conditions::once_after_real_delay};

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        .add_systems(Startup, (spawn_players, spawn_enemies))
        .add_systems(Update, query_players)
        .add_systems(Update, app_exit.run_if(once_after_real_delay(Duration::from_secs(3))))
        .run()
}

#[derive(Component, Default)]
struct Player;

#[derive(Component, Default)]
struct Enemy;

#[derive(Component, Default, Debug)]
struct Position(Vec2);

#[derive(Component, Default, Debug)]
struct Level(u32);

#[derive(Component, Default, Debug)]
struct Health(u32);

#[derive(Bundle, Default)]
struct PlayerBundle {
    player: Player,
    position: Position,
    level: Level,
    health: Health,
}

#[derive(Bundle, Default)]
struct EnemyBundle {
    enemy: Enemy,
    position: Position,
    level: Level,
    health: Health,
}

fn spawn_players(mut commands: Commands) {
    commands.spawn((
        PlayerBundle {
            level: Level(5),
            health: Health(72),
            ..default()
        },
        Name::new("小智"),
    ));

    commands.spawn(PlayerBundle {
        level: Level(12),
        health: Health(104),
        position: Position(Vec2::new(5.2, 7.8)),
        ..default()
    });

    commands.spawn((
        Level(15),
        Health(214),
        Position(Vec2::new(78., 64.2)),
    ));
}

fn spawn_enemies(mut commands: Commands) {
    commands.spawn((
        EnemyBundle {
            level: Level(8),
            health: Health(70),
            ..default()
        },
        Name::new("骷髅"),
    ));

    commands.spawn((
        Level(7),
        Position(Vec2::ZERO),
    ));
}

// fn query_player_bundles(bundles: Query<&PlayerBundle>) {}

fn query_players(players: Query<(&Level, &Health, &Position), With<Player>>) {
    for (level, health, position)in &players {
        info!("level: {level:?}, health: {health:?}, position: {position:?}");
    }
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}