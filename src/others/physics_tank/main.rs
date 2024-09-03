mod blender_editor;
mod spawn_tank;
mod control_tank;

use bevy::{asset::AssetPath, dev_tools::fps_overlay::FpsOverlayPlugin, prelude::*};
use bevy_blendy_cameras::BlendyCamerasPlugin;
use bevy_rapier3d::plugin::*;
use blender_editor::BlenderEditorPlugin;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            BlendyCamerasPlugin,
            FpsOverlayPlugin::default(),
            BlenderEditorPlugin::new(
                vec![AssetPath::from("physics_tank/physics_tank.glb#Scene0")],
                GameState::Start,
            ),
            spawn_tank::plugin,
            control_tank::plugin,
        ))
        .init_state::<GameState>()
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Fixed { dt: 1. / 60., substeps: 10 },
            ..RapierConfiguration::new(1.)
        })
        .run()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, States, Default)]
enum GameState {
    #[default]
    Loading,
    Start,
}
