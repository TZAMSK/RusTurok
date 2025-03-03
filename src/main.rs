mod camera;
mod player;
mod settings;
mod world;

use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
