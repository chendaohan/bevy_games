use bevy::{
    color::palettes::tailwind,
    input::common_conditions::input_just_pressed,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
}

#[derive(Component)]
struct GameMenu;

#[derive(Component)]
struct GamePaused;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct ReturnMenuButton;

#[derive(Component)]
struct MyBall;

fn main() -> AppExit {
    App::new()
        .add_plugins((DefaultPlugins, menu_plugin, play_plugin))
        .init_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, buttons_background_color)
        .run()
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn buttons_background_color(
    mut buttons: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background_color) in &mut buttons {
        let color = match interaction {
            Interaction::Pressed => Srgba::hex("ff595e").unwrap(),
            Interaction::Hovered => Srgba::hex("6a4c93").unwrap(),
            Interaction::None => Srgba::hex("ffca3a").unwrap(),
        };
        background_color.0 = color.into();
    }
}

fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), spawn_game_menu)
        .add_systems(OnExit(GameState::Menu), despawn_game_menu)
        .add_systems(
            Update,
            (start_button_action, exit_button_action).run_if(in_state(GameState::Menu)),
        );
}

fn spawn_game_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    ..default()
                },
                background_color: BackgroundColor(tailwind::SKY_300.with_luminance(0.7).into()),
                ..default()
            },
            GameMenu,
        ))
        .with_children(|parent| {
            let button_bundle = ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(350.),
                    height: Val::Px(120.),
                    margin: UiRect::vertical(Val::Px(30.)),
                    border: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: BackgroundColor(Srgba::hex("ffca3a").unwrap().into()),
                border_radius: BorderRadius::all(Val::Px(60.)),
                border_color: BorderColor(Color::BLACK),
                ..default()
            };

            let text_style = TextStyle {
                font_size: 90.,
                color: Srgba::hex("ad343e").unwrap().into(),
                ..default()
            };

            parent
                .spawn((button_bundle.clone(), StartButton))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Start", text_style.clone()));
                });

            parent
                .spawn((button_bundle, ExitButton))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Exit", text_style));
                });
        });
}

fn despawn_game_menu(mut commands: Commands, game_menu: Query<Entity, With<GameMenu>>) {
    let Ok(entity) = game_menu.get_single() else {
        return;
    };
    commands.entity(entity).despawn_recursive();
}

fn start_button_action(
    start_button: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(interaction) = start_button.get_single() else {
        return;
    };
    if let Interaction::Pressed = interaction {
        next_state.set(GameState::Playing);
    }
}

fn exit_button_action(
    exit_button: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut exit_writer: EventWriter<AppExit>,
) {
    let Ok(interaction) = exit_button.get_single() else {
        return;
    };
    if let Interaction::Pressed = interaction {
        exit_writer.send_default();
    }
}

fn play_plugin(app: &mut App) {
    app.add_systems(
        OnTransition {
            exited: GameState::Menu,
            entered: GameState::Playing,
        },
        spawn_balls,
    )
    .add_systems(OnEnter(GameState::Paused), spawn_paused_page)
    .add_systems(OnExit(GameState::Paused), despawn_paused_page)
    .add_systems(
        OnTransition {
            exited: GameState::Paused,
            entered: GameState::Menu,
        },
        despawn_game,
    )
    .add_systems(Update, rotate_balls.run_if(in_state(GameState::Playing)))
    .add_systems(
        Update,
        toggle_palying_paused.run_if(
            input_just_pressed(KeyCode::Space)
                .and_then(in_state(GameState::Playing).or_else(in_state(GameState::Paused))),
        ),
    )
    .add_systems(
        Update,
        return_menu_button_action.run_if(in_state(GameState::Paused)),
    );
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let circle_handle = Mesh2dHandle(meshes.add(Circle::new(30.)));
    for row in -2..=2 {
        for column in -3..=3 {
            let color = Srgba::WHITE
                .with_red((column + 3) as f32 / 7.)
                .with_green((row + 2) as f32 / 5.);
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: circle_handle.clone(),
                    material: materials.add(Color::Srgba(color)),
                    transform: Transform::from_xyz(column as f32 * 100., row as f32 * 100., 0.),
                    ..default()
                },
                MyBall,
            ));
        }
    }
}

fn spawn_paused_page(mut commands: Commands) {
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
                background_color: BackgroundColor(tailwind::GRAY_300.with_alpha(0.5).into()),
                ..default()
            },
            GamePaused,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Paused",
                TextStyle {
                    font_size: 220.,
                    color: Srgba::RED.into(),
                    ..default()
                },
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(600.),
                            height: Val::Px(120.),
                            border: UiRect::all(Val::Px(10.)),
                            margin: UiRect::vertical(Val::Px(100.)),
                            ..default()
                        },
                        background_color: BackgroundColor(Srgba::hex("ffca3a").unwrap().into()),
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::all(Val::Px(60.)),
                        ..default()
                    },
                    ReturnMenuButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Return Menu",
                        TextStyle {
                            font_size: 80.,
                            ..default()
                        },
                    ));
                });
        });
}

fn return_menu_button_action(
    return_menu_button: Query<&Interaction, (Changed<Interaction>, With<ReturnMenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(interaction) = return_menu_button.get_single() else {
        return;
    };
    if let Interaction::Pressed = interaction {
        next_state.set(GameState::Menu);
    }
}

fn rotate_balls(mut balls: Query<&mut Transform, With<MyBall>>, time: Res<Time>) {
    let rotation = std::f32::consts::PI / 2. * time.delta_seconds();
    for mut transform in &mut balls {
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(rotation));
    }
}

fn toggle_palying_paused(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    match state.get() {
        GameState::Playing => next_state.set(GameState::Paused),
        GameState::Paused => next_state.set(GameState::Playing),
        GameState::Menu => (),
    }
}

fn despawn_game(mut commands: Commands, balls: Query<Entity, With<MyBall>>) {
    for entity in &balls {
        commands.entity(entity).despawn();
    }
}

fn despawn_paused_page(mut commands: Commands, game_paused: Query<Entity, With<GamePaused>>) {
    if let Ok(entity) = game_paused.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
