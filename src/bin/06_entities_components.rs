use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*, time::common_conditions::once_after_real_delay};

fn main() -> AppExit {
    App::new()
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(1))))
        .add_plugins(LogPlugin::default())
        .add_systems(Startup, (spawn_players, spawn_enemies))
        .add_systems(Update, (despawn_my_players, despawn_my_enemies))
        .add_systems(Update, app_exit.run_if(once_after_real_delay(Duration::from_secs(5))))
        .run()
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Level(u32);

// 演示孤儿原则
trait MyTrait {
    fn my_trait(&self);
}

impl MyTrait for u32 {
    fn my_trait(&self) {
        println!("my_trait: {self}");
    }
}

#[test]
fn test_my_trait() {
    2_u32.my_trait();
}

#[derive(Component)]
struct Health(f32);

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    level: Level,
    health: Health,
}

#[derive(Component)]
struct MyPlayer;

#[derive(Component)]
struct MyEnemy;

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    level: Level,
    health: Health,
}

// 使用 Commands 来创建实体
fn spawn_players(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        player: Player,
        level: Level(5),
        health: Health(104.5),
    });

    commands.spawn((
        PlayerBundle {
            player: Player,
            level: Level(10),
            health: Health(500.35),
        },
        MyPlayer,
    ));
}

// 使用独占系统来创建实体
fn spawn_enemies(world: &mut World) {
    world.spawn(EnemyBundle {
        enemy: Enemy,
        level: Level(7),
        health: Health(80.7),
    });

    world.spawn((
        EnemyBundle {
            enemy: Enemy,
            level: Level(8),
            health: Health(52.1),
        },
        MyEnemy,
    ));
}

// 使用命令来删除有 MyPlayer 组件的实体
fn despawn_my_players(mut commands: Commands, my_players: Query<Entity, With<MyPlayer>>) {
    for entity in &my_players {
        info!("player entity: {entity}");
        commands.entity(entity).despawn();
    }
}

// 使用独占系统删除有 MyEnemy 组件的实体
fn despawn_my_enemies(world: &mut World) {
    let mut enemies = world.query_filtered::<Entity, With<MyEnemy>>();
    for entity in enemies.iter(world).collect::<Vec<Entity>>() {
        info!("enemy entity: {entity}");
        world.despawn(entity);
    }
}


fn app_exit(mut exit_writer: EventWriter<AppExit>) {
    info!("app_exit");
    exit_writer.send(AppExit::Success);
}