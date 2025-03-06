mod camera;
mod crosshair;
mod player;
mod settings;
mod world;

use bevy::prelude::*;

use bevy_fps_counter::FpsCounterPlugin;
use camera::CameraPlugin;
use crosshair::CrosshairPlugin;
use player::PlayerPlugin;
use settings::{exit, fps, settings};
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(settings()))
        .add_plugins(FpsCounterPlugin)
        .add_systems(Update, fps)
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(CrosshairPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Update, exit)
        .run();
}
