use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, ecs::world::Command, log::LogPlugin, prelude::*,
    time::common_conditions::once_after_real_delay,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        // 有排序的在下一个系统之前应用
        .add_systems(Startup, (spawn_players, change_player).chain())
        .add_systems(
            Startup,
            (
                custom_commands_spawn_player,
                use_add_enemy_command,
                use_add_enemy_command_ext,
            )
        )
        // 在下一个 Schedule 之前应用
        .add_systems(Update, complete_player_count.run_if(run_once()))
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(3))),
        )
        .run()
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Level(u32);

#[derive(Component)]
struct Health(u32);

struct AddEnemyCommand {
    level: u32,
    health: u32,
}

impl Command for AddEnemyCommand {
    fn apply(self, world: &mut World) {
        world.spawn((Enemy, Level(self.level), Health(self.health)));
    }
}

trait AddEnemyCommandExt {
    fn add_enemy(&mut self, level: u32, health: u32);
}

impl<'w, 's> AddEnemyCommandExt for Commands<'w, 's> {
    fn add_enemy(&mut self, level: u32, health: u32) {
        self.add(AddEnemyCommand { level, health });
    }
}

fn spawn_players(mut commands: Commands) {
    for number in 0..10 {
        commands.spawn((Player, Level(number), Health(number)));
    }
}

// 操作实体
fn change_player(mut commands: Commands, players: Query<Entity, With<Player>>) {
    let mut players = players.iter();
    let player_1 = players.next().unwrap();
    // 从实体中删除组件，向实体插入组件
    commands.entity(player_1).remove::<Player>().insert(Enemy);

    let player_2 = players.next().unwrap();
    // 保留实体中的部分组件
    commands.entity(player_2).retain::<Level>();

    let player_3 = players.next().unwrap();
    // 删除实体
    commands.entity(player_3).despawn();
}

// 统计拥有完整 player 组件的实体数量
fn complete_player_count(players: Query<(), (With<Player>, With<Level>, With<Health>)>) {
    let count = players.iter().count();
    info!("complete player count: {count}");
}

// 自定义命令
fn custom_commands_spawn_player(mut commands: Commands) {
    commands.add(|world: &mut World| {
        world.spawn((Player, Level(45), Health(236)));
    });
}

// 使用 AddEnemyCommand
fn use_add_enemy_command(mut commands: Commands) {
    commands.add(AddEnemyCommand {
        level: 5,
        health: 28,
    });
}

// 使用 AddEnemyExt
fn use_add_enemy_command_ext(mut commands: Commands) {
    commands.add_enemy(34, 356);
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
