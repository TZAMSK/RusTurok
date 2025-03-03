mod camera;
mod player;
mod settings;
mod world;

use bevy::prelude::*;

use camera::CameraPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
