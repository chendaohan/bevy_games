use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::spawn_tank::{LeftBackDriveWheel, LeftBackWheel, LeftFrontDriveWheel, LeftFrontWheel, RightBackDriveWheel, RightBackWheel, RightFrontDriveWheel, RightFrontWheel};

pub fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, movement);
}

fn movement(
    mut left_wheels: Query<(&mut Velocity, &Transform), (Or<(With<LeftFrontDriveWheel>, With<LeftBackDriveWheel>, With<LeftFrontWheel>, With<LeftBackWheel>)>, Without<RightFrontDriveWheel>, Without<RightBackDriveWheel>, Without<RightFrontWheel>, Without<RightBackWheel>)>,
    mut right_wheels: Query<(&mut Velocity, &Transform), Or<(With<RightFrontDriveWheel>, With<RightBackDriveWheel>, With<RightFrontWheel>, With<RightBackWheel>)>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    const SPEED: f32 = 400.;
    let speed_increment = SPEED * time.delta_seconds();
    let mut left_velocity= Vec3::ZERO;
    let mut right_velocity= Vec3::ZERO;

    if keyboard.pressed(KeyCode::ArrowUp) {
        left_velocity.x += speed_increment;
        right_velocity.x += speed_increment;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        left_velocity.x -= speed_increment;
        right_velocity.x -= speed_increment;
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        left_velocity.x -= speed_increment;
        right_velocity.x += speed_increment;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        left_velocity.x += speed_increment;
        right_velocity.x -= speed_increment;
    }

    for (mut velocity, transform) in &mut left_wheels {
        velocity.angvel += transform.rotation * left_velocity;
    }
    for (mut velocity, transform) in &mut right_wheels {
        velocity.angvel += transform.rotation * right_velocity;
    }
}
