use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_shop);
    }
}
