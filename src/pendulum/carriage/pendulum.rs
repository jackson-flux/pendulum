use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::pendulum::carriage::block;

#[derive(Component)]
pub struct Pendulum;

pub fn add(
    pendulum_config: block::Config,
    carriage: &mut Entity,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Add the little bit to join the pendulum and the carriage.
    let joiner_height = pendulum_config.length * 2.0;
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
