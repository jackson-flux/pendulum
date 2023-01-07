use bevy::prelude::*;

mod block;
mod pendulum;
pub mod wheel;

#[derive(Component)]
struct Carriage;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //---- Configuration constants. ----//
    const WHEEL_BASE: f32 = 100.0;
    const WHEEL_RADIUS: f32 = 10.0;
    const CARRIAGE_LENGTH: f32 = WHEEL_BASE + 10.0;
    const CARRIAGE_HEIGHT: f32 = 10.0;
    const PENDULUM_LENGTH: f32 = CARRIAGE_HEIGHT;
    const PENDULUM_HEIGHT: f32 = CARRIAGE_LENGTH;
    const Y_ZERO: f32 = 50.0;

    //---- Create config structs from constants. ----//
    let left_wheel_config = wheel::Config {
        radius: WHEEL_RADIUS,
        initial_position: Transform::from_xyz(-WHEEL_BASE / 2.0, Y_ZERO, 1.0),
        restitution: 1.0,
        friction: 1.0,
    };
    let right_wheel_config = wheel::Config {
        initial_position: Transform::from_xyz(WHEEL_BASE / 2.0, Y_ZERO, 1.0),
        ..left_wheel_config
    };

    let carriage_config = block::Config {
        length: CARRIAGE_LENGTH,
        height: CARRIAGE_HEIGHT,
        initial_position: Transform::from_xyz(0.0, Y_ZERO, 0.0),
    };

    const PENDULUM_OFFSET: f32 = Y_ZERO + (PENDULUM_HEIGHT / 2.0) + (PENDULUM_LENGTH * 3.0);
    let pendulum_config = block::Config {
        length: PENDULUM_LENGTH,
        height: PENDULUM_HEIGHT,
        initial_position: Transform::from_xyz(0.0, PENDULUM_OFFSET, 0.0),
    };

    //---- Construct the carriage. ----//
    // Add the base of the carriage:
    let mut carriage = block::add(carriage_config, &mut commands, &mut meshes, &mut materials);

    pendulum::add(
        pendulum_config,
        &mut carriage,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    wheel::add(
        left_wheel_config,
        &mut carriage,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    wheel::add(
        right_wheel_config,
        &mut carriage,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}
