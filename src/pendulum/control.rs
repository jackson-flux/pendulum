use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::pendulum::carriage::wheel::Wheel;

pub fn control_wheel(
    keyboard_input: Res<Input<KeyCode>>,
    velocities: Query<&Velocity, With<Wheel>>,
    mut forces: Query<&mut ExternalForce, With<Wheel>>,
) {
    for (velocity, mut force) in velocities.iter().zip(forces.iter_mut()) {
        let torque = get_torque(velocity.angvel, &keyboard_input);
        force.torque = torque;
    }
}

fn get_torque(angular_velocity: f32, keyboard_input: &Res<Input<KeyCode>>) -> f32 {
    const MAX_TORQUE: f32 = 0.1;
    const MAX_ANGULAR_VELOCITY: f32 = 10.0;

    // Between -1.0 and 1.0.
    const TRUNC_MIN: f32 = -1.0;
    const TRUNC_MAX: f32 = 1.0;
    let velocity_proportion = (angular_velocity / MAX_ANGULAR_VELOCITY)
        .max(TRUNC_MIN)
        .min(TRUNC_MAX);
    let positive_residual_velocity = 1.0 - velocity_proportion.max(0.0);
    let negative_residual_velocity = 1.0 + velocity_proportion.min(0.0);

    let mut torque: f32 = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        // On left keypress, we want the torque to be positive, so wheel spins anticlockwise.
        torque = positive_residual_velocity * MAX_TORQUE;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        torque = -negative_residual_velocity * MAX_TORQUE;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        torque = -velocity_proportion * MAX_TORQUE;
    }
    return torque;
}
