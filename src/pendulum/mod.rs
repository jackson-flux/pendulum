use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub struct PendulumPlugin;

impl Plugin for PendulumPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ground)
            .add_startup_system(setup_pendulum)
            .add_system(control_wheel);
    }
}

fn setup_ground(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(Friction::coefficient(1.0));
}

#[derive(Component)]
struct Wheel;

fn setup_pendulum(mut commands: Commands,
                  mut meshes: ResMut<Assets<Mesh>>,
                  mut materials: ResMut<Assets<ColorMaterial>>,
) {
    add_wheel(100.0, &mut commands, &mut meshes, &mut materials);
    add_wheel(-100.0, &mut commands, &mut meshes, &mut materials);
}

fn add_wheel(offset: f32, commands: &mut Commands,
             meshes: &mut ResMut<Assets<Mesh>>,
             materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    const WHEEL_RADIUS: Real = 50.0;
    const RESTITUTION: Real = 0.7;
    const FRICTION: Real = 1.0;
    const INITIAL_HEIGHT: Real = 50.0;
    let transform = Transform::from_xyz(offset, INITIAL_HEIGHT, 0.0);
    commands
        .spawn((
            RigidBody::Dynamic, Collider::ball(WHEEL_RADIUS),
            Restitution::coefficient(RESTITUTION),
            Friction::coefficient(FRICTION),
            // TransformBundle::from(transform),
            ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(WHEEL_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform,
                ..default()
            },
            Wheel
        ));
}

fn get_torque(keyboard_input: Res<Input<KeyCode>>) -> f32 {
    const TORQUE: f32 = 10.0;
    let mut torque: f32 = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        torque = TORQUE;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        torque = -TORQUE;
    }
    return torque;
}

fn control_wheel(
    keyboard_input: Res<Input<KeyCode>>,
    mut forces: Query<&mut ExternalForce, With<Wheel>>,
) {
    let torque = get_torque(keyboard_input);
    for mut force in forces.iter_mut() {
        force.torque = torque;
    }
}
