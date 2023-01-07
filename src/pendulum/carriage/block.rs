use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::pendulum::collision_group;

pub struct Config {
    pub length: Real,
    pub height: Real,
    pub initial_position: Transform,
}

pub fn add(
    block_config: Config,
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
