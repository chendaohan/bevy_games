use std::time::Duration;

use avian2d::prelude::{Collision, LinearVelocity};
use bevy::{color::palettes::tailwind, prelude::*};

use crate::AppDefaultFont;

use super::{
    collider::{FirePitSensor, FoodSensor, PlayerRigidBody, SpikeSensor},
    open_close_menu_page, GamePhase, PlayerAnimation, Score, ScoreText, FOOD_SCORE,
};

// 移动速度
const MOVEMENT_SPEED: f32 = 3.;
// 跳跃速度
const JUMP_SPEED: f32 = 4.;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GamePhase::Playing), spawn_score_text)
        .add_systems(
            Update,
            (
                update_score_text,
                player_movement,
                player_jump,
                player_eat_food,
                contact_fire_pit_or_spike,
                game_over,
                control_walk_animation,
                open_close_menu_page,
            )
                .run_if(in_state(GamePhase::Playing)),
        );
}

// 生成分数文本
fn spawn_score_text(mut commands: Commands, default_font: Res<AppDefaultFont>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            StateScoped(GamePhase::Playing),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(300.),
                        height: Val::Px(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(tailwind::SKY_500.with_alpha(0.4).into()),
                    ..default()
                })
                .with_children(|parent| {
                    let text_style = TextStyle {
                        font: default_font.clone(),
                        font_size: 80.,
                        color: Color::WHITE,
                    };
                    parent.spawn((
                        TextBundle::from_sections([
                            TextSection {
                                value: "分数：".into(),
                                style: text_style.clone(),
                            },
                            TextSection {
                                value: "0".into(),
                                style: text_style,
                            },
                        ]),
                        ScoreText,
                    ));
                });
        });
}

// 更新分数文本
fn update_score_text(mut score_text: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        let Ok(mut text) = score_text.get_single_mut() else {
            return;
        };
        text.sections[1].value = score.0.to_string();
    }
}

// 玩家移动
fn player_movement(
    mut player: Query<&mut LinearVelocity, With<PlayerRigidBody>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let speed = if keyboard.pressed(KeyCode::KeyA) {
        -MOVEMENT_SPEED
    } else if keyboard.pressed(KeyCode::KeyD) {
        MOVEMENT_SPEED
    } else {
        0.
    };
    let Ok(mut linear_velocity) = player.get_single_mut() else {
        return;
    };
    linear_velocity.0.x = speed;
}

// 玩家跳跃
fn player_jump(
    mut player: Query<&mut LinearVelocity, With<PlayerRigidBody>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut speed = 0.;
    if keyboard.just_pressed(KeyCode::KeyJ) {
        speed += JUMP_SPEED;
    }
    let Ok(mut linear_velocity) = player.get_single_mut() else {
        return;
    };
    linear_velocity.0.y = (speed + linear_velocity.0.y).clamp(-JUMP_SPEED, JUMP_SPEED);
}

// 玩家吃食物
fn player_eat_food(
    mut commands: Commands,
    player: Query<Entity, With<PlayerRigidBody>>,
    foods: Query<(), With<FoodSensor>>,
    mut collision_reader: EventReader<Collision>,
    mut score: ResMut<Score>,
) {
    let Ok(player_entity) = player.get_single() else {
        return;
    };
    for collision in collision_reader.read() {
        if collision.0.is_sensor {
            let food_entity = if collision.0.entity1 == player_entity {
                collision.0.entity2
            } else if collision.0.entity2 == player_entity {
                collision.0.entity1
            } else {
                Entity::PLACEHOLDER
            };
            if food_entity != Entity::PLACEHOLDER && foods.contains(food_entity) {
                score.0 += FOOD_SCORE;
                commands.entity(food_entity).despawn_recursive();
            }
        }
    }
}

// 接触火坑或尖刺
fn contact_fire_pit_or_spike(
    mut player: Query<(Entity, &mut Transform, &mut LinearVelocity), With<PlayerRigidBody>>,
    fire_pits: Query<(), With<FirePitSensor>>,
    spikes: Query<(), With<SpikeSensor>>,
    mut collision_reader: EventReader<Collision>,
) {
    let Ok((player_entity, mut player_transform, mut linear_velocity)) = player.get_single_mut()
    else {
        return;
    };
    for collision in collision_reader.read() {
        let fire_pit_or_spike_entity = if collision.0.entity1 == player_entity {
            collision.0.entity2
        } else if collision.0.entity2 == player_entity {
            collision.0.entity1
        } else {
            Entity::PLACEHOLDER
        };
        if fire_pit_or_spike_entity != Entity::PLACEHOLDER
            && (fire_pits.contains(fire_pit_or_spike_entity)
                || spikes.contains(fire_pit_or_spike_entity))
        {
            player_transform.translation = Vec3::new(-18.837, 0.1, 0.);
            linear_velocity.0 = Vec2::ZERO;
        }
    }
}

// 游戏结束
fn game_over(foods: Query<(), With<FoodSensor>>, mut next_state: ResMut<NextState<GamePhase>>) {
    if foods.iter().len() == 0 {
        next_state.set(GamePhase::Over);
    }
}

// 控制行走动画
fn control_walk_animation(
    mut player: Query<(&mut AnimationTransitions, &mut AnimationPlayer), With<PlayerRigidBody>>,
    animation: Res<PlayerAnimation>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut animation_transitions, mut animation_palyer)) = player.get_single_mut() else {
        return;
    };
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::KeyD) {
        if animation_palyer.all_finished() {
            animation_transitions.play(
                &mut animation_palyer,
                animation.animation,
                Duration::from_millis(300),
            );
        }
    } else if !animation_palyer.all_finished() {
        animation_palyer.stop(animation.animation);
    }
}
