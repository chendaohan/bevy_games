use bevy::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        // .insert_resource(GlobalVolume::new(2.))
        .add_systems(Startup, setup)
        .add_systems(Update, control_audio)
        .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("19_playing_sounds/Windless Slopes.ogg"),
        settings: PlaybackSettings::LOOP,
    });
}

fn control_audio(audio: Query<&AudioSink>, keyboard: Res<ButtonInput<KeyCode>>) {
    let Ok(sink) = audio.get_single() else {
        return;
    };
    if keyboard.just_pressed(KeyCode::ArrowUp) {
        sink.set_volume(sink.volume() + 0.2);
    }
    if keyboard.just_pressed(KeyCode::ArrowDown) {
        sink.set_volume(sink.volume() - 0.2);
    }
    if keyboard.just_pressed(KeyCode::KeyW) {
        sink.set_speed(sink.speed() + 0.2);
    }
    if keyboard.just_pressed(KeyCode::KeyS) {
        sink.set_speed(sink.speed() - 0.2);
    }
    if keyboard.just_pressed(KeyCode::KeyP) {
        sink.pause();
    }
    if keyboard.just_pressed(KeyCode::KeyL) {
        sink.play();
    }
    if keyboard.just_pressed(KeyCode::KeyO) {
        sink.stop();
    }
    if keyboard.just_pressed(KeyCode::KeyT) {
        sink.toggle();
    }
}