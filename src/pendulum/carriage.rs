use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::pendulum::collision_group;
use crate::pendulum::wheel;

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

    let carriage_config = BlockConfig {
        length: CARRIAGE_LENGTH,
        height: CARRIAGE_HEIGHT,
        initial_position: Transform::from_xyz(0.0, Y_ZERO, 0.0),
    };

    const PENDULUM_OFFSET: f32 = Y_ZERO + (PENDULUM_HEIGHT / 2.0) + (PENDULUM_LENGTH * 3.0);
    let pendulum_config = BlockConfig {
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

struct BlockConfig {
    length: Real,
    height: Real,
    initial_position: Transform,
}

fn add_carriage(
    carriage_config: BlockConfig,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    return add_block(carriage_config, commands, meshes, materials);
}

#[derive(Component)]
struct Pendulum;

fn add_pendulum(
    pendulum_config: BlockConfig,
    carriage: &mut Entity,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Add the little bit to join the pendulum and the carriage.
    let joiner_height = pendulum_config.length * 3.0;
    let pendulum_height = pendulum_config.height;
    let joiner_config = BlockConfig {
        height: joiner_height,
        length: pendulum_config.length,
        initial_position: Transform::from_xyz(
            0.0,
            pendulum_config.length + pendulum_config.initial_position.translation.y,
            0.0,
        ),
    };
    let joiner = add_block(joiner_config, commands, meshes, materials);

    let pendulum = add_block(pendulum_config, commands, meshes, materials);

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

fn add_block(
    block_config: BlockConfig,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let collision_group_filter = collision_group::GROUND;
    return commands
        .spawn((
            RigidBody::Dynamic,
            // Note factors of 2.0: Rapier uses half-length and height as the parameters.
            Collider::cuboid(block_config.length / 2.0, block_config.height / 2.0),
            ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            },
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Box::new(block_config.length, block_config.height, 0.0).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: block_config.initial_position,
                ..default()
            },
            CollisionGroups::new(collision_group::CARRIAGE, collision_group_filter),
        ))
        .id();
}
