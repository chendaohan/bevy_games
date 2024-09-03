use bevy::{
    color::palettes::css,
    input::{
        common_conditions::input_just_pressed,
        keyboard::{Key, KeyboardInput},
        ButtonState,
    },
    prelude::*,
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        // .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Paused), spawn_pause_text)
        .add_systems(
            Update,
            (
                move_player.run_if(in_state(GameState::Playing)),
                toggle_game_state.run_if(input_just_pressed(KeyCode::Space)),
                (input_text, clear_text).run_if(in_state(GameState::Paused)),
            ),
        )
        .run()
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PauseText;

#[derive(Debug, Clone, PartialEq, Eq, Hash, States, Default)]
enum GameState {
    #[default]
    Playing,
    Paused,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2., 7., 14.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(1., 2., 1.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(1.)),
            material: materials.add(Color::Srgba(css::RED)),
            transform: Transform::from_xyz(0., 1., 0.),
            ..default()
        },
        Player,
        Name::new("Player"),
    ));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.))),
        material: materials.add(Color::Srgba(css::ORANGE)),
        ..default()
    });
}

fn spawn_pause_text(mut commands: Commands) {
    let text = commands
        .spawn(TextBundle::from_section(
            "Paused",
            TextStyle {
                font_size: 180.,
                color: Color::WHITE,
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    ..default()
                },
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.5)),
                ..default()
            },
            PauseText,
            StateScoped(GameState::Paused),
        ))
        .add_child(text);
}

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = player.get_single_mut() else {
        return;
    };
    let speed = 5. * time.delta_seconds();
    let mut velocity = Vec3::ZERO;
    if keyboard.pressed(KeyCode::ArrowUp) {
        velocity += transform.forward() * speed;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        velocity += transform.back() * speed;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        velocity += transform.left() * speed;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        velocity += transform.right() * speed;
    }
    transform.translation += velocity;
}

fn toggle_game_state(
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let GameState::Playing = game_state.get() {
        next_state.set(GameState::Paused);
    } else {
        next_state.set(GameState::Playing);
    }
}

fn input_text(mut text: Query<&mut Text>, mut keyboard_reader: EventReader<KeyboardInput>) {
    let Ok(mut text) = text.get_single_mut() else {
        return;
    };
    for keyboard in keyboard_reader.read() {
        if let ButtonState::Released = keyboard.state {
            continue;
        }
        match &keyboard.logical_key {
            Key::Backspace => {
                text.sections[0].value.pop();
            }
            Key::Character(string) => {
                if string.chars().any(char::is_control) {
                    continue;
                }
                text.sections[0].value.push_str(&string);
            }
            _ => continue,
        }
    }
}

fn clear_text(mut text: Query<&mut Text>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Enter) {
        if let Ok(mut text) = text.get_single_mut() {
            text.sections[0].value.clear();
        }
    }
}
