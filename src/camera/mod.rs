pub mod components;
pub mod renderlayers;
mod systems;

use bevy::prelude::*;

use systems::spawn_camera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}
