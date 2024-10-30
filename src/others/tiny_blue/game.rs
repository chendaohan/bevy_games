mod collider;
mod introduction;
mod menu;
mod over;
mod playing;

use bevy::{asset::RecursiveDependencyLoadState, prelude::*};

use crate::AppState;

// 游戏场景路径
const SCENE_PATH: &str = "tiny_blue.glb";
// 每个食物的分数
const FOOD_SCORE: usize = 10;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        introduction::plugin,
        collider::plugin,
        playing::plugin,
        over::plugin,
        menu::plugin,
    ))
    .add_sub_state::<GamePhase>()
    .enable_state_scoped_entities::<GamePhase>()
    .add_systems(
        OnEnter(GamePhase::Loading),
        (spawn_scene, insert_score, add_animation_graph),
    )
    .add_systems(
        OnExit(AppState::Game),
        (remove_player_animations, remove_score),
    )
    .add_systems(
        Update,
        check_scene_load_state.run_if(in_state(GamePhase::Loading)),
    )
    .add_systems(
        Update,
        open_close_menu_page
            .run_if(in_state(GamePhase::Playing).or_else(in_state(GamePhase::Menu))),
    );
}

// 游戏的不同阶段
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, SubStates)]
#[source(AppState = AppState::Game)]
enum GamePhase {
    #[default]
    Loading, // 加载游戏场景
    Introduction, // 生成游戏介绍
    Playing,      // 玩游戏
    Over,         // 游戏结束
    Menu,         // 游戏菜单
}

// 玩家动画索引和动画图句柄
#[derive(Resource)]
struct PlayerAnimation {
    animation: AnimationNodeIndex,
    graph: Handle<AnimationGraph>,
}

// 玩家分数
#[derive(Resource)]
struct Score(usize);

// 分数文本
#[derive(Component)]
struct ScoreText;

// 返回开始菜单按钮
#[derive(Component)]
pub struct ReturnStartMenuButton;

// 检查场景加载状态
fn check_scene_load_state(
    scene: Query<&Handle<Scene>>,
    asset_server: Res<AssetServer>,
    mut game_phase: ResMut<NextState<GamePhase>>,
) {
    let Ok(scene) = scene.get_single() else {
        return;
    };
    if let RecursiveDependencyLoadState::Loaded =
        asset_server.recursive_dependency_load_state(scene)
    {
        game_phase.set(GamePhase::Introduction);
    }
}

// 生成场景
fn spawn_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SceneBundle {
            scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(SCENE_PATH)),
            ..default()
        },
        StateScoped(AppState::Game),
    ));
}

// 添加玩家的动画图资产，并插入 PlayerAnimation 资源
fn add_animation_graph(
    mut commands: Commands,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    asset_server: Res<AssetServer>,
) {
    let mut graph = AnimationGraph::new();
    let animation = graph.add_clip(
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(SCENE_PATH)),
        1.,
        graph.root,
    );
    let graph = graphs.add(graph);
    commands.insert_resource(PlayerAnimation { animation, graph });
}

// 删除 PlayerAnimations 资源
fn remove_player_animations(mut commands: Commands) {
    commands.remove_resource::<PlayerAnimation>();
}

// 插入分数资源
fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score(0));
}

// 删除分数资源
fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

// 打开或关闭菜单
fn open_close_menu_page(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<GamePhase>>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        match state.get() {
            GamePhase::Playing => next_state.set(GamePhase::Menu),
            GamePhase::Menu => next_state.set(GamePhase::Playing),
            _ => (),
        }
    }
}

// 按下返回开始菜单按钮
fn pressed_return_start_menu_button(
    buttons: Query<&Interaction, (Changed<Interaction>, With<ReturnStartMenuButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in &buttons {
        if let Interaction::Pressed = interaction {
            next_state.set(AppState::StartMenu);
        }
    }
}
