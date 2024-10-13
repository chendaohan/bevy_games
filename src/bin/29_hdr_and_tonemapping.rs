use std::f32::consts::PI;

use bevy::{core_pipeline::tonemapping::Tonemapping, pbr::CascadeShadowConfigBuilder, prelude::*};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "assets/29_hdr_and_tonemapping".into(),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_tonemapping_method)
        .run()
}

#[derive(Component)]
struct MyCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.7, 0.7, 1.)
                .looking_at(Vec3::new(0., 0.3, 0.), Vec3::Y),
            ..default()
        },
        MyCamera,
    ));

    commands.spawn(SceneBundle {
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("TonemappingTest/TonemappingTest.gltf")),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server
            .load(GltfAssetLabel::Scene(0).from_asset("FlightHelmet/FlightHelmet.gltf")),
        transform: Transform::from_xyz(0.5, 0., -0.5)
            .with_rotation(Quat::from_rotation_y(-0.15 * PI)),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15_000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            PI * -0.15,
            PI * -0.15,
        )),
        cascade_shadow_config: CascadeShadowConfigBuilder {
            maximum_distance: 3.0,
            first_cascade_far_bound: 0.9,
            ..default()
        }
        .into(),
        ..default()
    });
}

fn toggle_tonemapping_method(
    mut tonemapping: Query<&mut Tonemapping, With<MyCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut method) = tonemapping.get_single_mut() else {
        return;
    };

    if keys.just_pressed(KeyCode::Digit1) {
        *method = Tonemapping::None;
    } else if keys.just_pressed(KeyCode::Digit2) {
        *method = Tonemapping::Reinhard;
    } else if keys.just_pressed(KeyCode::Digit3) {
        *method = Tonemapping::ReinhardLuminance;
    } else if keys.just_pressed(KeyCode::Digit4) {
        *method = Tonemapping::AcesFitted;
    } else if keys.just_pressed(KeyCode::Digit5) {
        *method = Tonemapping::AgX;
    } else if keys.just_pressed(KeyCode::Digit6) {
        *method = Tonemapping::SomewhatBoringDisplayTransform;
    } else if keys.just_pressed(KeyCode::Digit7) {
        *method = Tonemapping::TonyMcMapface;
    } else if keys.just_pressed(KeyCode::Digit8) {
        *method = Tonemapping::BlenderFilmic;
    }
}
