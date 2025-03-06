pub mod components;
mod systems;

use bevy::prelude::*;

use systems::spawn_crosshair;

pub struct CrosshairPlugin;

impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_crosshair);
    }
}
