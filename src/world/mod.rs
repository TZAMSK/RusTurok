mod components;
mod systems;

use bevy::prelude::*;

use systems::spawn_lights;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lights);
    }
}
