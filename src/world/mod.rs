mod components;
mod systems;

use bevy::prelude::*;

use systems::{spawn_lights, spawn_world_model};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lights)
            .add_systems(Startup, spawn_world_model);
    }
}
