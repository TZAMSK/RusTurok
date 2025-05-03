pub mod components;
pub mod systems;
mod tests;

use bevy::prelude::*;

use systems::spawn_bullets;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bullets);
    }
}
