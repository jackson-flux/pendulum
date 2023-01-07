use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::pendulum::collision_group;

#[derive(Component)]
pub struct Wheel;

pub struct Config {
    pub radius: Real,
    pub initial_position: Transform,
    pub restitution: Real,
    pub friction: Real,
}

pub fn add(
    config: Config,
    carriage: &mut Entity,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let collision_group_filter = collision_group::GROUND;
    let wheel = commands
        .spawn((
            RigidBody::Dynamic,
            Collider::ball(config.radius),
            Restitution::coefficient(config.restitution),
            Friction::coefficient(config.friction),
            ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            },
            Velocity {
                linvel: Default::default(),
                angvel: 0.0,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(config.radius).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: config.initial_position,
                ..default()
            },
            CollisionGroups::new(collision_group::WHEEL, collision_group_filter.into()),
            Wheel,
        ))
        .id();
    let axis = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(config.initial_position.translation.x, 0.0));

    commands.entity(wheel).with_children(|cmd| {
        cmd.spawn(ImpulseJoint::new(*carriage, axis));
    });
}
