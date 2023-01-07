use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::pendulum::collision_group;

pub fn setup(mut commands: Commands) {
    /* Create the ground. */
    let collision_group_filter =
        collision_group::GROUND | collision_group::WHEEL | collision_group::CARRIAGE;
    commands.spawn((
        Collider::cuboid(500.0, 50.0),
        TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)),
        Friction::coefficient(100.0),
        CollisionGroups::new(collision_group::GROUND, collision_group_filter),
    ));
}
