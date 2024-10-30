mod game;
mod start_menu;

use avian2d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

// 处于不同状态下的按钮颜色
const BUTTON_HOVERED: Color = Color::Srgba(tailwind::FUCHSIA_500);
const BUTTON_PRESSED: Color = Color::Srgba(tailwind::RED_600);
const BUTTON_NONE: Color = Color::Srgba(tailwind::ORANGE_500);

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "assets/tiny_blue".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tiny Blue".into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                }),
            PhysicsPlugins::default(),
        ))
        // 开发时使用的插件
        // .add_plugins((
        //     WorldInspectorPlugin::default(),
        //     PhysicsDebugPlugin::default(),
        // ))
        .add_plugins((start_menu::plugin, game::plugin))
        .init_state::<AppState>()
        .enable_state_scoped_entities::<AppState>()
        .init_resource::<AppDefaultFont>()
        .add_systems(Update, switch_button_background_color)
        .run()
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash, States)]
enum AppState {
    #[default]
    StartMenu,
    Game,
}

#[derive(Resource, Deref)]
struct AppDefaultFont(Handle<Font>);

impl FromWorld for AppDefaultFont {
    fn from_world(world: &mut World) -> Self {
        Self(world.resource::<AssetServer>().load("NotoSansSC-Bold.ttf"))
    }
}

// 切换按钮的背景颜色
fn switch_button_background_color(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background_color) in &mut buttons {
        match interaction {
            Interaction::Hovered => background_color.0 = BUTTON_HOVERED,
            Interaction::Pressed => background_color.0 = BUTTON_PRESSED,
            Interaction::None => background_color.0 = BUTTON_NONE,
        }
    }
}

// 生成按钮，这是函数
fn spawn_button<T: Bundle>(
    parent: &mut ChildBuilder,
    marker: T,
    text: &str,
    font: Handle<Font>,
    margin: UiRect,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(340.),
                    height: Val::Px(130.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(10.)),
                    margin,
                    ..default()
                },
                background_color: BackgroundColor(BUTTON_NONE),
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Percent(25.)),
                ..default()
            },
            marker,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 90.,
                    color: Color::WHITE,
                },
            ));
        });
}
