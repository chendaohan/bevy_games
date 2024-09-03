use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*,
    time::common_conditions::once_after_real_delay,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        .add_systems(
            Startup,
            (
                (spawn_players, spawn_enemies),
                (
                    level_info,
                    add_health,
                    health_info_my_player_add_marker,
                    position_entity_info,
                    level_info_by_parent,
                    health_info_by_children,
                    level_combinations,
                    my_player_health,
                    without_my_player_health,
                    my_player_and_player_level,
                    my_player_or_player_level,
                ),
            )
                .chain(),
        )
        .add_systems(
            Update,
            app_exit.run_if(once_after_real_delay(Duration::from_secs(3))),
        )
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

#[derive(Component)]
struct MyPlayer;

#[derive(Component)]
struct SubPlayer;

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
    let sub_entities: Vec<Entity> = (0..5)
        .map(|number| {
            commands
                .spawn((
                    PlayerBundle {
                        level: Level(number),
                        health: Health(number),
                        ..default()
                    },
                    SubPlayer,
                ))
                .id()
        })
        .collect();

    commands
        .spawn((
            PlayerBundle {
                level: Level(21),
                health: Health(94),
                position: Position(Vec2::new(10., 27.)),
                ..default()
            },
            MyPlayer,
        ))
        .push_children(&sub_entities);

    for number in 0_u32..10_u32 {
        commands.spawn(PlayerBundle {
            level: Level(number),
            health: Health(number),
            ..default()
        });
    }
}

fn spawn_enemies(mut commands: Commands) {
    for number in 0_u32..20_u32 {
        commands.spawn(EnemyBundle {
            level: Level(number),
            health: Health(number),
            ..default()
        });
    }
}

// 打印等级
fn level_info(levels: Query<&Level>) {
    levels.iter().for_each(|level| {
        info!("level: {level:?}");
    });
}

// 加生命
fn add_health(mut health: Query<&mut Health>) {
    health.iter_mut().for_each(|mut health| {
        health.0 += 1;
    });
}

// 打印生命，如果有 MyPlayer 组件添加标记
fn health_info_my_player_add_marker(health: Query<(&Health, Option<&MyPlayer>)>) {
    health.iter().for_each(|(health, my_player)| {
        if let Some(_) = my_player {
            info!("my player health: {health:?}");
        } else {
            info!("health: {health:?}");
        }
    });
}

// 打印位置，和有位置组件的实体
fn position_entity_info(positions: Query<(Entity, &Position)>) {
    positions.iter().for_each(|(entity, position)| {
        info!("entity: {entity}, position: {position:?}");
    });
}

// 读取父级的等级
fn level_info_by_parent(parents: Query<&Parent>, levels: Query<&Level>) {
    let parent = parents.single().get();
    if let Ok(level) = levels.get(parent) {
        info!("parent level: {level:?}");
    }
}

// 读取子级的等级
fn health_info_by_children(children: Query<&Children>, health: Query<&Health>) {
    let Ok(children) = children.get_single() else {
        return;
    };
    let mut children = children.iter();
    let entity_1 = *children.next().unwrap();
    let entity_2 = *children.next().unwrap();
    let Ok([health_1, health_2]) = health.get_many([entity_1, entity_2]) else {
        return;
    };
    println!("many child health_1: {health_1:?}, health_2: {health_2:?}");

    for health in health.iter_many(children) {
        info!("child health: {health:?}");
    }
}

// 等级组合
fn level_combinations(levels: Query<&Level>) {
    levels.iter_combinations().for_each(|[level1, level2]| {
        info!("level 1: {level1:?}, level 2: {level2:?}");
    });
}

// my player 的生命
fn my_player_health(health: Query<&Health, With<MyPlayer>>) {
    health.iter().for_each(|health| {
        info!("my player health: {health:?}");
    })
}

// 没有 my player 的生命
fn without_my_player_health(health: Query<&Health, Without<MyPlayer>>) {
    health.iter().for_each(|health| {
        info!("without my player health: {health:?}");
    });
}

// 有 my palyer 和 player 的等级
fn my_player_and_player_level(levels: Query<&Level, (With<MyPlayer>, With<Player>)>) {
    levels.iter().for_each(|level| {
        info!("my player level: {level:?}");
    })
}

// 有 my player 或 player 的等级
fn my_player_or_player_level(levels: Query<&Level, Or<(With<MyPlayer>, With<Player>)>>) {
    levels.iter().for_each(|level| {
        info!("player level: {level:?}");
    });
}

fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}
