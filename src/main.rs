mod camera;
mod player;
mod settings;
mod world;

use bevy::prelude::*;

use camera::CameraPlugin;
use player::PlayerPlugin;
use settings::{exit, settings};
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(settings()))
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Update, exit)
        .run();
}
