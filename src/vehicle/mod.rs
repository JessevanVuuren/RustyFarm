use bevy::prelude::*;

use crate::vehicle::spawn::spawn_vehicle;

pub mod components;
pub mod spawn;
pub mod systems;

pub struct VehiclePlugin;

impl Plugin for VehiclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_vehicle);
        // app.add_systems(
        //     FixedUpdate,

        // );
    }
}
