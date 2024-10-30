use bevy::prelude::*;

use crate::{spawn_button, AppDefaultFont};

use super::{pressed_return_start_menu_button, GamePhase, ReturnStartMenuButton};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GamePhase::Menu), spawn_menu_page)
        .add_systems(
            Update,
            pressed_return_game_button.run_if(in_state(GamePhase::Menu)),
        )
        .add_systems(Update, pressed_return_start_menu_button.run_if(in_state(GamePhase::Menu)));
}

// 返回游戏按钮
#[derive(Component)]
struct ReturnGameButton;

// 生成菜单页面
fn spawn_menu_page(mut commands: Commands, default_font: Res<AppDefaultFont>) {
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
            StateScoped(GamePhase::Menu),
        ))
        .with_children(|parent| {
            spawn_button(
                parent,
                ReturnGameButton,
                "返回游戏",
                default_font.clone(),
                UiRect::ZERO,
            );
            spawn_button(
                parent,
                ReturnStartMenuButton,
                "开始菜单",
                default_font.clone(),
                UiRect::top(Val::Percent(10.)),
            );
        });
}

// 按下返回游戏按钮
fn pressed_return_game_button(
    button: Query<&Interaction, (Changed<Interaction>, With<ReturnGameButton>)>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    let Ok(interaction) = button.get_single() else {
        return;
    };
    if let Interaction::Pressed = interaction {
        next_state.set(GamePhase::Playing);
    }
}
