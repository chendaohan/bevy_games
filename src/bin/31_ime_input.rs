use bevy::{
    color::palettes::tailwind,
    input::{
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
    window::PrimaryWindow,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "assets/31_ime_input".into(),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                button_interaction,
                pressed_open_button,
                pressed_close_button,
                ime_input,
                pressed_backspace,
            ),
        )
        .run()
}

#[derive(Component)]
struct OpenButton;

#[derive(Component)]
struct CloseButton;

#[derive(Component)]
struct InputText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            background_color: BackgroundColor(Color::Srgba(tailwind::SKY_400)),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                justify_content: JustifyContent::Start,
                                align_items: AlignItems::Center,
                                border: UiRect::all(Val::Px(10.)),
                                width: Val::Px(800.),
                                height: Val::Px(120.),
                                ..default()
                            },
                            background_color: BackgroundColor(Color::WHITE),
                            border_color: BorderColor(Color::BLACK),
                            border_radius: BorderRadius::all(Val::Px(40.)),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_sections([
                                    TextSection::new(
                                        "",
                                        TextStyle {
                                            font: asset_server.load("NotoSansSC-Bold.ttf"),
                                            font_size: 70.,
                                            color: Color::BLACK,
                                        },
                                    ),
                                    TextSection::new(
                                        "",
                                        TextStyle {
                                            font: asset_server.load("NotoSansSC-Bold.ttf"),
                                            font_size: 70.,
                                            color: Color::Srgba(Srgba::RED),
                                        },
                                    ),
                                ]),
                                InputText,
                            ));
                        });

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                width: Val::Px(800.),
                                height: Val::Px(200.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            spawn_button(parent, &asset_server, "Open", OpenButton);
                            spawn_button(parent, &asset_server, "Close", CloseButton);
                        });
                });
        });
}

fn spawn_button<T: Component>(
    parent: &mut ChildBuilder,
    asset_server: &AssetServer,
    text: &str,
    marker: T,
) {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(250.),
                    height: Val::Px(130.),
                    border: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::all(Val::Px(40.)),
                background_color: BackgroundColor(Color::Srgba(tailwind::ORANGE_400)),
                ..default()
            },
            marker,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("NotoSansSC-Bold.ttf"),
                    font_size: 80.,
                    color: Color::WHITE,
                },
            ));
        });
}

fn button_interaction(
    mut buttons: Query<(&mut BackgroundColor, &Interaction), Changed<Interaction>>,
) {
    for (mut background_color, interaction) in &mut buttons {
        match interaction {
            Interaction::None => background_color.0 = Color::Srgba(tailwind::ORANGE_400),
            Interaction::Hovered => background_color.0 = Color::Srgba(tailwind::PURPLE_400),
            Interaction::Pressed => background_color.0 = Color::Srgba(tailwind::RED_600),
        }
    }
}

fn pressed_open_button(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    open_button: Query<&Interaction, (With<OpenButton>, Changed<Interaction>)>,
) {
    let Ok(interaction) = open_button.get_single() else {
        return;
    };
    if let Interaction::Pressed = interaction {
        let Ok(mut window) = window.get_single_mut() else {
            return;
        };
        window.ime_enabled = true;
        window.ime_position = Vec2::new(300., 300.);
    }
}

fn pressed_close_button(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    close_button: Query<&Interaction, (With<CloseButton>, Changed<Interaction>)>,
) {
    let Ok(interaction) = close_button.get_single() else {
        return;
    };
    if let Interaction::Pressed = interaction {
        let Ok(mut window) = window.get_single_mut() else {
            return;
        };
        window.ime_enabled = false;
    }
}

fn ime_input(mut input_text: Query<&mut Text, With<InputText>>, mut ime_reader: EventReader<Ime>) {
    let Ok(mut text) = input_text.get_single_mut() else {
        return;
    };
    for ime in ime_reader.read() {
        match ime {
            Ime::Preedit {
                window: _,
                value,
                cursor: _,
            } => {
                text.sections[1].value = value.into();
            }
            Ime::Commit { window: _, value } => {
                text.sections[0].value.push_str(value);
            }
            _ => (),
        }
    }
}

fn pressed_backspace(
    mut input_text: Query<&mut Text, With<InputText>>,
    mut keyboard_reader: EventReader<KeyboardInput>,
) {
    let Ok(mut text) = input_text.get_single_mut() else {
        return;
    };
    for KeyboardInput {
        key_code: _,
        logical_key,
        state,
        window: _,
    } in keyboard_reader.read()
    {
        if *logical_key == Key::Backspace && *state == ButtonState::Pressed {
            text.sections[0].value.pop();
        }
    }
}
