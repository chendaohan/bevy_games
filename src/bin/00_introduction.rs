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
        // 生成实体
        .add_systems(Startup, spawn_player)
        // 查询和修改组件
        .add_systems(Update, (player_infos, reduce_health_and_magic))
        .run()
}

// 定义组件
#[derive(Component)]
struct Player {
    health: u32,
    magic: u32,
}

// 生成两个只有 Player 组件的实体
fn spawn_player(mut commands: Commands) {
    commands.spawn_batch([
        Player {
            health: 10000,
            magic: 50000,
        },
        Player {
            health: 40000,
            magic: 20000,
        },
    ]);
}

// 查询有 Player 组件的实体的 Entity 和 Player 组件的数据
fn player_infos(players: Query<(Entity, &Player)>) {
    for (entity, &Player { health, magic }) in &players {
        info!("entity: {entity}, health_points: {health}, magic_points: {magic}");
    }
}

// 修改 Player 组件的数据
fn reduce_health_and_magic(mut players: Query<&mut Player>) {
    for mut player in &mut players {
        player.health -= 1;
        player.magic -= 1;
    }
}

// 退出 App
fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
