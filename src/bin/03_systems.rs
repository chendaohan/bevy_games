use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*,
    time::common_conditions::once_after_real_delay,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        .insert_resource(PlayersCount(0))
        .add_systems(Startup, spawn_players)
        .add_systems(
            Update,
            (
                count_players,
                players_count_info,
                (increment_player_values, player_health_info),
            ),
        )
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(3))),
        )
        .run()
}

// 玩家组件
#[derive(Component)]
struct Player {
    level: u32,
    health: u32,
    magic: u32,
}

// 玩家总数
#[derive(Resource)]
struct PlayersCount(usize);

// 生成玩家
fn spawn_players(mut commands: Commands) {
    commands.spawn(Player {
        level: 5,
        health: 64,
        magic: 40,
    });

    commands.spawn_batch([
        Player {
            level: 10,
            health: 73,
            magic: 45,
        },
        Player {
            level: 10,
            health: 100,
            magic: 92,
        },
        Player {
            level: 34,
            health: 534,
            magic: 745,
        },
    ]);
}

// 统计玩家总数
// fn count_players(players: Query<&Player>, mut players_count: ResMut<PlayersCount>)
fn count_players((players, mut players_count): (Query<&Player>, ResMut<PlayersCount>)) {
    let count = players.into_iter().count();
    players_count.0 = count;
}

// 打印玩家总数
fn players_count_info(players_count: Res<PlayersCount>) {
    info!("players count: {}", players_count.0);
}

// 增加玩家值
fn increment_player_values(mut players: Query<&mut Player>) {
    for mut player in &mut players {
        player.level += 1;
        player.health += 1;
        player.magic += 2;
    }
}

// 打印玩家生命值
fn player_health_info(players: Query<&Player>) {
    for player in &players {
        info!("player health: {}", player.health);
    }
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
