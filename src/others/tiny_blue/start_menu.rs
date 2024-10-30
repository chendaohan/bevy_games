use bevy::{color::palettes::tailwind, prelude::*};

use crate::{spawn_button, AppDefaultFont, AppState};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::StartMenu), (spawn_ui_camera, spawn_start_menu_page))
        .add_systems(
            Update,
            (pressed_start_game_button, pressed_exit_game_button)
                .run_if(in_state(AppState::StartMenu)),
        );
}

// 开始游戏按钮
#[derive(Component)]
struct StartGameButton;

// 退出游戏按钮
#[derive(Component)]
struct ExitGameButton;

// 生成 UI 相机
fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), StateScoped(AppState::StartMenu)));
}

// 生成开始菜单
fn spawn_start_menu_page(mut commands: Commands, default_font: Res<AppDefaultFont>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(tailwind::GRAY_600.into()),
                ..default()
            },
            StateScoped(AppState::StartMenu),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        height: Val::Percent(100.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    spawn_button(
                        parent,
                        StartGameButton,
                        "开始游戏",
                        default_font.clone(),
                        UiRect::bottom(Val::Percent(20.)),
                    );
                    spawn_button(
                        parent,
                        ExitGameButton,
                        "退出游戏",
                        default_font.clone(),
                        UiRect::ZERO,
                    );
                });
        });
}

// 按下开始游戏按钮
fn pressed_start_game_button(
    button: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Ok(interaction) = button.get_single() else {
        return;
    };
    if let Interaction::Pressed = interaction {
        next_state.set(AppState::Game);
    }
}

// 按下退出游戏按钮
fn pressed_exit_game_button(
    button: Query<&Interaction, (Changed<Interaction>, With<ExitGameButton>)>,
    mut exit_writer: EventWriter<AppExit>,
) {
    let Ok(interaction) = button.get_single() else {
        return;
    };
    if let Interaction::Pressed = interaction {
        exit_writer.send_default();
    }
}
