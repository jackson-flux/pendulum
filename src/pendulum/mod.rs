use bevy::app::{App, Plugin};

pub struct PendulumPlugin;

mod carriage;
mod collision_group;
mod control;
mod ground;
mod wheel;

impl Plugin for PendulumPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ground::setup)
            .add_startup_system(carriage::setup)
            .add_system(control::control_wheel);
    }
}
