use bevy::{color::palettes::tailwind, prelude::*};

use crate::{spawn_button, AppDefaultFont};

use super::{pressed_return_start_menu_button, GamePhase, ReturnStartMenuButton, Score};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GamePhase::Over), spawn_game_over_page)
        .add_systems(
            Update,
            pressed_return_start_menu_button.run_if(in_state(GamePhase::Over)),
        );
}

// 生成游戏结束页面
fn spawn_game_over_page(
    mut commands: Commands,
    default_font: Res<AppDefaultFont>,
    score: Res<Score>,
) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.5)),
                ..default()
            },
            StateScoped(GamePhase::Over),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                format!("分数：{}", score.0),
                TextStyle {
                    font: default_font.clone(),
                    font_size: 160.,
                    color: tailwind::PINK_400.into(),
                },
            ));
            parent.spawn(TextBundle::from_section(
                "Game Over!",
                TextStyle {
                    font: default_font.clone(),
                    font_size: 200.,
                    color: Color::WHITE,
                },
            ));
            spawn_button(
                parent,
                ReturnStartMenuButton,
                "开始菜单",
                default_font.clone(),
                UiRect::ZERO,
            );
        });
}
