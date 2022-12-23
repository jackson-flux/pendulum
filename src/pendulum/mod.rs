use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub struct PendulumPlugin;

// Collision groups
const GROUND: Group = Group::GROUP_1;
const WHEEL: Group = Group::GROUP_2;
const CARRIAGE: Group = Group::GROUP_3;

impl Plugin for PendulumPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ground)
            .add_startup_system(setup_pendulum)
            .add_system(control_wheel);
    }
}

fn setup_ground(mut commands: Commands) {
    /* Create the ground. */
    let collision_group_filter = GROUND | WHEEL | CARRIAGE;
    commands.spawn((
        Collider::cuboid(500.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)),
        Friction::coefficient(100.0),
        CollisionGroups::new(GROUND, collision_group_filter),
    ));
}

#[derive(Component)]
struct Carriage;

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
    let collision_group_filter = GROUND;
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
            CollisionGroups::new(CARRIAGE, collision_group_filter),
        ))
        .id();
}

#[derive(Component)]
struct Wheel;

struct WheelConfig {
    radius: Real,
    initial_position: Transform,
    restitution: Real,
    friction: Real,
}

fn add_wheel(
    wheel_config: WheelConfig,
    carriage: &mut Entity,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let collision_group_filter = GROUND;
    let wheel = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::ball(wheel_config.radius),
            Restitution::coefficient(wheel_config.restitution),
            Friction::coefficient(wheel_config.friction),
            ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            },
            Velocity {
                linvel: Default::default(),
                angvel: 0.0,
            },
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(wheel_config.radius).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: wheel_config.initial_position,
                ..default()
            },
            CollisionGroups::new(WHEEL, collision_group_filter.into()),
            Wheel,
        ))
        .id();
    let axis = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(wheel_config.initial_position.translation.x, 0.0));

    commands.entity(wheel).with_children(|cmd| {
        cmd.spawn(ImpulseJoint::new(*carriage, axis));
    });
}

fn setup_pendulum(
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

    let left_wheel_config = WheelConfig {
        radius: WHEEL_RADIUS,
        initial_position: Transform::from_xyz(-WHEEL_BASE / 2.0, Y_ZERO, 0.0),
        restitution: 1.0,
        friction: 1.0,
    };
    let right_wheel_config = WheelConfig {
        initial_position: Transform::from_xyz(WHEEL_BASE / 2.0, Y_ZERO, 0.0),
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
    add_wheel(
        left_wheel_config,
        &mut carriage,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    add_wheel(
        right_wheel_config,
        &mut carriage,
        &mut commands,
        &mut meshes,
        &mut materials,
    );
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

fn control_wheel(
    keyboard_input: Res<Input<KeyCode>>,
    velocities: Query<&Velocity, With<Wheel>>,
    mut forces: Query<&mut ExternalForce, With<Wheel>>,
) {
    for (velocity, mut force) in velocities.iter().zip(forces.iter_mut()) {
        let torque = get_torque(velocity.angvel, &keyboard_input);
        force.torque = torque;
    }
}
