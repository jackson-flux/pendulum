use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod block;
pub mod wheel;

#[derive(Component)]
struct Carriage;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const WHEEL_BASE: f32 = 100.0;
    const WHEEL_RADIUS: f32 = 10.0;
    const CARRIAGE_LENGTH: f32 = WHEEL_BASE + 10.0;
    const CARRIAGE_HEIGHT: f32 = 10.0;
    const PENDULUM_LENGTH: f32 = CARRIAGE_HEIGHT;
    const PENDULUM_HEIGHT: f32 = CARRIAGE_LENGTH;
    const Y_ZERO: f32 = 50.0;

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
    let mut carriage = add_carriage(carriage_config, &mut commands, &mut meshes, &mut materials);
    add_pendulum(
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

fn add_carriage(
    carriage_config: block::Config,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    return block::add(carriage_config, commands, meshes, materials);
}

#[derive(Component)]
struct Pendulum;

fn add_pendulum(
    pendulum_config: block::Config,
    carriage: &mut Entity,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Add the little bit to join the pendulum and the carriage.
    let joiner_height = pendulum_config.length * 3.0;
    let pendulum_height = pendulum_config.height;
    let joiner_config = block::Config {
        height: joiner_height,
        length: pendulum_config.length,
        initial_position: Transform::from_xyz(
            0.0,
            pendulum_config.length + pendulum_config.initial_position.translation.y,
            0.0,
        ),
    };
    let joiner = block::add(joiner_config, commands, meshes, materials);

    let pendulum = block::add(pendulum_config, commands, meshes, materials);

    let joiner_pin = FixedJointBuilder::new().local_anchor1(Vec2::new(0.0, -joiner_height / 2.0));
    commands.entity(*carriage).with_children(|cmd| {
        cmd.spawn(ImpulseJoint::new(joiner, joiner_pin));
    });

    let pendulum_pivot = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, joiner_height / 2.0))
        .local_anchor2(Vec2::new(0.0, -pendulum_height / 2.0));
    commands.entity(pendulum).with_children(|cmd| {
        cmd.spawn(ImpulseJoint::new(joiner, pendulum_pivot));
    });
}
