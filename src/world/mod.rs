mod components;
mod systems;

use bevy::prelude::*;

use systems::spawn_world_model;

use crate::world::systems::{spawn_fog, spawn_sun_light};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world_model)
            .add_systems(Startup, spawn_sun_light)
            .add_systems(
                Startup,
                (spawn_fog).after(crate::camera::systems::spawn_camera),
            );
    }
}
