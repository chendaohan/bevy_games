use bevy::{prelude::*, render::view::NoFrustumCulling};

use crate::{spawn_button, AppDefaultFont};

use super::{GamePhase, PlayerAnimation};

// 游戏介绍文本
const INTRODUCTION_TEXT: &str = "\
游戏目标：吃掉所有的红色小球
操作方式：A 向左移动，D 向右移动，J 跳跃
";

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GamePhase::Introduction),
        (
            spawn_introduction_page,
            player_no_frustum_culling,
            player_animation_transitions,
        ),
    )
    .add_systems(
        Update,
        pressed_skip_introduction_button.run_if(in_state(GamePhase::Introduction)),
    );
}

// 跳过介绍按钮
#[derive(Component)]
struct SkipIntroductionButton;

// 生成介绍页面
fn spawn_introduction_page(mut commands: Commands, default_font: Res<AppDefaultFont>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.5)),
                ..default()
            },
            StateScoped(GamePhase::Introduction),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                INTRODUCTION_TEXT,
                TextStyle {
                    font: default_font.clone(),
                    font_size: 60.,
                    color: Color::WHITE,
                },
            ));
            spawn_button(
                parent,
                SkipIntroductionButton,
                "跳过介绍",
                default_font.clone(),
                UiRect::top(Val::Percent(10.)),
            );
        });
}

// 按下跳过介绍按钮
fn pressed_skip_introduction_button(
    button: Query<&Interaction, (Changed<Interaction>, With<SkipIntroductionButton>)>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    let Ok(interaction) = button.get_single() else {
        return;
    };
    if let Interaction::Pressed = interaction {
        next_state.set(GamePhase::Playing);
    }
}

// 为玩家添加不视锥体剔除，因为玩家有骨骼动画，所以会将玩家剔除
fn player_no_frustum_culling(mut commands: Commands, bodies: Query<(Entity, &Name)>) {
    for (entity, name) in &bodies {
        let body_names = ["arm", "body", "eye", "leg", "mouth"];
        for body_name in body_names {
            if name.contains(body_name) {
                commands.entity(entity).insert(NoFrustumCulling);
            }
        }
    }
}

// 为玩家添加动画图和动画过渡
fn player_animation_transitions(
    mut commands: Commands,
    player: Query<Entity, With<AnimationPlayer>>,
    animation: Res<PlayerAnimation>,
) {
    let Ok(entity) = player.get_single() else {
        return;
    };
    commands
        .entity(entity)
        .insert((animation.graph.clone(), AnimationTransitions::new()));
}
